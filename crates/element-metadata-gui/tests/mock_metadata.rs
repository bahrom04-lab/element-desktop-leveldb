use element_metadata::ElementMetadata;
use element_metadata_gui::window::App;

fn mock_metadata() -> ElementMetadata {
    ElementMetadata {
        user_id: Some("@bahrom:matrix.org".into()),
        display_name: Some("Bahrom".into()),
        avatar_url: None,
        device_id: None,
        device_name: None,
        theme: None,
        language: None,
        notifications_enabled: None,
        room_ids: vec!["!room1".into(), "!room2".into()],
        encrypted_rooms: vec!["!secret1".into()],
        curve25519_key: None,
        ed25519_key: None,
        raw_entries: std::collections::HashMap::new(),
    }
}

#[test]
fn test_format_metadata_basic() {
    let meta = mock_metadata();
    let result = App::format_metadata(&Some(meta));

    assert!(result.contains("User"));
    assert!(result.contains("@bahrom:matrix.org"));
    assert!(result.contains("Bahrom"));
}

#[test]
fn test_format_metadata_none() {
    let result = App::format_metadata(&None);
    assert_eq!(result, "<i>No metadata</i>");
}

#[test]
fn test_build_rooms() {
    let meta = mock_metadata();

    let (rooms, encrypted) = App::build_rooms(&meta);

    assert_eq!(rooms.len(), 2);
    assert_eq!(encrypted.len(), 1);

    assert_eq!(rooms[0], ("!room1".into(), 0));
    assert_eq!(rooms[1], ("!room2".into(), 1));
    assert_eq!(encrypted[0], ("!secret1".into(), 0));
}

#[test]
fn test_empty_rooms() {
    let mut meta = mock_metadata();
    meta.room_ids.clear();
    meta.encrypted_rooms.clear();

    let (rooms, encrypted) = App::build_rooms(&meta);

    assert!(rooms.is_empty());
    assert!(encrypted.is_empty());
}
