use anyhow::{Result, anyhow};
use rusty_leveldb::{DB, LdbIterator, Options};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

const USER_IDS: &[&str] = &["user_id", "userId", "mx_user_id"];
const DISPLAY_NAME: &[&str] = &["display_name", "displayName", "displayname"];
const AVATAR_URL: &[&str] = &["avatar", "avatarUrl"];
const THEME: &[&str] = &["theme"];
const LANGUAGE: &[&str] = &["language", "locale"];
const NOTIFICATION: &[&str] = &["notification"];
const DEVICE_ID: &[&str] = &["device_id", "deviceId", "mx_device_id"];
const DEVICE_NAME: &[&str] = &["device_name", "deviceName"];
const CURVE25519: &[&str] = &["curve25519"];
const ED25519: &[&str] = &["ed25519"];
const ROOM: &[&str] = &["room", "id"];
const ENCRYPTED: &[&str] = &["encrypted"];

fn contains_any(key: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| key.contains(p))
}

/// Element Desktop LevelDB metadata types
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ElementMetadata {
    pub user_id: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,

    pub theme: Option<String>,
    pub language: Option<String>,
    pub notifications_enabled: Option<bool>,

    pub room_ids: Vec<String>,
    pub encrypted_rooms: Vec<String>,

    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub curve25519_key: Option<String>,
    pub ed25519_key: Option<String>,

    /// Raw metadata entries
    pub raw_entries: std::collections::HashMap<String, String>,
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

        if contains_any(key, USER_IDS) {
            metadata.user_id = Some(clean_value)
        } else if contains_any(key, DISPLAY_NAME) {
            metadata.display_name = Some(clean_value);
        } else if contains_any(key, AVATAR_URL) {
            metadata.avatar_url = Some(clean_value);
        } else if contains_any(key, THEME) {
            metadata.theme = Some(clean_value);
        } else if contains_any(key, LANGUAGE) {
            metadata.language = Some(clean_value);
        } else if contains_any(key, NOTIFICATION) {
            metadata.notifications_enabled = Some(clean_value.parse::<bool>().unwrap_or_default());
        } else if contains_any(key, DEVICE_ID) {
            metadata.device_id = Some(clean_value);
        } else if contains_any(key, DEVICE_NAME) {
            metadata.device_name = Some(clean_value);
        } else if contains_any(key, CURVE25519) {
            metadata.curve25519_key = Some(clean_value);
        } else if contains_any(key, ED25519) {
            metadata.ed25519_key = Some(clean_value);
        } else if contains_any(key, ROOM) {
            metadata.room_ids.push(clean_value);
        } else if contains_any(key, ENCRYPTED)
            && clean_value.to_lowercase().parse::<bool>().unwrap()
        {
            metadata.room_ids.push(key.to_string());
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
