//! Query builder for database operations

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Query builder for Appwrite database operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Query method
    pub method: String,
    /// Attribute name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// Query values (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Value>>,
}

impl Query {
    /// Create a new query
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            attribute: None,
            values: None,
        }
    }

    /// Create a new query with attribute
    pub fn with_attribute(method: impl Into<String>, attribute: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            attribute: Some(attribute.into()),
            values: None,
        }
    }

    /// Create a new query with attribute and values
    pub fn with_values<T: Into<Value>>(
        method: impl Into<String>,
        attribute: impl Into<String>,
        values: Vec<T>,
    ) -> Self {
        Self {
            method: method.into(),
            attribute: Some(attribute.into()),
            values: Some(values.into_iter().map(|v| v.into()).collect()),
        }
    }

    /// Create a new query with a single value
    pub fn with_value<T: Into<Value>>(
        method: impl Into<String>,
        attribute: impl Into<String>,
        value: T,
    ) -> Self {
        Self {
            method: method.into(),
            attribute: Some(attribute.into()),
            values: Some(vec![value.into()]),
        }
    }

    /// Equal query
    pub fn equal<T: Into<Value>>(attribute: impl Into<String>, value: T) -> String {
        serde_json::to_string(&Self::with_value("equal", attribute, value))
            .unwrap_or_default()
    }

    /// Not equal query
    pub fn not_equal<T: Into<Value>>(attribute: impl Into<String>, value: T) -> String {
        serde_json::to_string(&Self::with_value("notEqual", attribute, value))
            .unwrap_or_default()
    }

    /// Less than query
    pub fn less_than<T: Into<Value>>(attribute: impl Into<String>, value: T) -> String {
        serde_json::to_string(&Self::with_value("lessThan", attribute, value))
            .unwrap_or_default()
    }

    /// Less than or equal query
    pub fn less_than_equal<T: Into<Value>>(attribute: impl Into<String>, value: T) -> String {
        serde_json::to_string(&Self::with_value("lessThanEqual", attribute, value))
            .unwrap_or_default()
    }

    /// Greater than query
    pub fn greater_than<T: Into<Value>>(attribute: impl Into<String>, value: T) -> String {
        serde_json::to_string(&Self::with_value("greaterThan", attribute, value))
            .unwrap_or_default()
    }

    /// Greater than or equal query
    pub fn greater_than_equal<T: Into<Value>>(attribute: impl Into<String>, value: T) -> String {
        serde_json::to_string(&Self::with_value("greaterThanEqual", attribute, value))
            .unwrap_or_default()
    }

    /// Is null query
    pub fn is_null(attribute: impl Into<String>) -> String {
        serde_json::to_string(&Self::with_attribute("isNull", attribute))
            .unwrap_or_default()
    }

    /// Is not null query
    pub fn is_not_null(attribute: impl Into<String>) -> String {
        serde_json::to_string(&Self::with_attribute("isNotNull", attribute))
            .unwrap_or_default()
    }

    /// Between query
    pub fn between<T: Into<Value>>(attribute: impl Into<String>, start: T, end: T) -> String {
        serde_json::to_string(&Self::with_values(
            "between",
            attribute,
            vec![start.into(), end.into()],
        ))
        .unwrap_or_default()
    }

    /// Starts with query
    pub fn starts_with(attribute: impl Into<String>, value: impl Into<String>) -> String {
        serde_json::to_string(&Self::with_value("startsWith", attribute, value.into()))
            .unwrap_or_default()
    }

    /// Ends with query
    pub fn ends_with(attribute: impl Into<String>, value: impl Into<String>) -> String {
        serde_json::to_string(&Self::with_value("endsWith", attribute, value.into()))
            .unwrap_or_default()
    }

    /// Select query
    pub fn select(attributes: Vec<String>) -> String {
        let values: Vec<Value> = attributes.into_iter().map(Value::String).collect();
        serde_json::to_string(&Query {
            method: "select".to_string(),
            attribute: None,
            values: Some(values),
        })
        .unwrap_or_default()
    }

    /// Search query
    pub fn search(attribute: impl Into<String>, value: impl Into<String>) -> String {
        serde_json::to_string(&Self::with_value("search", attribute, value.into()))
            .unwrap_or_default()
    }

    /// Order ascending query
    pub fn order_asc(attribute: impl Into<String>) -> String {
        serde_json::to_string(&Self::with_attribute("orderAsc", attribute))
            .unwrap_or_default()
    }

    /// Order descending query
    pub fn order_desc(attribute: impl Into<String>) -> String {
        serde_json::to_string(&Self::with_attribute("orderDesc", attribute))
            .unwrap_or_default()
    }

    /// Cursor before query
    pub fn cursor_before(id: impl Into<String>) -> String {
        serde_json::to_string(&Query {
            method: "cursorBefore".to_string(),
            attribute: None,
            values: Some(vec![Value::String(id.into())]),
        })
        .unwrap_or_default()
    }

    /// Cursor after query
    pub fn cursor_after(id: impl Into<String>) -> String {
        serde_json::to_string(&Query {
            method: "cursorAfter".to_string(),
            attribute: None,
            values: Some(vec![Value::String(id.into())]),
        })
        .unwrap_or_default()
    }

    /// Limit query
    pub fn limit(limit: u32) -> String {
        serde_json::to_string(&Query {
            method: "limit".to_string(),
            attribute: None,
            values: Some(vec![Value::Number(limit.into())]),
        })
        .unwrap_or_default()
    }

    /// Offset query
    pub fn offset(offset: u32) -> String {
        serde_json::to_string(&Query {
            method: "offset".to_string(),
            attribute: None,
            values: Some(vec![Value::Number(offset.into())]),
        })
        .unwrap_or_default()
    }

    /// Contains query
    pub fn contains<T: Into<Value>>(attribute: impl Into<String>, value: T) -> String {
        serde_json::to_string(&Self::with_value("contains", attribute, value))
            .unwrap_or_default()
    }

    /// OR queries
    pub fn or_queries(queries: Vec<String>) -> String {
        let parsed_queries: Vec<Value> = queries
            .into_iter()
            .filter_map(|q| serde_json::from_str(&q).ok())
            .collect();

        serde_json::to_string(&Query {
            method: "or".to_string(),
            attribute: None,
            values: Some(parsed_queries),
        })
        .unwrap_or_default()
    }

    /// AND queries
    pub fn and_queries(queries: Vec<String>) -> String {
        let parsed_queries: Vec<Value> = queries
            .into_iter()
            .filter_map(|q| serde_json::from_str(&q).ok())
            .collect();

        serde_json::to_string(&Query {
            method: "and".to_string(),
            attribute: None,
            values: Some(parsed_queries),
        })
        .unwrap_or_default()
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}