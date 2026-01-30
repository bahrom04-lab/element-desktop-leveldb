# Element Desktop LevelDB Metadata Parser - Milestone 4 Early Prototype

## Project Overview

> [!NOTE]
> This project is an **early prototype artifact** for **Milestone 4: 
> Professionalism Section**, implementing a comprehensive LevelDB metadata 
> parser specifically designed for Element Desktop forensic analysis in Rust. 
>
> **Primary Objective**: Extract, parse, and analyze critical metadata from 
> Element Desktop's LevelDB database for digital forensic investigations.


> [!CAUTION]
> ## Evidence Handling & Chain-of-Custody Considerations
>
> This prototype operates exclusively in read-only mode and requires the 
> examiner to work on a copy of the original LevelDB database. The tool 
> does not modify, delete, or write to the source database under any circumstances.
>
> While the current prototype does not yet compute cryptographic hashes of 
> the input database, this functionality is planned for future versions to 
> support full chain-of-custody verification and evidence integrity validation.
> 
---

## Technology Stack & Crate Selection

After comprehensive research comparing available LevelDB Rust crates, This selected **`rusty-leveldb` v4.0.1** as the best maintained and most suitable option:

---

#### 1. User Profile Information
- **User ID**: Matrix user identifier (e.g., `@bahrom04:matrix.org`)
- **Display Name**: User's display name in the client
- **Avatar URL**: MXC URL for user's avatar
- **Device ID**: Unique device identifier
- **OIDC Client ID**: OpenID Connect client identifier

#### 2. Session & Authentication
- **Access Tokens**: Presence and metadata
- **Refresh Tokens**: Token status tracking
- **OIDC ID Tokens**: JWT tokens for identity verification
- **Device Encryption Status**: Crypto initialization state
- **Pickle Key Status**: Encryption key storage status

#### 3. Settings & Preferences
- **Theme**: Dark/light mode preference
- **Language/Locale**: Language settings
- **Notification Preferences**: Notification enabled/disabled
- **Right Panel Configuration**: UI state per room
- **UI State Preferences**: General UI preferences

#### 4. Room & Communication
- **Room IDs**: All rooms in which user participates
- **Encrypted Rooms**: Rooms with encryption enabled
- **Room-Specific Settings**: Per-room configuration
- **Last Accessed Room**: Most recently viewed room
- **Room Widget Configuration**: Widget state per room

#### 5. Security & Encryption
- **Device Name**: Human-readable device name
- **Curve25519 Keys**: Elliptic curve cryptography keys
- **Ed25519 Keys**: Edwards-curve signature keys
- **Encryption Failure Events**: Tracking of decryption failures
- **Crypto Status**: Encryption readiness indicators

#### 6. Raw Metadata Access

---

## Architecture & Implementation

### Data Structures

```rust
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

    /// Raw metadata entries (all extracted data)
    pub raw_entries: std::collections::HashMap<String, String>,
}

/// Parses Element Desktop LevelDB for metadata
pub struct ElementLevelDBParser {
    database: Mutex<DB>,
}
```

### Key Parser Methods

| Method | Purpose | Returns |
|--------|---------|---------|
| `open(path)` | Opens Element's LevelDB database | `Result<ElementLevelDBParser>` |
| `parse_metadata()` | Extracts all metadata into structured format | `Result<ElementMetadata>` |
| `to_json()` | Exports metadata as pretty-printed JSON | `Result<String>` |
| `get_value(key)` | Retrieves single key-value pair | `Result<Option<String>>` |
| `parse_key_value()` | Intelligent pattern matching for Element keys | `()` |

---

## Development, BUilding or Hack

The project has `shell.nix` which has development environment preconfigured already for you. Just open your
terminal and at the root of this project:

```bash
# VSCode
code .

# INstall packages
nix develop # -c $SHELL

# Copy your Element LevelDB to the project directory
./copy-leveldb.sh

# Run the project
cargo run
```

The development environment has whatever you may need already, but feel free to add or remove whatever
inside `shell.nix`.

### Locating Element Desktop LevelDB
```bash
## Windows:
%APPDATA%\Element\Local Storage\leveldb

## Linux:
~/.config/Element/Local Storage/leveldb

##MacOS:
~/Library/Application Support/Element/Local Storage/leveldb
```

---

## Example Output

