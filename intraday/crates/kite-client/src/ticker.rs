//! Kite v3 WebSocket ticker connection: subscribe, decode (see
//! `ticker_decode`), and reconnect with backoff on any failure — treating
//! "no ticks for a while" as the caller's problem to notice (via normal
//! stream/timeout handling), not something this module silently hides.

use crate::ticker_decode::decode_packets;
use core_types::{Instrument, Tick};
use futures::stream::BoxStream;
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_rustls::TlsConnector;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::tungstenite::Message;

const WS_HOST: &str = "ws.kite.trade";
const WS_PORT: u16 = 443;
const INITIAL_BACKOFF_SECS: u64 = 1;
const MAX_BACKOFF_SECS: u64 = 30;

/// Kite's WS endpoint returns a bare 400 if the TLS layer negotiates
/// HTTP/2 via ALPN (confirmed against the real endpoint — curl's default
/// ALPN offer, which includes h2, gets the same 400; forcing HTTP/1.1 gets
/// a normal 101 Switching Protocols). `tokio-tungstenite`'s own
/// `connect_async_tls_with_config` did not actually apply a custom ALPN
/// restriction in practice, so this drives TCP+TLS by hand (pinning ALPN
/// to http/1.1 directly) and only hands the resulting stream to
/// `tokio_tungstenite::client_async` for the WS handshake/framing.
fn tls_connector() -> TlsConnector {
    static CONNECTOR: OnceLock<TlsConnector> = OnceLock::new();
    CONNECTOR
        .get_or_init(|| {
            let mut root_store = rustls::RootCertStore::empty();
            root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            let mut config = rustls::ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth();
            config.alpn_protocols = vec![b"http/1.1".to_vec()];
            TlsConnector::from(Arc::new(config))
        })
        .clone()
}

/// Spawns a background task that connects, subscribes to `instruments` in
/// "full" mode, and forwards decoded ticks — reconnecting with exponential
/// backoff (capped at 30s, reset after a connection survives a while) on
/// any error or unexpected close. The returned stream ends only when the
/// caller drops it (which also stops the background task).
pub fn subscribe_ticks(api_key: String, access_token: String, instruments: &[Instrument]) -> BoxStream<'static, Tick> {
    let tokens: Vec<i64> = instruments.iter().map(|i| i.instrument_token as i64).collect();
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(run_with_reconnect(api_key, access_token, tokens, tx));
    Box::pin(UnboundedReceiverStream::new(rx))
}

async fn run_with_reconnect(
    api_key: String,
    access_token: String,
    tokens: Vec<i64>,
    tx: mpsc::UnboundedSender<Tick>,
) {
    let mut backoff = INITIAL_BACKOFF_SECS;
    loop {
        if tx.is_closed() {
            return; // caller dropped the stream — nothing left to do
        }
        let connected_at = tokio::time::Instant::now();
        match run_once(&api_key, &access_token, &tokens, &tx).await {
            Ok(()) => {
                if tx.is_closed() {
                    return;
                }
                tracing::warn!("Kite ticker connection closed, reconnecting");
            }
            Err(e) => {
                tracing::warn!(error = %e, "Kite ticker connection failed, reconnecting");
            }
        }
        // A connection that stayed up a while is treated as healthy —
        // don't let one long session's eventual drop inherit a maxed-out
        // backoff from a much earlier flapping period.
        if connected_at.elapsed() > Duration::from_secs(MAX_BACKOFF_SECS) {
            backoff = INITIAL_BACKOFF_SECS;
        }
        tokio::time::sleep(Duration::from_secs(backoff)).await;
        backoff = (backoff * 2).min(MAX_BACKOFF_SECS);
    }
}

async fn run_once(
    api_key: &str,
    access_token: &str,
    tokens: &[i64],
    tx: &mpsc::UnboundedSender<Tick>,
) -> anyhow::Result<()> {
    let url = format!("wss://{WS_HOST}/?api_key={api_key}&access_token={access_token}");

    let tcp = TcpStream::connect((WS_HOST, WS_PORT)).await?;
    let server_name = rustls::pki_types::ServerName::try_from(WS_HOST)?.to_owned();
    let tls_stream = tls_connector().connect(server_name, tcp).await?;

    let (ws_stream, _response) = tokio_tungstenite::client_async(&url, tls_stream).await?;
    let (mut write, mut read) = ws_stream.split();

    write
        .send(Message::text(serde_json::json!({"a": "subscribe", "v": tokens}).to_string()))
        .await?;
    write
        .send(Message::text(
            serde_json::json!({"a": "mode", "v": ["full", tokens]}).to_string(),
        ))
        .await?;

    while let Some(msg) = read.next().await {
        match msg? {
            Message::Binary(data) => {
                for tick in decode_packets(&data) {
                    if tx.send(tick).is_err() {
                        return Ok(()); // receiver dropped — stop cleanly, not an error
                    }
                }
            }
            Message::Text(text) => {
                tracing::debug!(%text, "Kite ticker text message (postback/error)");
            }
            Message::Close(frame) => {
                tracing::info!(?frame, "Kite ticker closed the connection");
                break;
            }
            Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => {}
        }
    }
    Ok(())
}
