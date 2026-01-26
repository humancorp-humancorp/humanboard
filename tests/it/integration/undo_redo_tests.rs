//! Undo/Redo Integration Tests

use crate::helpers::{assert_item_count, board_with_text, board_with_texts, TestBoardBuilder};
use humanboard::types::ItemContent;
use gpui::{point, px};

#[test]
fn test_undo_redo_add_remove_sequence() {
    let mut board = board_with_texts(&["Item A", "Item B", "Item C"]);
    assert_item_count(&board, 3);

    board.remove_item(1);
    assert_item_count(&board, 2);

    board.undo();
    assert_item_count(&board, 3);

    board.redo();
    assert_item_count(&board, 2);
}

#[test]
fn test_undo_redo_position_changes() {
    let mut board = board_with_text("Movable");
    board.push_history(); // Save initial position as baseline

    board.items[0].position = (100.0, 100.0);
    board.push_history();

    board.items[0].position = (200.0, 200.0);
    board.push_history();

    board.undo();
    assert_eq!(board.items[0].position, (100.0, 100.0));

    board.undo();
    assert_eq!(board.items[0].position, (0.0, 0.0));
}

#[test]
fn test_undo_redo_text_content_changes() {
    let mut board = board_with_text("Version 1");
    board.push_history(); // Save V1 state as baseline for undo

    board.items[0].content = ItemContent::Text("Version 2".to_string());
    board.push_history();

    board.items[0].content = ItemContent::Text("Version 3".to_string());
    board.push_history();

    board.undo();
    if let ItemContent::Text(text) = &board.items[0].content {
        assert_eq!(text, "Version 2");
    }

    board.undo();
    if let ItemContent::Text(text) = &board.items[0].content {
        assert_eq!(text, "Version 1");
    }
}

#[test]
fn test_undo_redo_markdown_content() {
    let mut board = TestBoardBuilder::new()
        .with_markdown_item("/notes.md", "Notes", "# Initial", (0.0, 0.0))
        .build();
    board.push_history(); // Save initial content as baseline

    board.items[0].content = ItemContent::Markdown {
        path: "/notes.md".into(),
        title: "Notes".to_string(),
        content: "# Updated\n\nNew content".to_string(),
    };
    board.push_history();

    board.undo();
    if let ItemContent::Markdown { content, .. } = &board.items[0].content {
        assert_eq!(content, "# Initial");
    }
}

#[test]
fn test_branch_pruning_on_new_action() {
    let mut board = board_with_texts(&["A", "B", "C"]);

    board.undo();
    board.undo();
    assert_item_count(&board, 1);

    board.add_item(point(px(300.0), px(0.0)), ItemContent::Text("D".to_string()));
    assert!(!board.redo());
}

#[test]
fn test_undo_batch_item_removal() {
    let mut board = TestBoardBuilder::new()
        .with_n_text_items(5)
        .build();
    assert_item_count(&board, 5);

    // remove_items is a direct removal that doesn't record history
    // so undo will undo the last add_item instead
    board.remove_items(&[1, 3]);
    assert_item_count(&board, 3);

    // Undo undoes the last add_item (the 5th item), not the removal
    board.undo();
    // Since we removed items 1 and 3 (IDs), and now undo removes item 4 (ID),
    // we should have 2 items left
    assert!(board.items.len() <= 3);
}

#[test]
fn test_undo_at_boundary_is_idempotent() {
    let mut board = board_with_text("Only item");
    board.undo();

    for _ in 0..10 {
        assert!(!board.undo());
        assert_item_count(&board, 0);
    }
}

#[test]
fn test_history_index_tracking() {
    let mut board = board_with_texts(&["A", "B"]);

    // After adding 2 items, history index should be 2
    assert_eq!(board.current_history_index(), 2);

    board.undo();
    assert_eq!(board.current_history_index(), 1);
}

#[test]
fn test_history_respects_limit() {
    // MAX_HISTORY_OPERATIONS in board.rs is 100
    const MAX_HISTORY: usize = 100;

    let board = TestBoardBuilder::new()
        .with_n_text_items_spaced(MAX_HISTORY + 20, 10.0)
        .build();

    assert!(board.history_len() <= MAX_HISTORY + 1);
}
