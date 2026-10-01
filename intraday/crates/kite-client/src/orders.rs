//! Real order placement/modification/cancellation/status and the margin
//! calculator, plus `KiteBroker`, the `core_types::Broker` implementation
//! over these. Every function here can move real money — see
//! `crate::rest::KiteRestClient::post_form`'s doc comment for why the raw
//! status+body is threaded through rather than bailing early.

use crate::rest::KiteRestClient;
use async_trait::async_trait;
use core_types::{
    Broker, BrokerPosition, Error, Exchange, MarginCheckResult, OrderRequest, OrderResult,
    OrderSide, OrderType, Product, Result,
};
use serde_json::Value;

const VARIETY: &str = "regular";

fn side_str(side: OrderSide) -> &'static str {
    match side {
        OrderSide::Buy => "BUY",
        OrderSide::Sell => "SELL",
    }
}

fn order_type_str(t: OrderType) -> &'static str {
    match t {
        OrderType::Market => "MARKET",
        OrderType::Limit => "LIMIT",
        OrderType::SlLimit => "SL",
        OrderType::SlMarket => "SL-M",
    }
}

fn product_str(p: Product) -> &'static str {
    match p {
        Product::Mis => "MIS",
    }
}

fn exchange_str(e: Exchange) -> &'static str {
    match e {
        Exchange::Nse => "NSE",
        Exchange::Nfo => "NFO",
    }
}

fn order_form(req: &OrderRequest, extra_qty: Option<String>) -> Vec<(&str, String)> {
    let mut form = vec![
        ("exchange", exchange_str(req.exchange).to_string()),
        ("tradingsymbol", req.tradingsymbol.clone()),
        ("transaction_type", side_str(req.side).to_string()),
        ("quantity", extra_qty.unwrap_or_else(|| req.quantity.to_string())),
        ("product", product_str(req.product).to_string()),
        ("order_type", order_type_str(req.order_type).to_string()),
        ("validity", "DAY".to_string()),
        ("tag", req.tag.clone()),
    ];
    if let Some(price) = req.price {
        form.push(("price", format!("{price:.2}")));
    }
    if let Some(trigger) = req.trigger_price {
        form.push(("trigger_price", format!("{trigger:.2}")));
    }
    form
}

/// Kite's envelope is always `{"status": "success"|"error", "data": ...,
/// "message": ..., "error_type": ...}` — parses that generically so every
/// call site gets consistent (ok/rejection_reason/raw_json) handling
/// instead of each hand-rolling its own.
struct Envelope {
    ok: bool,
    data: Value,
    message: Option<String>,
}

fn parse_envelope(body: &str) -> Envelope {
    let parsed: Value = serde_json::from_str(body).unwrap_or(Value::Null);
    let ok = parsed.get("status").and_then(|s| s.as_str()) == Some("success");
    let message = parsed
        .get("message")
        .and_then(|m| m.as_str())
        .map(|s| s.to_string());
    let data = parsed.get("data").cloned().unwrap_or(Value::Null);
    Envelope { ok, data, message }
}

fn parse_order_status(status: &str) -> core_types::OrderStatus {
    use core_types::OrderStatus::*;
    match status {
        "COMPLETE" => Complete,
        "CANCELLED" => Cancelled,
        "REJECTED" => Rejected,
        "OPEN" | "TRIGGER PENDING" | "MODIFY PENDING" | "CANCEL PENDING" => Open,
        "PUT ORDER REQ RECEIVED" | "VALIDATION PENDING" | "OPEN PENDING" => Pending,
        other => Other(other.to_string()),
    }
}

