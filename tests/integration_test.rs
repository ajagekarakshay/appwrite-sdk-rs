//! Integration tests for the Appwrite Rust SDK

use appwrite::{Client, Account, Databases, Storage, Query, Permission, Role, OAuthProvider};

#[tokio::test]
async fn test_client_creation() {
    let client = Client::new()
        .set_endpoint("https://example.com/v1")
        .expect("Valid endpoint")
        .set_project("test-project")
        .set_key("test-key");

    // Test service instantiation
    let _account = Account::new(&client);
    let _databases = Databases::new(&client);
    let _storage = Storage::new(&client);
}

#[test]
fn test_query_builder() {
    let query = Query::equal("status", "active");
    assert!(query.contains("equal"));
    assert!(query.contains("status"));
    assert!(query.contains("active"));

    let complex_query = Query::and_queries(vec![
        Query::greater_than("age", 18),
        Query::less_than("age", 65),
    ]);
    assert!(complex_query.contains("and"));
}

#[test]
fn test_permissions() {
    let read_perm = Permission::read(Role::any());
    assert_eq!(read_perm, r#"read("any")"#);

    let write_perm = Permission::write(Role::user("123"));
    assert_eq!(write_perm, r#"write("user:123")"#);

    let team_perm = Permission::update(Role::team("team-123"));
    assert_eq!(team_perm, r#"update("team:team-123")"#);
}

#[test]
fn test_roles() {
    assert_eq!(Role::any(), "any");
    assert_eq!(Role::users(), "users");
    assert_eq!(Role::guests(), "guests");
    assert_eq!(Role::user("123"), "user:123");
    assert_eq!(Role::team("team-123"), "team:team-123");
    assert_eq!(Role::team_with_role("team-123", "admin"), "team:team-123:admin");
}

#[test]
fn test_oauth_providers() {
    assert_eq!(OAuthProvider::Google.as_ref(), "google");
    assert_eq!(OAuthProvider::Github.as_ref(), "github");
    assert_eq!(OAuthProvider::Apple.as_ref(), "apple");
}

#[test]
fn test_error_types() {
    use appwrite::AppwriteError;

    let api_error = AppwriteError::api("Test error", 400);
    assert_eq!(api_error.code(), Some(400));

    let missing_param_error = AppwriteError::missing_parameter("user_id");
    assert!(missing_param_error.to_string().contains("user_id"));
}