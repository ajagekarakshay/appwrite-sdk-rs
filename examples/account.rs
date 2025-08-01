//! Account service example demonstrating user authentication

use appwrite::{Account, Client, OAuthProvider};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Appwrite client
    let client = Client::new()
        .set_endpoint("https://cloud.appwrite.io/v1")?
        .set_project(env::var("APPWRITE_PROJECT_ID").expect("APPWRITE_PROJECT_ID not set"));

    let account = Account::new(&client);

    println!("🚀 Appwrite Account Example");
    println!("==========================");

    // Example 1: Create a new user account
    println!("\n📝 Creating a new user account...");
    match account.create(
        "unique_user_123",
        "user@example.com",
        "securepassword123",
        Some("John Doe"),
    ).await {
        Ok(user) => println!("✅ User created successfully: {}", user),
        Err(e) => println!("❌ Failed to create user: {}", e),
    }

    // Example 2: Create email/password session
    println!("\n🔐 Creating email/password session...");
    match account.create_email_password_session("user@example.com", "securepassword123").await {
        Ok(session) => {
            println!("✅ Session created successfully: {}", session);
            
            // Now that we're logged in, we can get user info
            println!("\n👤 Getting current user info...");
            match account.get().await {
                Ok(user) => println!("✅ Current user: {}", user),
                Err(e) => println!("❌ Failed to get user: {}", e),
            }

            // Example 3: Update user name
            println!("\n✏️ Updating user name...");
            match account.update_name("Jane Doe").await {
                Ok(user) => println!("✅ Name updated successfully: {}", user),
                Err(e) => println!("❌ Failed to update name: {}", e),
            }

            // Example 4: Get user preferences
            println!("\n⚙️ Getting user preferences...");
            match account.get_prefs().await {
                Ok(prefs) => println!("✅ User preferences: {}", prefs),
                Err(e) => println!("❌ Failed to get preferences: {}", e),
            }

            // Example 5: Update user preferences
            println!("\n🔧 Updating user preferences...");
            let new_prefs = serde_json::json!({
                "theme": "dark",
                "language": "en",
                "notifications": true
            });
            match account.update_prefs(new_prefs).await {
                Ok(prefs) => println!("✅ Preferences updated: {}", prefs),
                Err(e) => println!("❌ Failed to update preferences: {}", e),
            }

            // Example 6: List all sessions
            println!("\n📋 Listing all sessions...");
            match account.list_sessions().await {
                Ok(sessions) => println!("✅ User sessions: {}", sessions),
                Err(e) => println!("❌ Failed to list sessions: {}", e),
            }

            // Example 7: Create email verification
            println!("\n📧 Creating email verification...");
            match account.create_verification("https://example.com/verify").await {
                Ok(verification) => println!("✅ Verification created: {}", verification),
                Err(e) => println!("❌ Failed to create verification: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to create session: {}", e),
    }

    // Example 8: Create anonymous session
    println!("\n👤 Creating anonymous session...");
    match account.create_anonymous_session().await {
        Ok(session) => println!("✅ Anonymous session created: {}", session),
        Err(e) => println!("❌ Failed to create anonymous session: {}", e),
    }

    // Example 9: OAuth2 session (returns redirect URL)
    println!("\n🔗 Creating OAuth2 session URL...");
    match account.create_oauth2_session(
        OAuthProvider::Google,
        Some("https://example.com/success"),
        Some("https://example.com/failure"),
        Some(vec!["email".to_string(), "profile".to_string()]),
    ).await {
        Ok(redirect_url) => println!("✅ OAuth2 redirect URL: {}", redirect_url),
        Err(e) => println!("❌ Failed to create OAuth2 session: {}", e),
    }

    // Example 10: Password recovery
    println!("\n🔄 Creating password recovery...");
    match account.create_recovery("user@example.com", "https://example.com/recovery").await {
        Ok(recovery) => println!("✅ Recovery created: {}", recovery),
        Err(e) => println!("❌ Failed to create recovery: {}", e),
    }

    println!("\n🎉 Account example completed!");
    Ok(())
}

// Helper function to demonstrate error handling
async fn safe_account_operation<F, Fut>(operation_name: &str, operation: F)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<serde_json::Value, appwrite::AppwriteError>>,
{
    println!("\n🔄 {}", operation_name);
    match operation().await {
        Ok(result) => println!("✅ Success: {}", result),
        Err(e) => {
            if let Some(code) = e.code() {
                println!("❌ API Error ({}): {}", code, e);
            } else {
                println!("❌ Error: {}", e);
            }
        }
    }
}