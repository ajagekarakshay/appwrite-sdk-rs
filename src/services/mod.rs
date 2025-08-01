//! Appwrite API services

pub mod account;
pub mod avatars;
pub mod databases;
pub mod functions;
pub mod graphql;
pub mod health;
pub mod locale;
pub mod messaging;
pub mod sites;
pub mod storage;
pub mod teams;
pub mod tokens;
pub mod users;

pub use account::Account;
pub use avatars::Avatars;
pub use databases::Databases;
pub use functions::Functions;
pub use graphql::GraphQL;
pub use health::Health;
pub use locale::Locale;
pub use messaging::Messaging;
pub use sites::Sites;
pub use storage::Storage;
pub use teams::Teams;
pub use tokens::Tokens;
pub use users::Users;