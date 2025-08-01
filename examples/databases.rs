//! Database service example demonstrating CRUD operations

use appwrite::{Client, Databases, Query, Permission, Role};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Appwrite client
    let client = Client::new()
        .set_endpoint("https://cloud.appwrite.io/v1")?
        .set_project(env::var("APPWRITE_PROJECT_ID").expect("APPWRITE_PROJECT_ID not set"))
        .set_key(env::var("APPWRITE_API_KEY").expect("APPWRITE_API_KEY not set"));

    let databases = Databases::new(&client);

    println!("🗄️ Appwrite Databases Example");
    println!("=============================");

    // Example 1: List all databases
    println!("\n📋 Listing all databases...");
    match databases.list(None, None).await {
        Ok(db_list) => println!("✅ Databases: {}", db_list),
        Err(e) => println!("❌ Failed to list databases: {}", e),
    }

    // Example 2: Create a new database
    println!("\n📝 Creating a new database...");
    let database_id = "example_db_123";
    match databases.create(database_id, "Example Database", Some(true)).await {
        Ok(database) => {
            println!("✅ Database created: {}", database);

            // Example 3: Create a collection in the database
            println!("\n📂 Creating a collection...");
            let collection_id = "users_collection";
            match databases.create_collection(
                database_id,
                collection_id,
                "Users",
                Some(vec![
                    Permission::read(Role::any()),
                    Permission::create(Role::users()),
                    Permission::update(Role::user("self")),
                    Permission::delete(Role::user("self")),
                ]),
                Some(false), // document security
                Some(true),  // enabled
            ).await {
                Ok(collection) => {
                    println!("✅ Collection created: {}", collection);

                    // Example 4: Create attributes for the collection
                    println!("\n🏷️ Creating string attribute...");
                    match databases.create_string_attribute(
                        database_id,
                        collection_id,
                        "name",
                        255,     // size
                        true,    // required
                        None,    // default
                        Some(false), // array
                        Some(false), // encrypt
                    ).await {
                        Ok(attr) => println!("✅ String attribute created: {}", attr),
                        Err(e) => println!("❌ Failed to create string attribute: {}", e),
                    }

                    println!("\n📧 Creating email attribute...");
                    match databases.create_email_attribute(
                        database_id,
                        collection_id,
                        "email",
                        true,    // required
                        None,    // default
                        Some(false), // array
                    ).await {
                        Ok(attr) => println!("✅ Email attribute created: {}", attr),
                        Err(e) => println!("❌ Failed to create email attribute: {}", e),
                    }

                    println!("\n🔢 Creating integer attribute...");
                    match databases.create_integer_attribute(
                        database_id,
                        collection_id,
                        "age",
                        false,   // required
                        Some(0), // min
                        Some(150), // max
                        Some(18), // default
                        Some(false), // array
                    ).await {
                        Ok(attr) => println!("✅ Integer attribute created: {}", attr),
                        Err(e) => println!("❌ Failed to create integer attribute: {}", e),
                    }

                    // Wait a moment for attributes to be processed
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                    // Example 5: Create a document
                    println!("\n📄 Creating a document...");
                    let document_data = serde_json::json!({
                        "name": "John Doe",
                        "email": "john@example.com",
                        "age": 30
                    });

                    match databases.create_document(
                        database_id,
                        collection_id,
                        "user_doc_123",
                        document_data,
                        Some(vec![
                            Permission::read(Role::any()),
                            Permission::update(Role::user("user_123")),
                            Permission::delete(Role::user("user_123")),
                        ]),
                    ).await {
                        Ok(document) => {
                            println!("✅ Document created: {}", document);

                            // Example 6: Get the document
                            println!("\n📖 Getting the document...");
                            match databases.get_document(database_id, collection_id, "user_doc_123", None).await {
                                Ok(doc) => println!("✅ Retrieved document: {}", doc),
                                Err(e) => println!("❌ Failed to get document: {}", e),
                            }

                            // Example 7: Update the document
                            println!("\n✏️ Updating the document...");
                            let update_data = serde_json::json!({
                                "name": "John Smith",
                                "age": 31
                            });

                            match databases.update_document(
                                database_id,
                                collection_id,
                                "user_doc_123",
                                Some(update_data),
                                None, // permissions
                            ).await {
                                Ok(doc) => println!("✅ Document updated: {}", doc),
                                Err(e) => println!("❌ Failed to update document: {}", e),
                            }

                            // Example 8: List documents with queries
                            println!("\n📋 Listing documents with queries...");
                            let queries = vec![
                                Query::equal("email", "john@example.com"),
                                Query::greater_than("age", 25),
                                Query::limit(10),
                                Query::order_desc("$createdAt"),
                            ];

                            match databases.list_documents(
                                database_id,
                                collection_id,
                                Some(queries),
                            ).await {
                                Ok(docs) => println!("✅ Documents found: {}", docs),
                                Err(e) => println!("❌ Failed to list documents: {}", e),
                            }

                            // Example 9: Search documents
                            println!("\n🔍 Searching documents...");
                            let search_queries = vec![
                                Query::search("name", "John"),
                                Query::limit(5),
                            ];

                            match databases.list_documents(
                                database_id,
                                collection_id,
                                Some(search_queries),
                            ).await {
                                Ok(docs) => println!("✅ Search results: {}", docs),
                                Err(e) => println!("❌ Failed to search documents: {}", e),
                            }

                            // Example 10: Create an index
                            println!("\n🗂️ Creating an index...");
                            match databases.create_index(
                                database_id,
                                collection_id,
                                "email_index",
                                appwrite::IndexType::Key,
                                vec!["email".to_string()],
                                None, // orders
                            ).await {
                                Ok(index) => println!("✅ Index created: {}", index),
                                Err(e) => println!("❌ Failed to create index: {}", e),
                            }
                        }
                        Err(e) => println!("❌ Failed to create document: {}", e),
                    }
                }
                Err(e) => println!("❌ Failed to create collection: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to create database: {}", e),
    }

    // Example 11: Demonstrate complex queries
    println!("\n🔍 Complex query examples...");
    let complex_queries = vec![
        Query::and_queries(vec![
            Query::greater_than("age", 18),
            Query::less_than("age", 65),
        ]),
        Query::or_queries(vec![
            Query::equal("status", "active"),
            Query::equal("status", "pending"),
        ]),
        Query::between("age", 25, 35),
        Query::starts_with("name", "John"),
        Query::is_not_null("email"),
        Query::limit(20),
        Query::offset(0),
    ];

    println!("📊 Query examples:");
    for (i, query) in complex_queries.iter().enumerate() {
        println!("  {}. {}", i + 1, query);
    }

    println!("\n🎉 Databases example completed!");
    Ok(())
}

// Helper to create a test database
async fn ensure_test_database(
    databases: &Databases<'_>,
    database_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Try to get the database first
    match databases.get(database_id).await {
        Ok(db) => Ok(db),
        Err(_) => {
            // Database doesn't exist, create it
            databases.create(database_id, "Test Database", Some(true)).await
                .map_err(|e| e.into())
        }
    }
}