/// Fetches the full order history for one order_id and returns its *latest*
/// entry — Kite's `/orders/:order_id` returns every state transition, not
/// just the current one.
async fn fetch_latest_order_state(rest: &KiteRestClient, broker_order_id: &str) -> Result<OrderResult> {
    let body = rest
        .get_text(&format!("/orders/{broker_order_id}"))
        .await
        .map_err(|e| Error::Broker(format!("order_status({broker_order_id}): {e}")))?;
    let env = parse_envelope(&body);
    let history = env.data.as_array().cloned().unwrap_or_default();
    let Some(latest) = history.last() else {
        return Err(Error::Broker(format!(
            "order_status({broker_order_id}): empty history — {body}"
        )));
    };
    let status_str = latest.get("status").and_then(|s| s.as_str()).unwrap_or("");
    Ok(OrderResult {
        broker_order_id: broker_order_id.to_string(),
        status: parse_order_status(status_str),
        filled_quantity: latest
            .get("filled_quantity")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        average_price: latest.get("average_price").and_then(|v| v.as_f64()),
        rejection_reason: latest
            .get("status_message")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()),
        raw_response_json: body,
    })
}

pub struct KiteBroker {
    rest: KiteRestClient,
}

impl KiteBroker {
    pub fn new(rest: KiteRestClient) -> Self {
        Self { rest }
    }
}

