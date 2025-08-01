//! Error types for the Appwrite SDK using thiserror

// use std::fmt; // Currently unused
use thiserror::Error;

/// Result type alias for Appwrite operations
pub type Result<T> = std::result::Result<T, AppwriteError>;

/// Appwrite SDK error types
#[derive(Error, Debug)]
pub enum AppwriteError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// API error from Appwrite server
    #[error("API error ({code}): {message}")]
    Api {
        message: String,
        code: u16,
        error_type: Option<String>,
        response: Option<String>,
    },

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid parameter error
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Missing required parameter
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    /// Invalid endpoint URL
    #[error("Invalid endpoint URL: {0}")]
    InvalidEndpoint(String),

    /// File operation error
    #[error("File operation error: {0}")]
    FileError(String),

    /// URL parsing error
    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error for other cases
    #[error("Appwrite error: {0}")]
    Generic(String),
}

impl AppwriteError {
    /// Create a new API error
    pub fn api(message: impl Into<String>, code: u16) -> Self {
        Self::Api {
            message: message.into(),
            code,
            error_type: None,
            response: None,
        }
    }

    /// Create a new API error with type and response
    pub fn api_with_details(
        message: impl Into<String>,
        code: u16,
        error_type: Option<String>,
        response: Option<String>,
    ) -> Self {
        Self::Api {
            message: message.into(),
            code,
            error_type,
            response,
        }
    }

    /// Create a missing parameter error
    pub fn missing_parameter(param: impl Into<String>) -> Self {
        Self::MissingParameter(param.into())
    }

    /// Create an invalid parameter error
    pub fn invalid_parameter(param: impl Into<String>) -> Self {
        Self::InvalidParameter(param.into())
    }

    /// Create a file error
    pub fn file_error(message: impl Into<String>) -> Self {
        Self::FileError(message.into())
    }

    /// Get the error code if this is an API error
    pub fn code(&self) -> Option<u16> {
        match self {
            Self::Api { code, .. } => Some(*code),
            _ => None,
        }
    }

    /// Get the error type if this is an API error
    pub fn error_type(&self) -> Option<&str> {
        match self {
            Self::Api { error_type, .. } => error_type.as_deref(),
            _ => None,
        }
    }

    /// Get the response body if this is an API error
    pub fn response(&self) -> Option<&str> {
        match self {
            Self::Api { response, .. } => response.as_deref(),
            _ => None,
        }
    }
}