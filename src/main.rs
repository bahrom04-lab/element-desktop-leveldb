use anyhow::{anyhow, Result};
use rusty_leveldb::{LdbIterator, Options, DB};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

/// Element Desktop LevelDB metadata types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementMetadata {
    /// User ID and profile information
    pub user_id: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,

    /// Account settings
    pub theme: Option<String>,
    pub language: Option<String>,
    pub notifications_enabled: Option<bool>,

    /// Rooms and messages
    pub room_ids: Vec<String>,
    pub encrypted_rooms: Vec<String>,

    /// Device and encryption
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub curve25519_key: Option<String>,
    pub ed25519_key: Option<String>,

    /// Raw metadata entries
    pub raw_entries: std::collections::HashMap<String, String>,
}

impl Default for ElementMetadata {
    fn default() -> Self {
        Self {
            user_id: None,
            display_name: None,
            avatar_url: None,
            theme: None,
            language: None,
            notifications_enabled: None,
            room_ids: Vec::new(),
            encrypted_rooms: Vec::new(),
            device_id: None,
            device_name: None,
            curve25519_key: None,
            ed25519_key: None,
            raw_entries: std::collections::HashMap::new(),
        }
    }
}

/// Parses Element Desktop LevelDB for metadata
pub struct ElementLevelDBParser {
    database: Mutex<DB>,
}

impl ElementLevelDBParser {
    /// Opens Element's LevelDB database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = DB::open(path.as_ref(), Options::default())?;
        Ok(ElementLevelDBParser {
            database: Mutex::new(db),
        })
    }

    /// Extracts metadata from the LevelDB database
    pub fn parse_metadata(&self) -> Result<ElementMetadata> {
        let mut metadata = ElementMetadata::default();
        let mut db = self
            .database
            .lock()
            .map_err(|e| anyhow!("Failed to lock database: {}", e))?;

        // Iterate through all entries in the database
        let mut iter = db.new_iter()?;
        iter.seek_to_first();

        while iter.valid() {
            if let Some((key, value)) = iter.current() {
                let key_str = match String::from_utf8(key.to_vec()) {
                    Ok(s) => s,
                    Err(_) => {
                        iter.advance();
                        continue;
                    }
                };

                let value_str = match String::from_utf8(value.to_vec()) {
                    Ok(s) => s,
                    Err(_) => {
                        // Store binary data as hex if not UTF-8
                        let hex_value = hex::encode(&value);
                        metadata
                            .raw_entries
                            .insert(key_str.clone(), format!("0x{}", hex_value));
                        iter.advance();
                        continue;
                    }
                };

                // Parse Element-specific keys
                self.parse_key_value(&key_str, &value_str, &mut metadata);
                metadata.raw_entries.insert(key_str, value_str);
            }

            iter.advance();
        }

        Ok(metadata)
    }

    /// Parses individual key-value pairs for Element metadata
    fn parse_key_value(&self, key: &str, value: &str, metadata: &mut ElementMetadata) {
        // Clean LevelDB control characters
        let clean_value = value.trim_start_matches('\u{0001}').to_string();

        match key {
            // User information
            k if k.contains("user_id") || k.contains("userId") || k.contains("mx_user_id") => {
                metadata.user_id = Some(clean_value);
            }
            k if k.contains("display_name")
                || k.contains("displayName")
                || k.contains("displayname") =>
            {
                metadata.display_name = Some(clean_value);
            }
            k if k.contains("avatar") || k.contains("avatarUrl") || k.contains("avatar_url") => {
                metadata.avatar_url = Some(clean_value);
            }

            // Settings
            k if k.contains("theme") => {
                metadata.theme = Some(clean_value);
            }
            k if k.contains("language") || k.contains("locale") => {
                metadata.language = Some(clean_value);
            }
            k if k.contains("notification") => {
                metadata.notifications_enabled = Some(clean_value.to_lowercase() == "true");
            }

            // Device and encryption keys
            k if k.contains("device_id")
                || k.contains("deviceId")
                || k.contains("mx_device_id") =>
            {
                metadata.device_id = Some(clean_value);
            }
            k if k.contains("device_name") || k.contains("deviceName") => {
                metadata.device_name = Some(clean_value);
            }
            k if k.contains("curve25519") => {
                metadata.curve25519_key = Some(clean_value);
            }
            k if k.contains("ed25519") => {
                metadata.ed25519_key = Some(clean_value);
            }

            // Room information
            k if k.contains("room") && k.contains("id") => {
                metadata.room_ids.push(clean_value);
            }
            k if k.contains("encrypted") => {
                if clean_value.to_lowercase() == "true" {
                    metadata.encrypted_rooms.push(key.to_string());
                }
            }

            _ => {}
        }
    }

    /// Exports metadata as JSON
    pub fn to_json(&self) -> Result<String> {
        let metadata = self.parse_metadata()?;
        Ok(serde_json::to_string_pretty(&metadata)?)
    }

    /// Gets a single value by key
    pub fn get_value(&self, key: &str) -> Result<Option<String>> {
        let mut db = self
            .database
            .lock()
            .map_err(|e| anyhow!("Failed to lock database: {}", e))?;
        match db.get(key.as_bytes()) {
            Some(data) => {
                let value = String::from_utf8_lossy(&data).to_string();
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}

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
