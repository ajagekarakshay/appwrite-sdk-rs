//! HTTP client for Appwrite API

use crate::error::{AppwriteError, Result};
// use crate::input_file::InputFile; // Currently unused
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response};
use serde_json::{Map, Value};
use std::collections::HashMap;
use url::Url;

/// HTTP client for communicating with Appwrite API
#[derive(Debug, Clone)]
pub struct Client {
    http_client: ReqwestClient,
    endpoint: String,
    headers: HashMap<String, String>,
    chunk_size: usize,
    self_signed: bool,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Create a new Appwrite client
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        
        // Set default headers similar to Python SDK
        headers.insert("content-type".to_string(), "".to_string());
        headers.insert(
            "user-agent".to_string(),
            format!(
                "AppwriteRustSDK/0.1.0 ({}; {})",
                std::env::consts::OS,
                std::env::consts::ARCH
            ),
        );
        headers.insert("x-sdk-name".to_string(), "Rust".to_string());
        headers.insert("x-sdk-platform".to_string(), "server".to_string());
        headers.insert("x-sdk-language".to_string(), "rust".to_string());
        headers.insert("x-sdk-version".to_string(), "0.1.0".to_string());
        headers.insert("X-Appwrite-Response-Format".to_string(), "1.7.0".to_string());

