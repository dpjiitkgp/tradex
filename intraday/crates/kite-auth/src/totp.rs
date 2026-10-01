use secrecy::{ExposeSecret, SecretString};
use totp_rs::{Algorithm, Secret, TOTP};

/// Current 6-digit TOTP code from the base32 seed Kite gives you when
/// setting up 2FA (not the 6-digit code itself — the seed).
pub fn current_code(secret: &SecretString) -> anyhow::Result<String> {
    let bytes = Secret::Encoded(secret.expose_secret().to_string())
        .to_bytes()
        .map_err(|e| anyhow::anyhow!("invalid TOTP secret (expected base32): {e:?}"))?;
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, bytes)
        .map_err(|e| anyhow::anyhow!("building TOTP generator: {e:?}"))?;
    Ok(totp.generate_current()?)
}
