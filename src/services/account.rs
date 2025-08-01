//! Account service for user authentication and management

use crate::{client::Client, error::Result, enums::*};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Account service for user authentication and management
#[derive(Debug, Clone)]
pub struct Account<'a> {
    client: &'a Client,
}

impl<'a> Account<'a> {
    /// Create a new Account service instance
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get the currently logged in user
    ///
    /// # Example
    /// ```rust,no_run
    /// use appwrite::{Client, Account};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new()
    ///         .set_endpoint("https://cloud.appwrite.io/v1")?
    ///         .set_project("your-project-id");
    ///
    ///     let account = Account::new(&client);
    ///     let user = account.get().await?;
    ///     println!("User: {:?}", user);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self) -> Result<Value> {
        self.client
            .call("get", "/account", None, None, None)
            .await
    }

    /// Create a new user account
    ///
    /// # Arguments
    /// * `user_id` - User ID. Choose a custom ID or generate a random ID
    /// * `email` - User email
    /// * `password` - User password. Must be between 8 and 256 chars
    /// * `name` - User name. Optional
    ///
    /// # Example
    /// ```rust,no_run
    /// use appwrite::{Client, Account};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new()
    ///         .set_endpoint("https://cloud.appwrite.io/v1")?
    ///         .set_project("your-project-id");
    ///
    ///     let account = Account::new(&client);
    ///     let user = account.create(
    ///         "unique_user_id",
    ///         "user@example.com",
    ///         "securepassword",
    ///         Some("John Doe")
    ///     ).await?;
    ///     println!("Created user: {:?}", user);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(
        &self,
        user_id: &str,
        email: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<Value> {
        if user_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("user_id"));
        }
        if email.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("email"));
        }
        if password.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("password"));
        }

        let mut params = Map::new();
        params.insert("userId".to_string(), Value::String(user_id.to_string()));
        params.insert("email".to_string(), Value::String(email.to_string()));
        params.insert("password".to_string(), Value::String(password.to_string()));
        
        if let Some(name) = name {
            params.insert("name".to_string(), Value::String(name.to_string()));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("post", "/account", Some(headers), Some(params), None)
            .await
    }

    /// Update user email
    ///
    /// # Arguments
    /// * `email` - User email
    /// * `password` - User password for security verification
    pub async fn update_email(&self, email: &str, password: &str) -> Result<Value> {
        if email.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("email"));
        }
        if password.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("password"));
        }

        let mut params = Map::new();
        params.insert("email".to_string(), Value::String(email.to_string()));
        params.insert("password".to_string(), Value::String(password.to_string()));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("patch", "/account/email", Some(headers), Some(params), None)
            .await
    }

    /// Update user name
    ///
    /// # Arguments
    /// * `name` - User name
    pub async fn update_name(&self, name: &str) -> Result<Value> {
        if name.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("name"));
        }

        let mut params = Map::new();
        params.insert("name".to_string(), Value::String(name.to_string()));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("patch", "/account/name", Some(headers), Some(params), None)
            .await
    }

    /// Update user password
    ///
    /// # Arguments
    /// * `password` - New user password
    /// * `old_password` - Current user password (optional if user has no password set)
    pub async fn update_password(&self, password: &str, old_password: Option<&str>) -> Result<Value> {
        if password.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("password"));
        }

        let mut params = Map::new();
        params.insert("password".to_string(), Value::String(password.to_string()));
        
        if let Some(old_password) = old_password {
            params.insert("oldPassword".to_string(), Value::String(old_password.to_string()));
        }

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("patch", "/account/password", Some(headers), Some(params), None)
            .await
    }

    /// Create email password session
    ///
    /// # Arguments
    /// * `email` - User email
    /// * `password` - User password
    pub async fn create_email_password_session(&self, email: &str, password: &str) -> Result<Value> {
        if email.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("email"));
        }
        if password.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("password"));
        }

        let mut params = Map::new();
        params.insert("email".to_string(), Value::String(email.to_string()));
        params.insert("password".to_string(), Value::String(password.to_string()));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("post", "/account/sessions/email", Some(headers), Some(params), None)
            .await
    }

    /// Create anonymous session
    pub async fn create_anonymous_session(&self) -> Result<Value> {
        let headers = HashMap::new();
        let params = Map::new();

        self.client
            .call("post", "/account/sessions/anonymous", Some(headers), Some(params), None)
            .await
    }

    /// Create OAuth2 session
    ///
    /// # Arguments
    /// * `provider` - OAuth2 provider
    /// * `success` - URL to redirect back to your app after a successful login attempt
    /// * `failure` - URL to redirect back to your app after a failed login attempt
    /// * `scopes` - A list of custom OAuth2 scopes (optional)
    pub async fn create_oauth2_session(
        &self,
        provider: OAuthProvider,
        success: Option<&str>,
        failure: Option<&str>,
        scopes: Option<Vec<String>>,
    ) -> Result<Value> {
        let mut params = Map::new();
        params.insert("provider".to_string(), Value::String(provider.as_ref().to_string()));
        
        if let Some(success) = success {
            params.insert("success".to_string(), Value::String(success.to_string()));
        }
        if let Some(failure) = failure {
            params.insert("failure".to_string(), Value::String(failure.to_string()));
        }
        if let Some(scopes) = scopes {
            let scopes_value: Vec<Value> = scopes.into_iter().map(Value::String).collect();
            params.insert("scopes".to_string(), Value::Array(scopes_value));
        }

        let headers = HashMap::new();

        self.client
            .call("get", &format!("/account/sessions/oauth2/{}", provider.as_ref()), Some(headers), Some(params), Some("location"))
            .await
    }

    /// Get current session
    pub async fn get_session(&self, session_id: &str) -> Result<Value> {
        if session_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("session_id"));
        }

        let path = format!("/account/sessions/{}", session_id);
        
        self.client
            .call("get", &path, None, None, None)
            .await
    }

    /// List all user sessions
    pub async fn list_sessions(&self) -> Result<Value> {
        self.client
            .call("get", "/account/sessions", None, None, None)
            .await
    }

    /// Delete a specific session
    pub async fn delete_session(&self, session_id: &str) -> Result<Value> {
        if session_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("session_id"));
        }

        let path = format!("/account/sessions/{}", session_id);
        
        self.client
            .call("delete", &path, None, None, None)
            .await
    }

    /// Delete all user sessions
    pub async fn delete_sessions(&self) -> Result<Value> {
        self.client
            .call("delete", "/account/sessions", None, None, None)
            .await
    }

    /// Create email verification
    ///
    /// # Arguments
    /// * `url` - URL to redirect the user back to your app from the verification email
    pub async fn create_verification(&self, url: &str) -> Result<Value> {
        if url.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("url"));
        }

        let mut params = Map::new();
        params.insert("url".to_string(), Value::String(url.to_string()));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("post", "/account/verification", Some(headers), Some(params), None)
            .await
    }

    /// Confirm email verification
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `secret` - Valid verification token
    pub async fn update_verification(&self, user_id: &str, secret: &str) -> Result<Value> {
        if user_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("user_id"));
        }
        if secret.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("secret"));
        }

        let mut params = Map::new();
        params.insert("userId".to_string(), Value::String(user_id.to_string()));
        params.insert("secret".to_string(), Value::String(secret.to_string()));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("put", "/account/verification", Some(headers), Some(params), None)
            .await
    }

    /// Create password recovery
    ///
    /// # Arguments
    /// * `email` - User email
    /// * `url` - URL to redirect the user back to your app from the recovery email
    pub async fn create_recovery(&self, email: &str, url: &str) -> Result<Value> {
        if email.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("email"));
        }
        if url.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("url"));
        }

        let mut params = Map::new();
        params.insert("email".to_string(), Value::String(email.to_string()));
        params.insert("url".to_string(), Value::String(url.to_string()));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("post", "/account/recovery", Some(headers), Some(params), None)
            .await
    }

    /// Complete password recovery
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `secret` - Valid recovery token
    /// * `password` - New password
    /// * `password_again` - Repeat new password
    pub async fn update_recovery(
        &self,
        user_id: &str,
        secret: &str,
        password: &str,
        password_again: &str,
    ) -> Result<Value> {
        if user_id.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("user_id"));
        }
        if secret.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("secret"));
        }
        if password.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("password"));
        }
        if password_again.is_empty() {
            return Err(crate::error::AppwriteError::missing_parameter("password_again"));
        }

        let mut params = Map::new();
        params.insert("userId".to_string(), Value::String(user_id.to_string()));
        params.insert("secret".to_string(), Value::String(secret.to_string()));
        params.insert("password".to_string(), Value::String(password.to_string()));
        params.insert("passwordAgain".to_string(), Value::String(password_again.to_string()));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("put", "/account/recovery", Some(headers), Some(params), None)
            .await
    }

    /// Get user preferences
    pub async fn get_prefs(&self) -> Result<Value> {
        self.client
            .call("get", "/account/prefs", None, None, None)
            .await
    }

    /// Update user preferences
    ///
    /// # Arguments
    /// * `prefs` - Preferences key-value JSON object
    pub async fn update_prefs(&self, prefs: Value) -> Result<Value> {
        let mut params = Map::new();
        params.insert("prefs".to_string(), prefs);

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        self.client
            .call("patch", "/account/prefs", Some(headers), Some(params), None)
            .await
    }
}