//! Browser enum for avatars

use serde::{Deserialize, Serialize};

/// Browser types for avatar generation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Browser {
    #[serde(rename = "aa")]
    Aa,
    #[serde(rename = "an")]
    An,
    #[serde(rename = "ch")]
    Ch,
    #[serde(rename = "cm")]
    Cm,
    #[serde(rename = "cr")]
    Cr,
    #[serde(rename = "ff")]
    Ff,
    #[serde(rename = "sf")]
    Sf,
    #[serde(rename = "ie")]
    Ie,
}

impl AsRef<str> for Browser {
    fn as_ref(&self) -> &str {
        match self {
            Browser::Aa => "aa",
            Browser::An => "an",
            Browser::Ch => "ch",
            Browser::Cm => "cm",
            Browser::Cr => "cr",
            Browser::Ff => "ff",
            Browser::Sf => "sf",
            Browser::Ie => "ie",
        }
    }
}