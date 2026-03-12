use anyhow::Result;
use element_metadata::{ElementLevelDBParser, ElementMetadata};
use std::path::Path;

fn main() -> Result<()> {
    println!("Element Desktop LevelDB Metadata Parser");
    println!("========================================\n");

    // Example usage - user would provide their Element LevelDB path
    let example_path = "./leveldb";

    println!("To use this parser:");
    println!("1. Locate your Element LevelDB database:");
    println!("   - Windows: %APPDATA%\\Element\\Local Storage\\leveldb");
    println!("   - Linux: ~/.config/Element/Local Storage/leveldb");
    println!("   - macOS: ~/Library/Application Support/Element/Local Storage/leveldb");
    println!("\n2. Provide the path to the parser\n");

    // Check if example path exists
    if Path::new(example_path).exists() {
        match ElementLevelDBParser::open(example_path) {
            Ok(parser) => {
                println!("✓ Successfully opened LevelDB database");

                match parser.to_json() {
                    Ok(json) => {
                        println!("\nExtracted Metadata (JSON):");
                        println!("{}", json);
                    }
                    Err(e) => eprintln!("Error parsing metadata: {}", e),
                }
            }
            Err(e) => eprintln!("Error opening database: {}", e),
        }
    } else {
        println!("Note: Example LevelDB path not found at '{}'", example_path);
        println!("This is expected for demonstration purposes.");

        // Show the data structures
        let example_metadata = ElementMetadata {
            user_id: Some("@user:example.com".to_string()),
            display_name: Some("Test User".to_string()),
            avatar_url: Some("mxc://example.com/abc123".to_string()),
            theme: Some("dark".to_string()),
            language: Some("en".to_string()),
            notifications_enabled: Some(true),
            room_ids: vec![
                "!room1:example.com".to_string(),
                "!room2:example.com".to_string(),
            ],
            encrypted_rooms: vec!["!encrypted1:example.com".to_string()],
            device_id: Some("GHTYAJCE".to_string()),
            device_name: Some("My Device".to_string()),
            curve25519_key: Some("example_curve_key".to_string()),
            ed25519_key: Some("example_ed_key".to_string()),
            raw_entries: std::collections::HashMap::new(),
        };

        println!("\nExample output structure:");
        println!("{}", serde_json::to_string_pretty(&example_metadata)?);
    }

    Ok(())
}
