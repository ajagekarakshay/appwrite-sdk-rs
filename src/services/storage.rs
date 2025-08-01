//! Storage service for file operations

use crate::{client::Client, error::Result, enums::*, input_file::InputFile};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Storage service for file operations
#[derive(Debug, Clone)]
pub struct Storage<'a> {
    client: &'a Client,
}

impl<'a> Storage<'a> {
    /// Create a new Storage service instance
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all storage buckets
    pub async fn list_buckets(&self, queries: Option<Vec<String>>, search: Option<&str>) -> Result<Value> {
        let mut params = Map::new();
        
        if let Some(queries) = queries {
            let queries_value: Vec<Value> = queries.into_iter().map(Value::String).collect();
            params.insert("queries".to_string(), Value::Array(queries_value));
        }
        if let Some(search) = search {
            params.insert("search".to_string(), Value::String(search.to_string()));
        }

        self.client
            .call("get", "/storage/buckets", None, Some(params), None)
            .await
    }

    /// Create a new storage bucket
    #[allow(clippy::too_many_arguments)]
    pub async fn create_bucket(
        &self,
        bucket_id: &str,
        name: &str,
        permissions: Option<Vec<String>>,
        file_security: Option<bool>,
        enabled: Option<bool>,
        maximum_file_size: Option<u64>,
        allowed_file_extensions: Option<Vec<String>>,
        compression: Option<Compression>,
        encryption: Option<bool>,
        antivirus: Option<bool>,
    ) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }
        if name.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("name"));
        }

        let mut params = Map::new();
        params.insert("bucketId".to_string(), Value::String(bucket_id.to_string()));
        params.insert("name".to_string(), Value::String(name.to_string()));
        
        if let Some(permissions) = permissions {
            let perms_value: Vec<Value> = permissions.into_iter().map(Value::String).collect();
            params.insert("permissions".to_string(), Value::Array(perms_value));
        }
        if let Some(file_security) = file_security {
            params.insert("fileSecurity".to_string(), Value::Bool(file_security));
        }
        if let Some(enabled) = enabled {
            params.insert("enabled".to_string(), Value::Bool(enabled));
        }
        if let Some(maximum_file_size) = maximum_file_size {
            params.insert("maximumFileSize".to_string(), Value::Number(maximum_file_size.into()));
        }
        if let Some(allowed_file_extensions) = allowed_file_extensions {
            let exts_value: Vec<Value> = allowed_file_extensions.into_iter().map(Value::String).collect();
            params.insert("allowedFileExtensions".to_string(), Value::Array(exts_value));
        }
        if let Some(compression) = compression {
            params.insert("compression".to_string(), Value::String(compression.as_ref().to_string()));
        }
        if let Some(encryption) = encryption {
            params.insert("encryption".to_string(), Value::Bool(encryption));
        }
        if let Some(antivirus) = antivirus {
            params.insert("antivirus".to_string(), Value::Bool(antivirus));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("post", "/storage/buckets", Some(headers), Some(params), None)
            .await
    }

    /// Get a bucket by ID
    pub async fn get_bucket(&self, bucket_id: &str) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }

        let path = format!("/storage/buckets/{}", bucket_id);
        
        self.client
            .call("get", &path, None, None, None)
            .await
    }

    /// Update a bucket
    #[allow(clippy::too_many_arguments)]
    pub async fn update_bucket(
        &self,
        bucket_id: &str,
        name: &str,
        permissions: Option<Vec<String>>,
        file_security: Option<bool>,
        enabled: Option<bool>,
        maximum_file_size: Option<u64>,
        allowed_file_extensions: Option<Vec<String>>,
        compression: Option<Compression>,
        encryption: Option<bool>,
        antivirus: Option<bool>,
    ) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }
        if name.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("name"));
        }

        let mut params = Map::new();
        params.insert("name".to_string(), Value::String(name.to_string()));
        
        if let Some(permissions) = permissions {
            let perms_value: Vec<Value> = permissions.into_iter().map(Value::String).collect();
            params.insert("permissions".to_string(), Value::Array(perms_value));
        }
        if let Some(file_security) = file_security {
            params.insert("fileSecurity".to_string(), Value::Bool(file_security));
        }
        if let Some(enabled) = enabled {
            params.insert("enabled".to_string(), Value::Bool(enabled));
        }
        if let Some(maximum_file_size) = maximum_file_size {
            params.insert("maximumFileSize".to_string(), Value::Number(maximum_file_size.into()));
        }
        if let Some(allowed_file_extensions) = allowed_file_extensions {
            let exts_value: Vec<Value> = allowed_file_extensions.into_iter().map(Value::String).collect();
            params.insert("allowedFileExtensions".to_string(), Value::Array(exts_value));
        }
        if let Some(compression) = compression {
            params.insert("compression".to_string(), Value::String(compression.as_ref().to_string()));
        }
        if let Some(encryption) = encryption {
            params.insert("encryption".to_string(), Value::Bool(encryption));
        }
        if let Some(antivirus) = antivirus {
            params.insert("antivirus".to_string(), Value::Bool(antivirus));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let path = format!("/storage/buckets/{}", bucket_id);
        
        self.client
            .call("put", &path, Some(headers), Some(params), None)
            .await
    }

    /// Create a file
    pub async fn create_file(
        &self,
        bucket_id: &str,
        file_id: &str,
        file: InputFile,
        permissions: Option<Vec<String>>,
    ) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }
        if file_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("file_id"));
        }

        let mut params = Map::new();
        params.insert("fileId".to_string(), Value::String(file_id.to_string()));
        
        if let Some(permissions) = permissions {
            let perms_value: Vec<Value> = permissions.into_iter().map(Value::String).collect();
            params.insert("permissions".to_string(), Value::Array(perms_value));
        }

        // For now, we'll simulate file upload by adding file info to params
        // In a real implementation, this would handle multipart/form-data
        params.insert("file".to_string(), Value::String(format!("file:{}", file.filename())));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "multipart/form-data".to_string());

        let path = format!("/storage/buckets/{}/files", bucket_id);
        
        self.client
            .call("post", &path, Some(headers), Some(params), None)
            .await
    }

    /// Get a file
    pub async fn get_file(&self, bucket_id: &str, file_id: &str) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }
        if file_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("file_id"));
        }

        let path = format!("/storage/buckets/{}/files/{}", bucket_id, file_id);
        
        self.client
            .call("get", &path, None, None, None)
            .await
    }

    /// Update a file
    pub async fn update_file(
        &self,
        bucket_id: &str,
        file_id: &str,
        name: Option<&str>,
        permissions: Option<Vec<String>>,
    ) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }
        if file_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("file_id"));
        }

        let mut params = Map::new();
        
        if let Some(name) = name {
            params.insert("name".to_string(), Value::String(name.to_string()));
        }
        if let Some(permissions) = permissions {
            let perms_value: Vec<Value> = permissions.into_iter().map(Value::String).collect();
            params.insert("permissions".to_string(), Value::Array(perms_value));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let path = format!("/storage/buckets/{}/files/{}", bucket_id, file_id);
        
        self.client
            .call("put", &path, Some(headers), Some(params), None)
            .await
    }

    /// List files in a bucket
    pub async fn list_files(
        &self,
        bucket_id: &str,
        queries: Option<Vec<String>>,
        search: Option<&str>,
    ) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }

        let mut params = Map::new();
        
        if let Some(queries) = queries {
            let queries_value: Vec<Value> = queries.into_iter().map(Value::String).collect();
            params.insert("queries".to_string(), Value::Array(queries_value));
        }
        if let Some(search) = search {
            params.insert("search".to_string(), Value::String(search.to_string()));
        }

        let path = format!("/storage/buckets/{}/files", bucket_id);
        
        self.client
            .call("get", &path, None, Some(params), None)
            .await
    }

    /// Get file download URL
    pub async fn get_file_download(&self, bucket_id: &str, file_id: &str) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }
        if file_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("file_id"));
        }

        let path = format!("/storage/buckets/{}/files/{}/download", bucket_id, file_id);
        
        self.client
            .call("get", &path, None, None, Some("location"))
            .await
    }

    /// Get file preview URL
    #[allow(clippy::too_many_arguments)]
    pub async fn get_file_preview(
        &self,
        bucket_id: &str,
        file_id: &str,
        width: Option<u32>,
        height: Option<u32>,
        gravity: Option<ImageGravity>,
        quality: Option<u8>,
        border_width: Option<u32>,
        border_color: Option<&str>,
        border_radius: Option<u32>,
        opacity: Option<f32>,
        rotation: Option<i32>,
        background: Option<&str>,
        output: Option<ImageFormat>,
    ) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }
        if file_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("file_id"));
        }

        let mut params = Map::new();
        
        if let Some(width) = width {
            params.insert("width".to_string(), Value::Number(width.into()));
        }
        if let Some(height) = height {
            params.insert("height".to_string(), Value::Number(height.into()));
        }
        if let Some(gravity) = gravity {
            params.insert("gravity".to_string(), Value::String(gravity.as_ref().to_string()));
        }
        if let Some(quality) = quality {
            params.insert("quality".to_string(), Value::Number(quality.into()));
        }
        if let Some(border_width) = border_width {
            params.insert("borderWidth".to_string(), Value::Number(border_width.into()));
        }
        if let Some(border_color) = border_color {
            params.insert("borderColor".to_string(), Value::String(border_color.to_string()));
        }
        if let Some(border_radius) = border_radius {
            params.insert("borderRadius".to_string(), Value::Number(border_radius.into()));
        }
        if let Some(opacity) = opacity {
            params.insert("opacity".to_string(), Value::Number(serde_json::Number::from_f64(opacity as f64).unwrap()));
        }
        if let Some(rotation) = rotation {
            params.insert("rotation".to_string(), Value::Number(rotation.into()));
        }
        if let Some(background) = background {
            params.insert("background".to_string(), Value::String(background.to_string()));
        }
        if let Some(output) = output {
            params.insert("output".to_string(), Value::String(output.as_ref().to_string()));
        }

        let path = format!("/storage/buckets/{}/files/{}/preview", bucket_id, file_id);
        
        self.client
            .call("get", &path, None, Some(params), Some("location"))
            .await
    }

    /// Get file view URL
    pub async fn get_file_view(&self, bucket_id: &str, file_id: &str) -> Result<Value> {
        if bucket_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("bucket_id"));
        }
        if file_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("file_id"));
        }

        let path = format!("/storage/buckets/{}/files/{}/view", bucket_id, file_id);
        
        self.client
            .call("get", &path, None, None, Some("location"))
            .await
    }
}