//! Index type enum

use serde::{Deserialize, Serialize};

/// Database index types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndexType {
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "unique")]
    Unique,
}

impl AsRef<str> for IndexType {
    fn as_ref(&self) -> &str {
        match self {
            IndexType::Key => "key",
            IndexType::Fulltext => "fulltext",
            IndexType::Unique => "unique",
        }
    }
}