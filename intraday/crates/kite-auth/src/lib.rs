//! Headless Kite Connect login (id/password/TOTP) that mints a fresh daily access token with no manual step. The most fragile/sensitive module - kept standalone and independently testable.

pub mod credentials;
pub mod login;
pub mod provider;
pub mod totp;

pub use credentials::KiteCredentials;
pub use provider::KiteLoginProvider;
