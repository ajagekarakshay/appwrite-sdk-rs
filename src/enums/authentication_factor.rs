//! Authentication factor enum

use serde::{Deserialize, Serialize};

/// Authentication factors for multi-factor authentication
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthenticationFactor {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "recoverycode")]
    RecoveryCode,
}

impl AsRef<str> for AuthenticationFactor {
    fn as_ref(&self) -> &str {
        match self {
            AuthenticationFactor::Email => "email",
            AuthenticationFactor::Phone => "phone",
            AuthenticationFactor::Totp => "totp",
            AuthenticationFactor::RecoveryCode => "recoverycode",
        }
    }
}