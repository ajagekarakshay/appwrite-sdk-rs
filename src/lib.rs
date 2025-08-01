//! # Appwrite Rust SDK
//!
//! This is the Rust SDK for Appwrite, an open-source backend-as-a-service platform.
//!
//! ## Features
//!
//! - Account management and authentication
//! - Database operations with query builder
//! - File storage with chunked uploads
//! - Real-time subscriptions
//! - Teams and permissions
//! - Functions and messaging
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use appwrite::{Client, Account};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new()
//!         .set_endpoint("https://cloud.appwrite.io/v1")?
//!         .set_project("your-project-id");
//!
//!     let account = Account::new(&client);
//!     let user = account.get().await?;
//!     
//!     println!("User: {:?}", user);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod query;
pub mod permission;
pub mod input_file;
pub mod services;
pub mod enums;

pub use client::Client;
pub use error::{AppwriteError, Result};
pub use query::Query;
pub use permission::{Permission, Role};
pub use input_file::InputFile;

// Re-export all services for convenience
pub use services::{
    Account, Avatars, Databases, Functions, GraphQL, Health, 
    Locale, Messaging, Sites, Storage, Teams, Tokens, Users
};

// Re-export common enums
pub use enums::{
    OAuthProvider, AuthenticatorType, AuthenticationFactor,
    Browser, CreditCard, Flag, ImageFormat, ImageGravity,
    IndexType, Runtime, RelationshipType, Compression
};