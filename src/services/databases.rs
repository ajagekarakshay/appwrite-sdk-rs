//! Databases service for database operations

use crate::{client::Client, error::Result, enums::*};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Databases service for database operations
#[derive(Debug, Clone)]
pub struct Databases<'a> {
    client: &'a Client,
}

impl<'a> Databases<'a> {
    /// Create a new Databases service instance
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all databases
    pub async fn list(&self, queries: Option<Vec<String>>, search: Option<&str>) -> Result<Value> {
        let mut params = Map::new();
        
        if let Some(queries) = queries {
            let queries_value: Vec<Value> = queries.into_iter().map(Value::String).collect();
            params.insert("queries".to_string(), Value::Array(queries_value));
        }
        if let Some(search) = search {
            params.insert("search".to_string(), Value::String(search.to_string()));
        }

        self.client
            .call("get", "/databases", None, Some(params), None)
            .await
    }

    /// Create a new database
    pub async fn create(&self, database_id: &str, name: &str, enabled: Option<bool>) -> Result<Value> {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }
        if name.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("name"));
        }

        let mut params = Map::new();
        params.insert("databaseId".to_string(), Value::String(database_id.to_string()));
        params.insert("name".to_string(), Value::String(name.to_string()));
        
        if let Some(enabled) = enabled {
            params.insert("enabled".to_string(), Value::Bool(enabled));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("post", "/databases", Some(headers), Some(params), None)
            .await
    }

    /// Get a database by ID
    pub async fn get(&self, database_id: &str) -> Result<Value> {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }

        let path = format!("/databases/{}", database_id);
        
        self.client
            .call("get", &path, None, None, None)
            .await
    }

    /// Create a collection
    pub async fn create_collection(
        &self,
        database_id: &str,
        collection_id: &str,
        name: &str,
        permissions: Option<Vec<String>>,
        document_security: Option<bool>,
        enabled: Option<bool>,
    ) -> Result<Value> {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }
        if collection_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("collection_id"));
        }
        if name.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("name"));
        }

        let mut params = Map::new();
        params.insert("collectionId".to_string(), Value::String(collection_id.to_string()));
        params.insert("name".to_string(), Value::String(name.to_string()));
        
        if let Some(permissions) = permissions {
            let perms_value: Vec<Value> = permissions.into_iter().map(Value::String).collect();
            params.insert("permissions".to_string(), Value::Array(perms_value));
        }
        if let Some(document_security) = document_security {
            params.insert("documentSecurity".to_string(), Value::Bool(document_security));
        }
        if let Some(enabled) = enabled {
            params.insert("enabled".to_string(), Value::Bool(enabled));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let path = format!("/databases/{}/collections", database_id);
        
        self.client
            .call("post", &path, Some(headers), Some(params), None)
            .await
    }

    /// Create a string attribute
    pub async fn create_string_attribute(
        &self,
        database_id: &str,
        collection_id: &str,
        key: &str,
        size: u32,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> Result<Value> {
        self.create_attribute(database_id, collection_id, "string", |params| {
            params.insert("key".to_string(), Value::String(key.to_string()));
            params.insert("size".to_string(), Value::Number(size.into()));
            params.insert("required".to_string(), Value::Bool(required));
            
            if let Some(default) = default {
                params.insert("default".to_string(), Value::String(default.to_string()));
            }
            if let Some(array) = array {
                params.insert("array".to_string(), Value::Bool(array));
            }
            if let Some(encrypt) = encrypt {
                params.insert("encrypt".to_string(), Value::Bool(encrypt));
            }
        }).await
    }

    /// Create an email attribute
    pub async fn create_email_attribute(
        &self,
        database_id: &str,
        collection_id: &str,
        key: &str,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> Result<Value> {
        self.create_attribute(database_id, collection_id, "email", |params| {
            params.insert("key".to_string(), Value::String(key.to_string()));
            params.insert("required".to_string(), Value::Bool(required));
            
            if let Some(default) = default {
                params.insert("default".to_string(), Value::String(default.to_string()));
            }
            if let Some(array) = array {
                params.insert("array".to_string(), Value::Bool(array));
            }
        }).await
    }

    /// Create an integer attribute
    pub async fn create_integer_attribute(
        &self,
        database_id: &str,
        collection_id: &str,
        key: &str,
        required: bool,
        min: Option<i64>,
        max: Option<i64>,
        default: Option<i64>,
        array: Option<bool>,
    ) -> Result<Value> {
        self.create_attribute(database_id, collection_id, "integer", |params| {
            params.insert("key".to_string(), Value::String(key.to_string()));
            params.insert("required".to_string(), Value::Bool(required));
            
            if let Some(min) = min {
                params.insert("min".to_string(), Value::Number(min.into()));
            }
            if let Some(max) = max {
                params.insert("max".to_string(), Value::Number(max.into()));
            }
            if let Some(default) = default {
                params.insert("default".to_string(), Value::Number(default.into()));
            }
            if let Some(array) = array {
                params.insert("array".to_string(), Value::Bool(array));
            }
        }).await
    }

    /// Create a document
    pub async fn create_document(
        &self,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
        data: Value,
        permissions: Option<Vec<String>>,
    ) -> Result<Value> {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }
        if collection_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("collection_id"));
        }
        if document_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("document_id"));
        }

        let mut params = Map::new();
        params.insert("documentId".to_string(), Value::String(document_id.to_string()));
        params.insert("data".to_string(), data);
        
        if let Some(permissions) = permissions {
            let perms_value: Vec<Value> = permissions.into_iter().map(Value::String).collect();
            params.insert("permissions".to_string(), Value::Array(perms_value));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let path = format!("/databases/{}/collections/{}/documents", database_id, collection_id);
        
        self.client
            .call("post", &path, Some(headers), Some(params), None)
            .await
    }

    /// Get a document
    pub async fn get_document(
        &self,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<Value> {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }
        if collection_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("collection_id"));
        }
        if document_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("document_id"));
        }

        let mut params = Map::new();
        if let Some(queries) = queries {
            let queries_value: Vec<Value> = queries.into_iter().map(Value::String).collect();
            params.insert("queries".to_string(), Value::Array(queries_value));
        }

        let path = format!("/databases/{}/collections/{}/documents/{}", database_id, collection_id, document_id);
        
        self.client
            .call("get", &path, None, Some(params), None)
            .await
    }

    /// Update a document
    pub async fn update_document(
        &self,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
        data: Option<Value>,
        permissions: Option<Vec<String>>,
    ) -> Result<Value> {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }
        if collection_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("collection_id"));
        }
        if document_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("document_id"));
        }

        let mut params = Map::new();
        
        if let Some(data) = data {
            params.insert("data".to_string(), data);
        }
        if let Some(permissions) = permissions {
            let perms_value: Vec<Value> = permissions.into_iter().map(Value::String).collect();
            params.insert("permissions".to_string(), Value::Array(perms_value));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let path = format!("/databases/{}/collections/{}/documents/{}", database_id, collection_id, document_id);
        
        self.client
            .call("patch", &path, Some(headers), Some(params), None)
            .await
    }

    /// List documents
    pub async fn list_documents(
        &self,
        database_id: &str,
        collection_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<Value> {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }
        if collection_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("collection_id"));
        }

        let mut params = Map::new();
        if let Some(queries) = queries {
            let queries_value: Vec<Value> = queries.into_iter().map(Value::String).collect();
            params.insert("queries".to_string(), Value::Array(queries_value));
        }

        let path = format!("/databases/{}/collections/{}/documents", database_id, collection_id);
        
        self.client
            .call("get", &path, None, Some(params), None)
            .await
    }

    /// Create an index
    pub async fn create_index(
        &self,
        database_id: &str,
        collection_id: &str,
        key: &str,
        index_type: IndexType,
        attributes: Vec<String>,
        orders: Option<Vec<String>>,
    ) -> Result<Value> {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }
        if collection_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("collection_id"));
        }
        if key.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("key"));
        }

        let mut params = Map::new();
        params.insert("key".to_string(), Value::String(key.to_string()));
        params.insert("type".to_string(), Value::String(index_type.as_ref().to_string()));
        
        let attrs_value: Vec<Value> = attributes.into_iter().map(Value::String).collect();
        params.insert("attributes".to_string(), Value::Array(attrs_value));
        
        if let Some(orders) = orders {
            let orders_value: Vec<Value> = orders.into_iter().map(Value::String).collect();
            params.insert("orders".to_string(), Value::Array(orders_value));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let path = format!("/databases/{}/collections/{}/indexes", database_id, collection_id);
        
        self.client
            .call("post", &path, Some(headers), Some(params), None)
            .await
    }

    // Helper method for creating attributes
    async fn create_attribute<F>(
        &self,
        database_id: &str,
        collection_id: &str,
        attr_type: &str,
        param_builder: F,
    ) -> Result<Value>
    where
        F: FnOnce(&mut Map<String, Value>),
    {
        if database_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("database_id"));
        }
        if collection_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("collection_id"));
        }

        let mut params = Map::new();
        param_builder(&mut params);

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let path = format!("/databases/{}/collections/{}/attributes/{}", database_id, collection_id, attr_type);
        
        self.client
            .call("post", &path, Some(headers), Some(params), None)
            .await
    }
}