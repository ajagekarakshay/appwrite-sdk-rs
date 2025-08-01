# Appwrite Rust SDK

[![Crates.io](https://img.shields.io/crates/v/appwrite-rs.svg)](https://crates.io/crates/appwrite-rs)
[![Documentation](https://docs.rs/appwrite-rs/badge.svg)](https://docs.rs/appwrite-rs)
[![License](https://img.shields.io/badge/license-BSD%203--Clause-blue.svg)](LICENSE)

The official Rust SDK for Appwrite, an open-source backend-as-a-service platform that provides developers with all the core APIs required to build any application.

## Features

- 🔐 **Authentication** - Complete user authentication with multiple methods
- 🗄️ **Databases** - NoSQL database with powerful query capabilities  
- 📁 **Storage** - File storage with image transformations
- ⚡ **Functions** - Serverless function execution
- 📧 **Messaging** - Email, SMS, and push notifications
- 👥 **Teams** - User team management
- 🔍 **Real-time** - WebSocket connections for live updates
- 🌍 **Localization** - Multi-language support
- 🏥 **Health** - System health monitoring

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
appwrite = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use appwrite::{Client, Account, OAuthProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Appwrite client
    let client = Client::new()
        .set_endpoint("https://cloud.appwrite.io/v1")?
        .set_project("your-project-id");

    // Create account service
    let account = Account::new(&client);

    // Create a new user
    let user = account.create(
        "unique_user_id",
        "user@example.com", 
        "securepassword",
        Some("John Doe")
    ).await?;

    println!("User created: {:?}", user);

    // Create email/password session
    let session = account.create_email_password_session(
        "user@example.com",
        "securepassword"
    ).await?;

    println!("Session created: {:?}", session);

    Ok(())
}
```

## Database Operations

```rust
use appwrite::{Client, Databases, Query, Permission, Role};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()
        .set_endpoint("https://cloud.appwrite.io/v1")?
        .set_project("your-project-id")
        .set_key("your-api-key");

    let databases = Databases::new(&client);

    // Create a document
    let document = databases.create_document(
        "database_id",
        "collection_id", 
        "document_id",
        json!({
            "name": "John Doe",
            "email": "john@example.com",
            "age": 30
        }),
        Some(vec![
            Permission::read(Role::any()),
            Permission::update(Role::user("user_id")),
        ])
    ).await?;

    // Query documents
    let documents = databases.list_documents(
        "database_id",
        "collection_id",
        Some(vec![
            Query::equal("email", "john@example.com"),
            Query::greater_than("age", 18),
            Query::limit(10),
        ])
    ).await?;

    println!("Documents: {:?}", documents);
    Ok(())
}
```

## File Storage

```rust
use appwrite::{Client, Storage, InputFile, Permission, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()
        .set_endpoint("https://cloud.appwrite.io/v1")?
        .set_project("your-project-id")
        .set_key("your-api-key");

    let storage = Storage::new(&client);

    // Upload a file from path
    let file = InputFile::from_path("./image.jpg")?;
    
    let result = storage.create_file(
        "bucket_id",
        "file_id",
        file,
        Some(vec![Permission::read(Role::any())])
    ).await?;

    // Upload from bytes
    let data = b"Hello, Appwrite!";
    let file = InputFile::from_bytes(
        data.to_vec(), 
        "hello.txt",
        Some("text/plain".to_string())
    );

    let result = storage.create_file(
        "bucket_id", 
        "text_file_id",
        file,
        None
    ).await?;

    println!("File uploaded: {:?}", result);
    Ok(())
}
```

## Configuration

The client can be configured with various options:

```rust
use appwrite::Client;

let client = Client::new()
    .set_endpoint("https://your-appwrite-endpoint.com/v1")?
    .set_project("your-project-id")
    .set_key("your-api-key")              // For server-side
    .set_session("user-session-token")    // For client-side
    .set_locale("en-US")
    .set_self_signed(true);               // Allow self-signed certificates
```

## Error Handling

The SDK uses the `thiserror` crate for structured error handling:

```rust
use appwrite::{AppwriteError, Result};

async fn example() -> Result<()> {
    match account.get().await {
        Ok(user) => println!("User: {:?}", user),
        Err(AppwriteError::Api { message, code, .. }) => {
            println!("API Error {}: {}", code, message);
        }
        Err(e) => println!("Other error: {}", e),
    }
    Ok(())
}
```

## Query Builder

Build complex database queries using the query builder:

```rust
use appwrite::Query;

let queries = vec![
    // Basic queries
    Query::equal("status", "published"),
    Query::not_equal("draft", true),
    Query::greater_than("views", 100),
    Query::less_than_equal("rating", 5),
    
    // Text queries
    Query::starts_with("title", "Hello"),
    Query::search("content", "rust programming"),
    
    // Array queries
    Query::contains("tags", "tutorial"),
    Query::is_not_null("description"),
    
    // Complex queries
    Query::between("created_at", "2023-01-01", "2023-12-31"),
    Query::or_queries(vec![
        Query::equal("category", "tech"),
        Query::equal("category", "programming")
    ]),
    
    // Pagination and ordering
    Query::limit(25),
    Query::offset(50),
    Query::order_desc("created_at"),
    Query::cursor_after("document_id"),
];
```

## Permission System

Manage document and file permissions:

```rust
use appwrite::{Permission, Role};

let permissions = vec![
    // Basic permissions
    Permission::read(Role::any()),                    // Anyone can read
    Permission::write(Role::users()),                 // Any logged-in user can write
    Permission::create(Role::user("user_id")),        // Specific user can create
    Permission::update(Role::team("team_id")),        // Team members can update
    Permission::delete(Role::team_with_role("team_id", "admin")), // Team admins can delete
    
    // Advanced roles
    Permission::read(Role::member("member_id")),      // Team member
    Permission::write(Role::label("premium")),        // Users with premium label
];
```

## Examples

Check out the `/examples` directory for comprehensive examples:

- [`account.rs`](examples/account.rs) - User authentication and account management
- [`databases.rs`](examples/databases.rs) - Database operations and queries  
- [`storage.rs`](examples/storage.rs) - File upload and storage operations

Run examples with:

```bash
# Set environment variables
export APPWRITE_PROJECT_ID="your-project-id"
export APPWRITE_API_KEY="your-api-key"

# Run examples
cargo run --example account
cargo run --example databases  
cargo run --example storage
```

## API Coverage

This SDK covers all major Appwrite services:

- ✅ **Account** - User authentication and management
- ✅ **Databases** - Document database with queries
- ✅ **Storage** - File storage with transformations
- 🚧 **Functions** - Serverless functions (coming soon)
- 🚧 **Messaging** - Email, SMS, push notifications (coming soon)
- 🚧 **Teams** - Team and membership management (coming soon)
- 🚧 **Avatars** - Avatar generation (coming soon)
- 🚧 **Locale** - Localization helpers (coming soon)

## Requirements

- Rust 1.70 or later
- An Appwrite instance (cloud or self-hosted)

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.

## Support

- 📚 [Documentation](https://appwrite.io/docs)
- 💬 [Discord Community](https://appwrite.io/discord)
- 🐛 [Issue Tracker](https://github.com/appwrite/sdk-for-rust/issues)
- 📧 [Email Support](mailto:team@appwrite.io)

## Links

- 🌐 [Appwrite Website](https://appwrite.io)
- 📱 [Other SDKs](https://appwrite.io/docs/sdks)
- 🎓 [Tutorials](https://appwrite.io/docs/tutorials)
- 📖 [API Reference](https://appwrite.io/docs/references)

---

Built with ❤️ by the Appwrite team and contributors.