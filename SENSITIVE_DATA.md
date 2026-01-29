# Sensitive Data Analysis - Element Desktop LevelDB Parser

## Overview
This document identifies which keys/arguments parsed by the Element Desktop LevelDB parser contain **HIGHLY SENSITIVE** information that could compromise user privacy and security.

## CRITICAL SENSITIVE KEYS

### 1. **Authentication Tokens** 🔴 HIGHEST RISK
- **`mx_oidc_id_token`** - JWT ID token containing:
  - User subject identifier (sub)
  - Authentication time
  - Token expiration
  - Client ID
  - Nonce values
  - Can be used for unauthorized access if not expired
  
- **`mx_has_access_token`** - Boolean flag indicating active session
- **`mx_has_refresh_token`** - Boolean flag for token refresh capability
- **`mx_oidc_client_id`** - Client identifier for authentication flow

### 2. **User Identity Information** 🔴 HIGH RISK
- **`user_id`** (`mx_user_id`) - Full Matrix user identifier
  - Format: `@username:matrix.org`
  - Reveals user account across all rooms
  
- **`display_name`** (`mx_profile_displayname`) - Real name/pseudonym
  - Can identify user in real world
  - May link to social media accounts

- **`avatar_url`** (`mx_profile_avatar_url`) - Avatar image location
  - mxc:// protocol URL pointing to avatar
  - Can track user across platforms

### 3. **Device & Encryption Information** 🔴 HIGH RISK
- **`device_id`** (`mx_device_id`) - Unique device identifier
  - Tracks specific device across sessions
  - Enables device fingerprinting
  
- **`mx_has_pickle_key`** - Boolean indicating encryption key storage
  - Shows device has encryption capability
  - May indicate encryption of sensitive data

- **`mx_crypto_initialised`** - Flag indicating E2E encryption enabled
  - Shows user participates in encrypted communications

- **`curve25519_key`** / **`ed25519_key`** - Encryption keys
  - NOT currently extracted in structured form
  - Present in raw_entries if exported
  - Used for message encryption/decryption

### 4. **Communication Metadata** 🟠 MEDIUM-HIGH RISK
- **`room_ids`** - Enumeration of all rooms user participates in
  - Format: `!RoomId:matrix.org`
  - Reveals all communities/groups user is member of
  - Example: `!F4lx_vebTWh6X3T19Z2T7IPmJxLdWZd7sVIEwm8LE4Q`

- **`mx_last_room_id`** - Last accessed room
  - Reveals current/recent activity

- **`mx_space_context_home-space`** - Primary space/community
  - Shows user's primary interest area

- **`encrypted_rooms`** - List of encrypted communications
  - Identifies which relationships use encryption

### 5. **Server & Configuration Information** 🟠 MEDIUM-HIGH RISK
- **`mx_hs_url`** - Homeserver URL
  - Shows which matrix server user relies on
  - Typically: `https://matrix-client.matrix.org`

- **`mx_oidc_token_issuer`** - OIDC provider URL
  - Shows authentication infrastructure
  - Typically: `https://account.matrix.org/`

- **`mx_is_url`** - Element Identity Services URL
  - Shows user's trusted identity provider

### 6. **Session & Activity Metadata** 🟡 MEDIUM RISK
- **`mx_draft_cleanup`** - Last draft cleanup timestamp
  - Reveals recent activity times
  - Unix timestamp: `1768054204763`

- **`mx_last_room_directory_server`** - Last directory server queried
  - Shows directory search behavior

- **`mx_show_images_migration_done`** - Configuration flag
  - User preference tracking

- **`must_verify_device`** - Security verification status
  - Shows if device requires verification

- **`mxjssdk_memory_filter_FILTER_SYNC_<user_id>`** - Sync cache metadata
  - Contains filter state information

### 7. **Behavioral & Preference Data** 🟡 MEDIUM RISK
- **`mx_local_settings`** - User preferences JSON:
  ```json
  {
    "language": "en",
    "use_system_theme": false,
    "theme": "dark",
    "showMediaEventIds": {}
  }
  ```
  - Reveals UI preferences
  - Language selection
  - Accessibility settings

- **`mx_setting_RightPanel.phases_<room_id>`** - UI state per room
  - Shows which rooms user has explored in detail
  - Panel state (open/closed)
  - Panel navigation history

- **`url_previews_e2ee_migration_done`** - Feature flag
  - Shows which features user has enabled

### 8. **Encryption Failure Data** 🟡 MEDIUM RISK
- **`mx_decryption_failure_event_ids`** - ScalableBloomFilter
  - Compressed list of events that failed decryption
  - Can reveal communication patterns
  - Bloom filter containing 77+ event hashes initially

## Risk Levels Summary

| Risk Level | Count | Key Examples |
|-----------|-------|--------------|
| 🔴 CRITICAL | 8 | ID tokens, user ID, device ID, encryption keys |
| 🟠 HIGH | 8 | Room IDs, server URLs, activity metadata |
| 🟡 MEDIUM | 6 | Preferences, behavioral data, feature flags |
| 🟢 LOW | - | VERSION flag, metadata timestamps |


### Usage Guidelines
1. **Don't share output**: Keep JSON output confidential
2. **Secure parsing**: Use on trusted systems only
3. **Temporary copies**: Delete leveldb copy after analysis
4. **Audit logs**: Review who has accessed your data
5. **Clean up**: Ensure leveldb/ is in .gitignore

## Conclusion

The Element Desktop LevelDB contains **HIGHLY SENSITIVE** information that:

✓ **Uniquely identifies users** - User ID, display name, avatar  
✓ **Maps social networks** - Complete room/relationship enumeration  
✓ **Reveals activity patterns** - Timestamps and access history  
✓ **Enables impersonation** - Active authentication tokens  
✓ **Exposes encryption capability** - Device keys and status  

**This parser should be used ONLY on trusted systems with proper authorization.**

---

**Classification**: Internal Use / Forensic Analysis  
**Updated**: January 2026  
**Tool**: Element Desktop LevelDB Metadata Parser (Milestone 4)
