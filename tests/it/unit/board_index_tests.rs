//! Unit tests for board_index module.

use humanboard::board_index::{BoardIndex, BoardMetadata};

#[test]
fn test_board_metadata_new() {
    let meta = BoardMetadata::new("Test Board".to_string());
    assert_eq!(meta.name, "Test Board");
    assert!(!meta.id.is_empty());
    assert!(meta.created_at > 0);
    assert_eq!(meta.created_at, meta.updated_at);
}

#[test]
fn test_board_metadata_touch() {
    let mut meta = BoardMetadata::new("Test".to_string());
    let original = meta.updated_at;

    // Touch should update the timestamp
    // Note: In fast execution, updated_at might be the same if within the same second.
    // The important thing is that touch() doesn't panic and updates (or keeps) the timestamp.
    meta.touch();

    // updated_at should be >= original (it's based on system time)
    assert!(meta.updated_at >= original, "updated_at should not go backwards");
}

#[test]
fn test_formatted_date_just_now() {
    let meta = BoardMetadata::new("Test".to_string());
    assert_eq!(meta.formatted_date(), "Just now");
}

#[test]
fn test_uuid_generated_unique() {
    // Test that each BoardMetadata gets a unique ID
    let meta1 = BoardMetadata::new("Test 1".to_string());
    let meta2 = BoardMetadata::new("Test 2".to_string());
    assert_ne!(meta1.id, meta2.id);
    assert_eq!(meta1.id.len(), 36); // UUID v4 format with hyphens
}

#[test]
fn test_board_index_default() {
    let index = BoardIndex::default();
    assert!(index.boards.is_empty());
}

#[test]
fn test_board_metadata_is_not_deleted_by_default() {
    let meta = BoardMetadata::new("Test".to_string());
    assert!(!meta.is_deleted());
}

#[test]
fn test_board_metadata_trash_and_restore() {
    let mut meta = BoardMetadata::new("Test".to_string());

    assert!(!meta.is_deleted());

    meta.move_to_trash();
    assert!(meta.is_deleted());
    assert!(meta.deleted_at.is_some());

    meta.restore();
    assert!(!meta.is_deleted());
    assert!(meta.deleted_at.is_none());
}
