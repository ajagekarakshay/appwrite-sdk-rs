//! Storage service example demonstrating file operations

use appwrite::{Client, Storage, InputFile, Permission, Role, Compression, ImageGravity, ImageFormat};
use std::env;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Appwrite client
    let client = Client::new()
        .set_endpoint("https://cloud.appwrite.io/v1")?
        .set_project(env::var("APPWRITE_PROJECT_ID").expect("APPWRITE_PROJECT_ID not set"))
        .set_key(env::var("APPWRITE_API_KEY").expect("APPWRITE_API_KEY not set"));

    let storage = Storage::new(&client);

    println!("📁 Appwrite Storage Example");
    println!("===========================");

    // Example 1: List all buckets
    println!("\n📋 Listing all storage buckets...");
    match storage.list_buckets(None, None).await {
        Ok(buckets) => println!("✅ Buckets: {}", buckets),
        Err(e) => println!("❌ Failed to list buckets: {}", e),
    }

    // Example 2: Create a new bucket
    println!("\n🪣 Creating a new storage bucket...");
    let bucket_id = "example_bucket_123";
    match storage.create_bucket(
        bucket_id,
        "Example Bucket",
        Some(vec![
            Permission::read(Role::any()),
            Permission::create(Role::users()),
            Permission::update(Role::users()),
            Permission::delete(Role::users()),
        ]),
        Some(false), // file security
        Some(true),  // enabled
        Some(10 * 1024 * 1024), // 10MB max file size
        Some(vec!["jpg".to_string(), "png".to_string(), "gif".to_string(), "pdf".to_string()]), // allowed file extensions
        Some(Compression::None), // compression
        Some(false), // encryption
        Some(false), // antivirus
    ).await {
        Ok(bucket) => {
            println!("✅ Bucket created: {}", bucket);

            // Example 3: Create a test file to upload
            println!("\n📝 Creating a test file...");
            let test_content = b"Hello, Appwrite! This is a test file for the Storage example.";
            let temp_file_path = "temp_test_file.txt";
            
            {
                let mut file = std::fs::File::create(temp_file_path)?;
                file.write_all(test_content)?;
            }

            // Example 4: Upload file from path
            println!("\n⬆️ Uploading file from path...");
            let input_file = InputFile::from_path(temp_file_path)?;
            
            match storage.create_file(
                bucket_id,
                "test_file_123",
                input_file,
                Some(vec![
                    Permission::read(Role::any()),
                    Permission::update(Role::user("user_123")),
                    Permission::delete(Role::user("user_123")),
                ]),
            ).await {
                Ok(file) => {
                    println!("✅ File uploaded from path: {}", file);

                    // Example 5: Get file details
                    println!("\n📖 Getting file details...");
                    match storage.get_file(bucket_id, "test_file_123").await {
                        Ok(file_info) => println!("✅ File details: {}", file_info),
                        Err(e) => println!("❌ Failed to get file: {}", e),
                    }

                    // Example 6: Get file for download
                    println!("\n⬇️ Getting file download URL...");
                    match storage.get_file_download(bucket_id, "test_file_123").await {
                        Ok(download_url) => println!("✅ Download URL: {}", download_url),
                        Err(e) => println!("❌ Failed to get download URL: {}", e),
                    }

                    // Example 7: Get file preview (for supported formats)
                    println!("\n👁️ Getting file preview URL...");
                    match storage.get_file_preview(
                        bucket_id,
                        "test_file_123",
                        Some(300), // width
                        Some(300), // height
                        Some(ImageGravity::Center),
                        Some(80), // quality
                        Some(0),  // border width
                        None,     // border color
                        Some(0),  // border radius
                        Some(1.0), // opacity
                        Some(0),  // rotation
                        None,     // background
                        Some(ImageFormat::Png),
                    ).await {
                        Ok(preview_url) => println!("✅ Preview URL: {}", preview_url),
                        Err(e) => println!("❌ Failed to get preview URL: {}", e),
                    }

                    // Example 8: Update file
                    println!("\n✏️ Updating file metadata...");
                    match storage.update_file(
                        bucket_id,
                        "test_file_123",
                        Some("updated_test_file.txt"),
                        Some(vec![
                            Permission::read(Role::users()),
                            Permission::update(Role::user("user_123")),
                            Permission::delete(Role::user("user_123")),
                        ]),
                    ).await {
                        Ok(updated_file) => println!("✅ File updated: {}", updated_file),
                        Err(e) => println!("❌ Failed to update file: {}", e),
                    }
                }
                Err(e) => println!("❌ Failed to upload file: {}", e),
            }

            // Example 9: Upload file from bytes
            println!("\n⬆️ Uploading file from bytes...");
            let bytes_content = b"This is another test file created from byte data!";
            let input_file_bytes = InputFile::from_bytes(
                bytes_content.to_vec(),
                "bytes_test.txt",
                Some("text/plain".to_string()),
            );

            match storage.create_file(
                bucket_id,
                "bytes_file_123",
                input_file_bytes,
                Some(vec![Permission::read(Role::any())]),
            ).await {
                Ok(file) => println!("✅ File uploaded from bytes: {}", file),
                Err(e) => println!("❌ Failed to upload file from bytes: {}", e),
            }

            // Example 10: List files in bucket
            println!("\n📋 Listing files in bucket...");
            match storage.list_files(
                bucket_id,
                None, // queries
                None, // search
            ).await {
                Ok(files) => println!("✅ Files in bucket: {}", files),
                Err(e) => println!("❌ Failed to list files: {}", e),
            }

            // Example 11: Get bucket details
            println!("\n🪣 Getting bucket details...");
            match storage.get_bucket(bucket_id).await {
                Ok(bucket_info) => println!("✅ Bucket details: {}", bucket_info),
                Err(e) => println!("❌ Failed to get bucket: {}", e),
            }

            // Example 12: Update bucket
            println!("\n🔧 Updating bucket...");
            match storage.update_bucket(
                bucket_id,
                "Updated Example Bucket",
                Some(vec![Permission::read(Role::any())]),
                Some(false), // file security
                Some(true),  // enabled
                Some(20 * 1024 * 1024), // 20MB max file size
                Some(vec!["jpg".to_string(), "png".to_string(), "gif".to_string(), "pdf".to_string(), "txt".to_string()]),
                Some(Compression::Gzip),
                Some(false), // encryption
                Some(true),  // antivirus
            ).await {
                Ok(updated_bucket) => println!("✅ Bucket updated: {}", updated_bucket),
                Err(e) => println!("❌ Failed to update bucket: {}", e),
            }

            // Cleanup: Remove temporary file
            if std::fs::remove_file(temp_file_path).is_err() {
                println!("⚠️ Could not remove temporary file: {}", temp_file_path);
            }
        }
        Err(e) => println!("❌ Failed to create bucket: {}", e),
    }

    // Example 13: Demonstrate file upload with progress tracking
    println!("\n📊 Example of chunked upload (for large files)...");
    let large_content = vec![b'A'; 1024 * 1024]; // 1MB of 'A' characters
    let large_file_input = InputFile::from_bytes(
        large_content,
        "large_test_file.txt",
        Some("text/plain".to_string()),
    );

    match storage.create_file(
        bucket_id,
        "large_file_123",
        large_file_input,
        Some(vec![Permission::read(Role::any())]),
    ).await {
        Ok(file) => println!("✅ Large file uploaded: {}", file),
        Err(e) => println!("❌ Failed to upload large file: {}", e),
    }

    println!("\n🎉 Storage example completed!");
    Ok(())
}

// Helper function to create test image file
fn create_test_image() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Create a simple test "image" (actually just some bytes with PNG header-like data)
    let mut image_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
    ];
    
    // Add some dummy data
    image_data.extend_from_slice(b"This is a fake PNG file for testing purposes");
    
    Ok(image_data)
}