#[async_trait]
impl Broker for KiteBroker {
    async fn place_order(&self, req: &OrderRequest) -> Result<OrderResult> {
        let form = order_form(req, None);
        let form_refs: Vec<(&str, &str)> = form.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let (status, body) = self
            .rest
            .post_form(&format!("/orders/{VARIETY}"), &form_refs)
            .await
            .map_err(|e| Error::Broker(format!("place_order transport failure: {e}")))?;
        let env = parse_envelope(&body);
        if !env.ok {
            // A rejection is a normal, expected outcome here (RMS block,
            // insufficient margin, bad price) — not a transport failure —
            // so it comes back as an Ok(OrderResult) with status=Rejected,
            // never a bailed Err, so the caller can log it and move on
            // through the same path as any other order outcome.
            return Ok(OrderResult {
                broker_order_id: String::new(),
                status: core_types::OrderStatus::Rejected,
                filled_quantity: 0,
                average_price: None,
                rejection_reason: env.message.or(Some(format!("HTTP {status}"))),
                raw_response_json: body,
            });
        }
        let order_id = env
            .data
            .get("order_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Broker(format!("place_order: no order_id in response: {body}")))?
            .to_string();
        // Kite's place-order response only confirms acceptance, not fill —
        // fetch the actual current state immediately rather than assume.
        fetch_latest_order_state(&self.rest, &order_id).await
    }

    async fn modify_order(
        &self,
        broker_order_id: &str,
        new_price: Option<f64>,
        new_trigger_price: Option<f64>,
        new_quantity: Option<u32>,
    ) -> Result<OrderResult> {
        let mut form: Vec<(&str, String)> = Vec::new();
        if let Some(p) = new_price {
            form.push(("price", format!("{p:.2}")));
        }
        if let Some(t) = new_trigger_price {
            form.push(("trigger_price", format!("{t:.2}")));
        }
        if let Some(q) = new_quantity {
            form.push(("quantity", q.to_string()));
        }
        let form_refs: Vec<(&str, &str)> = form.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let (status, body) = self
            .rest
            .put_form(&format!("/orders/{VARIETY}/{broker_order_id}"), &form_refs)
            .await
            .map_err(|e| Error::Broker(format!("modify_order({broker_order_id}) transport failure: {e}")))?;
        let env = parse_envelope(&body);
        if !env.ok {
            return Err(Error::Broker(format!(
                "modify_order({broker_order_id}) failed (HTTP {status}): {}",
                env.message.unwrap_or(body)
            )));
        }
        fetch_latest_order_state(&self.rest, broker_order_id).await
    }

    async fn cancel_order(&self, broker_order_id: &str) -> Result<()> {
        let (status, body) = self
            .rest
            .delete(&format!("/orders/{VARIETY}/{broker_order_id}"))
            .await
            .map_err(|e| Error::Broker(format!("cancel_order({broker_order_id}) transport failure: {e}")))?;
        let env = parse_envelope(&body);
        if !env.ok {
            // A cancel racing a fill (order completed a moment before the
            // cancel arrived) is a normal outcome the caller must handle by
            // re-checking status, not a fatal error — but any other
            // rejection reason is surfaced.
            let msg = env.message.unwrap_or_default();
            if msg.to_lowercase().contains("not cancellable") || msg.to_lowercase().contains("already") {
                return Ok(());
            }
            return Err(Error::Broker(format!(
                "cancel_order({broker_order_id}) failed (HTTP {status}): {msg}"
            )));
        }
        Ok(())
    }

    async fn order_status(&self, broker_order_id: &str) -> Result<OrderResult> {
        fetch_latest_order_state(&self.rest, broker_order_id).await
    }

    async fn check_margin(&self, req: &OrderRequest) -> Result<MarginCheckResult> {
        // Unlike order placement, the margin calculator takes a JSON
        // *array* body (it can batch-check several prospective orders at
        // once) — form-encoding here returns a bare "invalid json" error.
        let mut order = serde_json::json!({
            "exchange": exchange_str(req.exchange),
            "tradingsymbol": req.tradingsymbol,
            "transaction_type": side_str(req.side),
            "variety": VARIETY,
            "product": product_str(req.product),
            "order_type": order_type_str(req.order_type),
            "quantity": req.quantity,
        });
        if let Some(price) = req.price {
            order["price"] = serde_json::json!(price);
        }
        if let Some(trigger) = req.trigger_price {
            order["trigger_price"] = serde_json::json!(trigger);
        }
        let (status, body) = self
            .rest
            .post_json_body("/margins/orders", &Value::Array(vec![order]))
            .await
            .map_err(|e| Error::Broker(format!("check_margin transport failure: {e}")))?;
        let env = parse_envelope(&body);
        if !env.ok {
            return Ok(MarginCheckResult {
                ok: false,
                required_margin: None,
                available_margin: None,
                rejection_reason: env.message.or(Some(format!("HTTP {status}"))),
                raw_response_json: body,
            });
        }
        // /margins/orders returns a JSON array, one element per order in
        // the (possibly batched) request — we always send exactly one.
        let entry = env.data.as_array().and_then(|a| a.first()).cloned().unwrap_or(Value::Null);
        let required = entry.get("total").and_then(|v| v.as_f64());
        Ok(MarginCheckResult {
            ok: true,
            required_margin: required,
            available_margin: None,
            rejection_reason: None,
            raw_response_json: body,
        })
    }

    async fn positions(&self) -> Result<Vec<BrokerPosition>> {
        let body = self
            .rest
            .get_text("/portfolio/positions")
            .await
            .map_err(|e| Error::Broker(format!("positions: {e}")))?;
        let env = parse_envelope(&body);
        if !env.ok {
            return Err(Error::Broker(format!(
                "positions failed: {}",
                env.message.unwrap_or(body)
            )));
        }
        // "day" positions (today's intraday activity) is what matters for
        // an MIS-only strategy — "net" carries forward positions we never
        // hold overnight.
        let day = env.data.get("day").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        Ok(day
            .into_iter()
            .filter_map(|p| {
                Some(BrokerPosition {
                    tradingsymbol: p.get("tradingsymbol")?.as_str()?.to_string(),
                    net_quantity: p.get("quantity")?.as_i64()?,
                    average_price: p.get("average_price").and_then(|v| v.as_f64()).unwrap_or(0.0),
                })
            })
            .collect())
    }
}

/// Read-only account/margin check — `GET /user/margins` — used by the
/// pre-flight `live-trade check` command to confirm this API key/token can
/// see trading-account data at all, before anything that could place an
/// order runs. Kept separate from the `Broker` trait since it's not part
/// of the order-execution contract, just a one-time sanity probe.
pub async fn fetch_equity_margins(rest: &KiteRestClient) -> anyhow::Result<Value> {
    let body = rest.get_text("/user/margins/equity").await?;
    let env = parse_envelope(&body);
    if !env.ok {
        anyhow::bail!(
            "GET /user/margins/equity failed: {}",
            env.message.unwrap_or(body)
        );
    }
    Ok(env.data)
}
