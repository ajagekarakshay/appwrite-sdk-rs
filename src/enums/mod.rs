//! Enums for Appwrite SDK

pub mod oauth_provider;
pub mod authenticator_type;
pub mod authentication_factor;
pub mod browser;
pub mod credit_card;
pub mod flag;
pub mod image_format;
pub mod image_gravity;
pub mod index_type;
pub mod runtime;
pub mod relationship_type;
pub mod compression;

pub use oauth_provider::OAuthProvider;
pub use authenticator_type::AuthenticatorType;
pub use authentication_factor::AuthenticationFactor;
pub use browser::Browser;
pub use credit_card::CreditCard;
pub use flag::Flag;
pub use image_format::ImageFormat;
pub use image_gravity::ImageGravity;
pub use index_type::IndexType;
pub use runtime::Runtime;
pub use relationship_type::RelationshipType;
pub use compression::Compression;