//! Relationship type enum

use serde::{Deserialize, Serialize};

/// Database relationship types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    #[serde(rename = "oneToOne")]
    OneToOne,
    #[serde(rename = "oneToMany")]
    OneToMany,
    #[serde(rename = "manyToOne")]
    ManyToOne,
    #[serde(rename = "manyToMany")]
    ManyToMany,
}

impl AsRef<str> for RelationshipType {
    fn as_ref(&self) -> &str {
        match self {
            RelationshipType::OneToOne => "oneToOne",
            RelationshipType::OneToMany => "oneToMany",
            RelationshipType::ManyToOne => "manyToOne",
            RelationshipType::ManyToMany => "manyToMany",
        }
    }
}