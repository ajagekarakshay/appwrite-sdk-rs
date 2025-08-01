# Contributing to Appwrite Rust SDK

Thank you for your interest in contributing to the Appwrite Rust SDK! We welcome contributions from developers of all experience levels.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- An Appwrite instance (cloud or self-hosted) for testing

### Setting up the development environment

1. Fork and clone the repository:
```bash
git clone https://github.com/your-username/sdk-for-rust.git
cd sdk-for-rust
```

2. Install dependencies:
```bash
cargo build
```

3. Set up environment variables for testing:
```bash
export APPWRITE_PROJECT_ID="your-project-id"
export APPWRITE_API_KEY="your-api-key"
export APPWRITE_ENDPOINT="https://cloud.appwrite.io/v1"
```

## Code Structure

The SDK is organized as follows:

```
src/
├── lib.rs              # Main library entry point
├── client.rs           # HTTP client implementation
├── error.rs            # Error types and handling
├── query.rs            # Database query builder
├── permission.rs       # Permission helpers
├── input_file.rs       # File upload utilities
├── enums/              # Enum definitions
│   ├── mod.rs
│   ├── oauth_provider.rs
│   └── ...
└── services/           # API service implementations
    ├── mod.rs
    ├── account.rs      # User authentication
    ├── databases.rs    # Database operations
    ├── storage.rs      # File storage
    └── ...
```

## Development Guidelines

### Code Style

- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Follow Rust naming conventions
- Add documentation comments for public APIs
- Include examples in documentation

### Error Handling

- Use the `Result<T>` type alias from `crate::error`
- Create specific error types using `thiserror`
- Provide meaningful error messages

### API Consistency

- Follow the patterns established in existing services
- Use async/await for all API calls
- Accept borrowed types (`&str`) for parameters when possible
- Return `serde_json::Value` for API responses

### Testing

```bash
# Run tests
cargo test

# Run with all features
cargo test --all-features

# Run examples
cargo run --example account
cargo run --example databases
cargo run --example storage
```

## Adding New Services

To add a new service:

1. Create a new file in `src/services/`
2. Follow this template:

```rust
//! Service description

use crate::{client::Client, error::Result};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Service documentation
#[derive(Debug, Clone)]
pub struct ServiceName<'a> {
    client: &'a Client,
}

impl<'a> ServiceName<'a> {
    /// Create a new service instance
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Method documentation
    pub async fn method_name(&self, param: &str) -> Result<Value> {
        // Implementation
        self.client
            .call("get", "/api/path", None, None, None)
            .await
    }
}
```

3. Add the service to `src/services/mod.rs`
4. Add re-exports to `src/lib.rs`
5. Create examples and documentation

## Adding New Enums

1. Create a new file in `src/enums/`
2. Follow this template:

```rust
//! Enum description

use serde::{Deserialize, Serialize};

/// Enum documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnumName {
    #[serde(rename = "value1")]
    Value1,
    #[serde(rename = "value2")]
    Value2,
}

impl AsRef<str> for EnumName {
    fn as_ref(&self) -> &str {
        match self {
            EnumName::Value1 => "value1",
            EnumName::Value2 => "value2",
        }
    }
}
```

3. Add to `src/enums/mod.rs`
4. Add re-export to `src/lib.rs`

## Documentation

- Write comprehensive API documentation
- Include usage examples
- Keep README.md updated
- Document breaking changes in commit messages

## Submitting Changes

### Pull Request Process

1. Create a feature branch from `main`:
```bash
git checkout -b feature/my-new-feature
```

2. Make your changes and commit them:
```bash
git add .
git commit -m "feat: add new feature"
```

3. Ensure tests pass:
```bash
cargo test
cargo clippy
cargo fmt
```

4. Push to your fork:
```bash
git push origin feature/my-new-feature
```

5. Create a Pull Request with:
   - Clear title and description
   - Reference any related issues
   - Include examples of usage
   - List any breaking changes

### Commit Message Format

Use conventional commits:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding tests
- `chore:` - Maintenance tasks

## Release Process

Releases are managed by the Appwrite team:

1. Version bumps follow semantic versioning
2. Changelog is automatically generated
3. Releases are published to crates.io

## Getting Help

- 💬 [Discord Community](https://appwrite.io/discord)
- 🐛 [Issue Tracker](https://github.com/appwrite/sdk-for-rust/issues)
- 📧 [Email Support](mailto:team@appwrite.io)

## Code of Conduct

Please read and follow our [Code of Conduct](https://github.com/appwrite/appwrite/blob/main/CODE_OF_CONDUCT.md).

## License

By contributing to this project, you agree that your contributions will be licensed under the BSD 3-Clause License.