        Self {
            http_client: ReqwestClient::new(),
            endpoint: "https://cloud.appwrite.io/v1".to_string(),
            headers,
            chunk_size: 5 * 1024 * 1024, // 5MB
            self_signed: false,
        }
    }

    /// Set the API endpoint
    pub fn set_endpoint(mut self, endpoint: impl Into<String>) -> Result<Self> {
        let endpoint = endpoint.into();
        
        // Validate endpoint URL
        if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
            return Err(AppwriteError::InvalidEndpoint(endpoint));
        }
        
        // Ensure URL is valid
        Url::parse(&endpoint).map_err(|_| AppwriteError::InvalidEndpoint(endpoint.clone()))?;
        
        self.endpoint = endpoint;
        Ok(self)
    }

    /// Set whether to allow self-signed certificates
    pub fn set_self_signed(mut self, status: bool) -> Self {
        self.self_signed = status;
        self
    }

    /// Add a custom header
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into().to_lowercase(), value.into());
        self
    }

    /// Set the project ID
    pub fn set_project(self, project_id: impl Into<String>) -> Self {
        self.add_header("x-appwrite-project", project_id)
    }

    /// Set the API key
    pub fn set_key(self, api_key: impl Into<String>) -> Self {
        self.add_header("x-appwrite-key", api_key)
    }

    /// Set the JWT token
    pub fn set_jwt(self, jwt: impl Into<String>) -> Self {
        self.add_header("x-appwrite-jwt", jwt)
    }

    /// Set the locale
    pub fn set_locale(self, locale: impl Into<String>) -> Self {
        self.add_header("x-appwrite-locale", locale)
    }

    /// Set the session
    pub fn set_session(self, session: impl Into<String>) -> Self {
        self.add_header("x-appwrite-session", session)
    }

    /// Set the forwarded user agent
    pub fn set_forwarded_user_agent(self, user_agent: impl Into<String>) -> Self {
        self.add_header("x-forwarded-user-agent", user_agent)
    }

    /// Make an HTTP request to the Appwrite API
    pub async fn call(
        &self,
        method: &str,
        path: &str,
        headers: Option<HashMap<String, String>>,
        params: Option<Map<String, Value>>,
        response_type: Option<&str>,
    ) -> Result<Value> {
        let method = method.to_uppercase();
        let method = Method::from_bytes(method.as_bytes())
            .map_err(|_| AppwriteError::InvalidParameter(format!("Invalid HTTP method: {}", method)))?;

        let url = format!("{}{}", self.endpoint, path);
        let mut request = self.http_client.request(method.clone(), &url);

        // Merge headers
        let mut all_headers = self.headers.clone();
        if let Some(custom_headers) = headers {
            all_headers.extend(custom_headers);
        }

        // Add headers to request
        for (key, value) in &all_headers {
            request = request.header(key, value);
        }

        // Handle request body based on method and content type
        if method != Method::GET {
            if let Some(params) = params {
                let default_content_type = String::new();
                let content_type = all_headers.get("content-type").unwrap_or(&default_content_type);
                
                if content_type.starts_with("application/json") {
                    request = request.json(&params);
                } else if content_type.starts_with("multipart/form-data") {
                    request = self.build_multipart_request(request, params).await?;
                } else {
                    // Default to form data
                    let form_data = self.flatten_params(&params);
                    request = request.form(&form_data);
                }
            }
        } else if let Some(params) = params {
            // For GET requests, add as query parameters
            let query_params = self.flatten_params(&params);
            request = request.query(&query_params);
        }

        // Configure SSL verification 
        // Note: reqwest doesn't expose danger_accept_invalid_certs in the public API
        // This would need to be configured at the client level if needed

        // Execute request
        let response = request.send().await?;
        
        self.handle_response(response, response_type).await
    }

    /// Handle the HTTP response
    async fn handle_response(&self, response: Response, response_type: Option<&str>) -> Result<Value> {
        // Check for warnings
        if let Some(warnings) = response.headers().get("x-appwrite-warning") {
            if let Ok(warning_str) = warnings.to_str() {
                for warning in warning_str.split(';') {
                    eprintln!("Warning: {}", warning);
                }
            }
        }

        let status = response.status();
        let content_type = response.headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("")
            .to_string(); // Clone to avoid borrow issues

        // Handle different response types
        match response_type {
            Some("location") => {
                let location = response.headers()
                    .get("location")
                    .and_then(|loc| loc.to_str().ok())
                    .unwrap_or("");
                return Ok(Value::String(location.to_string()));
            }
            _ => {}
        }

        if status.is_success() {
            if content_type.starts_with("application/json") {
                let json_value: Value = response.json().await?;
                Ok(json_value)
            } else {
                // For non-JSON responses, return as string
                let text = response.text().await?;
                Ok(Value::String(text))
            }
        } else {
            // Handle error response
            let response_text = response.text().await?;
            
            if content_type.starts_with("application/json") {
                if let Ok(error_json) = serde_json::from_str::<Value>(&response_text) {
                    let message = error_json.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Unknown error")
                        .to_string();
                    let error_type = error_json.get("type")
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string());
                    
                    return Err(AppwriteError::api_with_details(
                        message,
                        status.as_u16(),
                        error_type,
                        Some(response_text),
                    ));
                }
            }
            
            Err(AppwriteError::api_with_details(
                response_text.clone(),
                status.as_u16(),
                None,
                Some(response_text),
            ))
        }
    }

    /// Build multipart form request for file uploads
    async fn build_multipart_request(
        &self,
        mut request: RequestBuilder,
        params: Map<String, Value>,
    ) -> Result<RequestBuilder> {
        let mut form = reqwest::multipart::Form::new();

        for (key, value) in params {
            match value {
                Value::String(s) => {
                    form = form.text(key, s);
                }
                Value::Number(n) => {
                    form = form.text(key, n.to_string());
                }
                Value::Bool(b) => {
                    form = form.text(key, b.to_string());
                }
                Value::Array(_) | Value::Object(_) => {
                    form = form.text(key, serde_json::to_string(&value)?);
                }
                Value::Null => {
                    // Skip null values
                }
            }
        }

        request = request.multipart(form);
        Ok(request)
    }

    /// Flatten nested parameters for form data
    fn flatten_params(&self, params: &Map<String, Value>) -> HashMap<String, String> {
        let mut flattened = HashMap::new();
        let params_value = Value::Object(params.clone());
        self.flatten_value("", &params_value, &mut flattened);
        flattened
    }

    /// Recursively flatten a JSON value
    fn flatten_value(&self, prefix: &str, value: &Value, output: &mut HashMap<String, String>) {
        match value {
            Value::Object(obj) => {
                for (key, val) in obj {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}[{}]", prefix, key)
                    };
                    self.flatten_value(&new_prefix, val, output);
                }
            }
            Value::Array(arr) => {
                for (i, val) in arr.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, i);
                    self.flatten_value(&new_prefix, val, output);
                }
            }
            Value::String(s) => {
                output.insert(prefix.to_string(), s.clone());
            }
            Value::Number(n) => {
                output.insert(prefix.to_string(), n.to_string());
            }
            Value::Bool(b) => {
                output.insert(prefix.to_string(), b.to_string());
            }
            Value::Null => {
                // Skip null values
            }
        }
    }

    /// Get the chunk size for file uploads
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Set the chunk size for file uploads
    pub fn set_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }
}