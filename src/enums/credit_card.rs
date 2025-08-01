//! Credit card enum for avatars

use serde::{Deserialize, Serialize};

/// Credit card types for avatar generation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreditCard {
    #[serde(rename = "amex")]
    Amex,
    #[serde(rename = "argencard")]
    Argencard,
    #[serde(rename = "cabal")]
    Cabal,
    #[serde(rename = "censosud")]
    Censosud,
    #[serde(rename = "diners")]
    Diners,
    #[serde(rename = "discover")]
    Discover,
    #[serde(rename = "elo")]
    Elo,
    #[serde(rename = "hipercard")]
    Hipercard,
    #[serde(rename = "jcb")]
    Jcb,
    #[serde(rename = "mastercard")]
    Mastercard,
    #[serde(rename = "naranja")]
    Naranja,
    #[serde(rename = "targeta-shopping")]
    TargetaShopping,
    #[serde(rename = "union-china-pay")]
    UnionChinaPay,
    #[serde(rename = "visa")]
    Visa,
    #[serde(rename = "mir")]
    Mir,
    #[serde(rename = "maestro")]
    Maestro,
}

impl AsRef<str> for CreditCard {
    fn as_ref(&self) -> &str {
        match self {
            CreditCard::Amex => "amex",
            CreditCard::Argencard => "argencard",
            CreditCard::Cabal => "cabal",
            CreditCard::Censosud => "censosud",
            CreditCard::Diners => "diners",
            CreditCard::Discover => "discover",
            CreditCard::Elo => "elo",
            CreditCard::Hipercard => "hipercard",
            CreditCard::Jcb => "jcb",
            CreditCard::Mastercard => "mastercard",
            CreditCard::Naranja => "naranja",
            CreditCard::TargetaShopping => "targeta-shopping",
            CreditCard::UnionChinaPay => "union-china-pay",
            CreditCard::Visa => "visa",
            CreditCard::Mir => "mir",
            CreditCard::Maestro => "maestro",
        }
    }
}