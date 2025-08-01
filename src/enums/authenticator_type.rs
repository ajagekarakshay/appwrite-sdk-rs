//! Authenticator type enum

use serde::{Deserialize, Serialize};

/// Authenticator types for multi-factor authentication
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthenticatorType {
    #[serde(rename = "totp")]
    Totp,
}

impl AsRef<str> for AuthenticatorType {
    fn as_ref(&self) -> &str {
        match self {
            AuthenticatorType::Totp => "totp",
        }
    }
}