```json
{
  "user_id": "@prezident:matrix.org",
  "display_name": "AHHHHHH UHHHHHHH",
  "avatar_url": "mxc://matrix.org/assssssssssss",
  "theme": null,
  "language": null,
  "notifications_enabled": null,
  "room_ids": [
    "assssssssddddd"
  ],
  "encrypted_rooms": [],
  "device_id": "asdaaaaa",
  "device_name": null,
  "curve25519_key": null,
  "ed25519_key": null,
  "raw_entries": {
    "VERSION": "1",
    "_vector://vector\u0001mx_user_id": "@prezident:matrix.org",
    "_vector://vector\u0001mx_profile_displayname": "assssss ssssss",
    "_vector://vector\u0001mx_device_id": "assassas",
    "_vector://vector\u0001mx_profile_avatar_url": "mxc://matrix.org/assassas",
    "_vector://vector\u0001mx_hs_url": "https://matrix-client.matrix.org",
    "_vector://vector\u0001mx_is_guest": "false",
    "_vector://vector\u0001mx_crypto_initialised": "true",
    "_vector://vector\u0001mx_has_access_token": "true",
    "_vector://vector\u0001mx_has_refresh_token": "true",
    "_vector://vector\u0001mx_has_pickle_key": "true",
    "_vector://vector\u0001mx_last_room_id": "!assassas",
    "_vector://vector\u0001mx_oidc_client_id": "assassas",
    "_vector://vector\u0001mx_oidc_token_issuer": "https://account.matrix.org/",
    "_vector://vector\u0001mx_local_settings": "{\"language\":\"en\",\"use_system_theme\":false,\"theme\":\"dark\"}",
    "...": "20+ additional metadata entries"
  }
}
```

---

## Test Plan Implementation

### ✅ Test Validation Checklist

- [x] **LevelDB Format Compatibility**: Parser successfully reads Element Desktop's LevelDB format
- [x] **Metadata Extraction**: All critical metadata fields extracted successfully
- [x] **Data Normalization**: Control characters removed and data cleaned
- [x] **Binary Data Handling**: Non-UTF8 data converted to hex encoding
- [x] **JSON Export**: Valid JSON output generated and validated
- [x] **Error Handling**: Graceful error handling for edge cases
- [x] **Cross-Platform**: Code compiles on Linux (verified)
- [x] **Real-World Data**: Tested on actual Element Desktop database
- [x] **User Extraction**: Successfully extracted @bahrom04:matrix.org
- [x] **Device Detection**: Successfully identified device VFaWHJGKgR
- [x] **Room Detection**: Room IDs correctly identified and listed
- [x] **Release Build**: Optimized release binary compiles successfully

---

## Limitations & Future Enhancements

### Current Limitations
- **Read-only access** (by design - forensic preservation)
- **No encrypted message body extraction** (cryptographic limitation)
- **Single entry per metadata field** (design choice for clarity)
- **No automatic database discovery** (requires manual path)
- **No GUI interface** (CLI only in prototype)

### Planned Enhancements
- [ ] Message history parsing and timeline reconstruction
- [ ] Contact/presence information extraction
- [ ] Batch processing of multiple profiles
- [ ] Export to forensic formats (CSV, XLSX, HTML)
- [ ] Database integrity verification
- [ ] Graphical UI for analysis
- [ ] Automated database discovery
- [ ] Differential analysis between snapshots
- [ ] Timeline reconstruction with timestamps
- [ ] Integration with forensic frameworks

---

## References & Resources

### LevelDB Documentation
- [LevelDB GitHub Repository](https://github.com/google/leveldb)
- [rusty-leveldb Documentation](https://docs.rs/rusty-leveldb/4.0.1/)
- [LevelDB Format Specification](https://github.com/google/leveldb/blob/main/doc/impl.md)

### Element Desktop
- [Element Desktop GitHub](https://github.com/element-hq/element-desktop)
- [Matrix Protocol Specification](https://spec.matrix.org/)
- [Matrix Client SDKs](https://matrix.org/ecosystem/clients/)

### Rust Forensics
- [Rust Book](https://doc.rust-lang.org/book/)
- [Serde Documentation](https://serde.rs/)
- [anyhow Error Handling](https://docs.rs/anyhow/)

---

## Early Prototype Submission - Milestone 4

**Project**: Element Desktop LevelDB Metadata Parser
**Status**: ✅ **Complete & Validated**
**Date**: January 29, 2026
**Milestone**: 4 - Professionalism Section