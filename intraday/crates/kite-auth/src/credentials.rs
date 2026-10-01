use secrecy::SecretString;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct RawFile {
    kite: RawKite,
}

#[derive(Debug, Deserialize)]
struct RawKite {
    api_key: String,
    api_secret: String,
    user_id: String,
    password: String,
    totp_secret: String,
}

/// Loaded from `secrets/credentials.toml` (gitignored, chmod 600) — see
/// `secrets/credentials.example.toml` for the template. Password/secrets are
/// wrapped in `SecretString` so they never show up in `{:?}` output or logs.
#[derive(Debug, Clone)]
pub struct KiteCredentials {
    pub api_key: String,
    pub api_secret: SecretString,
    pub user_id: String,
    pub password: SecretString,
    pub totp_secret: SecretString,
}

impl KiteCredentials {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|e| {
            anyhow::anyhow!(
                "reading {path:?}: {e} (copy secrets/credentials.example.toml to \
                 secrets/credentials.toml and fill in real Kite Connect values)"
            )
        })?;
        let raw: RawFile = toml::from_str(&text)?;
        if raw.kite.api_key.is_empty() || raw.kite.user_id.is_empty() {
            anyhow::bail!("{path:?} has an empty api_key/user_id — fill in real credentials");
        }
        Ok(Self {
            api_key: raw.kite.api_key,
            api_secret: raw.kite.api_secret.into(),
            user_id: raw.kite.user_id,
            password: raw.kite.password.into(),
            totp_secret: raw.kite.totp_secret.into(),
        })
    }
}
