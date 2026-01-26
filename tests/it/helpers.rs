//! Test helpers and builders for reducing boilerplate in tests.
//!
//! This module provides:
//! - `TestBoardBuilder` - Builder pattern for creating test boards with items
//! - Helper functions like `add_text_item()`, `add_image_item()`, etc.
//! - Common test fixtures and utilities

use humanboard::board::{Board, BoardState};
use humanboard::selection::SelectionManager;
use humanboard::types::{CanvasItem, ItemContent};
use gpui::{point, px, Point, Pixels};
use std::path::PathBuf;

// ============================================================================
// TestBoardBuilder - Builder pattern for creating test boards
// ============================================================================

/// Builder for creating test boards with items and configuration.
///
/// # Example
/// ```ignore
/// let board = TestBoardBuilder::new()
///     .with_text_item("First note", (0.0, 0.0))
///     .with_text_item("Second note", (100.0, 0.0))
///     .with_zoom(1.5)
///     .with_offset(50.0, 50.0)
///     .build();
/// ```
pub struct TestBoardBuilder {
    items: Vec<(Point<Pixels>, ItemContent)>,
    zoom: f32,
    offset: (f32, f32),
}

impl Default for TestBoardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TestBoardBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            zoom: 1.0,
            offset: (0.0, 0.0),
        }
    }

    /// Set the zoom level.
    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    /// Set the canvas offset.
    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset = (x, y);
        self
    }

    /// Add a text item at the specified position.
    pub fn with_text_item(mut self, text: impl Into<String>, pos: (f32, f32)) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::Text(text.into()),
        ));
        self
    }

    /// Add multiple text items with auto-incrementing x positions.
    ///
    /// Items are placed at (0, 0), (100, 0), (200, 0), etc.
    pub fn with_text_items(mut self, texts: &[&str]) -> Self {
        for (i, text) in texts.iter().enumerate() {
            self.items.push((
                point(px(i as f32 * 100.0), px(0.0)),
                ItemContent::Text((*text).to_string()),
            ));
        }
        self
    }

    /// Add an image item at the specified position.
    pub fn with_image_item(mut self, path: impl Into<PathBuf>, pos: (f32, f32)) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::Image(path.into()),
        ));
        self
    }

    /// Add a markdown item at the specified position.
    pub fn with_markdown_item(
        mut self,
        path: impl Into<PathBuf>,
        title: impl Into<String>,
        content: impl Into<String>,
        pos: (f32, f32),
    ) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::Markdown {
                path: path.into(),
                title: title.into(),
                content: content.into(),
            },
        ));
        self
    }

    /// Add a code item at the specified position.
    pub fn with_code_item(
        mut self,
        path: impl Into<PathBuf>,
        language: impl Into<String>,
        pos: (f32, f32),
    ) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::Code {
                path: path.into(),
                language: language.into(),
            },
        ));
        self
    }

    /// Add a PDF item at the specified position.
    pub fn with_pdf_item(mut self, path: impl Into<PathBuf>, pos: (f32, f32)) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::Pdf {
                path: path.into(),
                thumbnail: None,
            },
        ));
        self
    }

    /// Add a video item at the specified position.
    pub fn with_video_item(mut self, path: impl Into<PathBuf>, pos: (f32, f32)) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::Video(path.into()),
        ));
        self
    }

    /// Add an audio item at the specified position.
    pub fn with_audio_item(mut self, path: impl Into<PathBuf>, pos: (f32, f32)) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::Audio(path.into()),
        ));
        self
    }

    /// Add a YouTube item at the specified position.
    pub fn with_youtube_item(mut self, video_id: impl Into<String>, pos: (f32, f32)) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::YouTube(video_id.into()),
        ));
        self
    }

    /// Add a link item at the specified position.
    pub fn with_link_item(mut self, url: impl Into<String>, pos: (f32, f32)) -> Self {
        self.items.push((
            point(px(pos.0), px(pos.1)),
            ItemContent::Link(url.into()),
        ));
        self
    }

    /// Add a custom content item at the specified position.
    pub fn with_item(mut self, content: ItemContent, pos: (f32, f32)) -> Self {
        self.items.push((point(px(pos.0), px(pos.1)), content));
        self
    }

    /// Add N text items with sequential content ("Item 0", "Item 1", etc.)
    ///
    /// Items are placed at (i * spacing, 0) where spacing defaults to 100.0
    pub fn with_n_text_items(mut self, count: usize) -> Self {
        for i in 0..count {
            self.items.push((
                point(px(i as f32 * 100.0), px(0.0)),
                ItemContent::Text(format!("Item {}", i)),
            ));
        }
        self
    }

    /// Add N text items with custom spacing.
    pub fn with_n_text_items_spaced(mut self, count: usize, spacing: f32) -> Self {
        for i in 0..count {
            self.items.push((
                point(px(i as f32 * spacing), px(0.0)),
                ItemContent::Text(format!("Item {}", i)),
            ));
        }
        self
    }

    /// Build the Board with all configured items.
    pub fn build(self) -> Board {
        let mut board = Board::new_for_test();
        board.canvas_offset = point(px(self.offset.0), px(self.offset.1));
        board.zoom = self.zoom;

        for (pos, content) in self.items {
            board.add_item(pos, content);
        }

        board
    }
}

// ============================================================================
// Standalone helper functions
// ============================================================================

/// Create a test board with a single text item.
pub fn board_with_text(text: &str) -> Board {
    TestBoardBuilder::new()
        .with_text_item(text, (0.0, 0.0))
        .build()
}

/// Create a test board with multiple text items at default positions.
pub fn board_with_texts(texts: &[&str]) -> Board {
    TestBoardBuilder::new().with_text_items(texts).build()
}

/// Create an empty test board.
pub fn empty_board() -> Board {
    Board::new_for_test()
}

/// Create a SelectionManager with items already selected.
pub fn selection_with_items(ids: &[u64]) -> SelectionManager {
    let mut selection = SelectionManager::new();
    for &id in ids {
        selection.toggle(id);
    }
    selection
}

/// Create a BoardState from a Board for serialization tests.
pub fn board_to_state(board: &Board) -> BoardState {
    BoardState {
        canvas_offset: (
            f32::from(board.canvas_offset.x),
            f32::from(board.canvas_offset.y),
        ),
        zoom: board.zoom,
        items: board
            .items
            .iter()
            .map(|item| CanvasItem {
                id: item.id,
                position: item.position,
                size: item.size,
                content: item.content.clone(),
            })
            .collect(),
        next_item_id: board.next_item_id,
        data_sources: board.data_sources.clone(),
        next_data_source_id: board.next_data_source_id,
    }
}

/// Create a minimal CanvasItem for testing.
pub fn test_canvas_item(id: u64, text: &str) -> CanvasItem {
    CanvasItem {
        id,
        position: (0.0, 0.0),
        size: (300.0, 100.0),
        content: ItemContent::Text(text.to_string()),
    }
}

/// Create a CanvasItem with custom position and size.
pub fn test_canvas_item_at(id: u64, text: &str, pos: (f32, f32), size: (f32, f32)) -> CanvasItem {
    CanvasItem {
        id,
        position: pos,
        size,
        content: ItemContent::Text(text.to_string()),
    }
}

// ============================================================================
// Content creation helpers
// ============================================================================

/// Create text content.
pub fn text_content(text: &str) -> ItemContent {
    ItemContent::Text(text.to_string())
}

/// Create markdown content.
pub fn markdown_content(path: &str, title: &str, content: &str) -> ItemContent {
    ItemContent::Markdown {
        path: PathBuf::from(path),
        title: title.to_string(),
        content: content.to_string(),
    }
}

/// Create code content.
pub fn code_content(path: &str, language: &str) -> ItemContent {
    ItemContent::Code {
        path: PathBuf::from(path),
        language: language.to_string(),
    }
}

/// Create image content.
pub fn image_content(path: &str) -> ItemContent {
    ItemContent::Image(PathBuf::from(path))
}

/// Create PDF content.
pub fn pdf_content(path: &str) -> ItemContent {
    ItemContent::Pdf {
        path: PathBuf::from(path),
        thumbnail: None,
    }
}

/// Create video content.
pub fn video_content(path: &str) -> ItemContent {
    ItemContent::Video(PathBuf::from(path))
}

/// Create audio content.
pub fn audio_content(path: &str) -> ItemContent {
    ItemContent::Audio(PathBuf::from(path))
}

// ============================================================================
// Position helpers
// ============================================================================

/// Create a Point from (x, y) tuple.
pub fn pos(x: f32, y: f32) -> Point<Pixels> {
    point(px(x), px(y))
}

/// Common positions for test items in a grid layout.
pub mod positions {
    pub const ORIGIN: (f32, f32) = (0.0, 0.0);
    pub const ROW1_COL1: (f32, f32) = (0.0, 0.0);
    pub const ROW1_COL2: (f32, f32) = (100.0, 0.0);
    pub const ROW1_COL3: (f32, f32) = (200.0, 0.0);
    pub const ROW2_COL1: (f32, f32) = (0.0, 100.0);
    pub const ROW2_COL2: (f32, f32) = (100.0, 100.0);
    pub const ROW2_COL3: (f32, f32) = (200.0, 100.0);

    /// Generate a position at column index (spacing of 100)
    pub fn col(index: usize) -> (f32, f32) {
        (index as f32 * 100.0, 0.0)
    }

    /// Generate a position at row, column
    pub fn at(row: usize, col: usize) -> (f32, f32) {
        (col as f32 * 100.0, row as f32 * 100.0)
    }
}

// ============================================================================
// Assertion helpers
// ============================================================================

/// Assert that a board has a specific number of items.
pub fn assert_item_count(board: &Board, expected: usize) {
    assert_eq!(
        board.items.len(),
        expected,
        "Expected {} items, found {}",
        expected,
        board.items.len()
    );
}

/// Assert that an item exists with specific text content.
pub fn assert_has_text_item(board: &Board, id: u64, expected_text: &str) {
    let item = board.get_item(id);
    assert!(item.is_some(), "Item {} not found", id);
    if let ItemContent::Text(text) = &item.unwrap().content {
        assert_eq!(text, expected_text, "Item {} has wrong text", id);
    } else {
        panic!("Item {} is not a text item", id);
    }
}

/// Assert that an item exists at a specific position.
pub fn assert_item_position(board: &Board, id: u64, expected_pos: (f32, f32)) {
    let item = board.get_item(id);
    assert!(item.is_some(), "Item {} not found", id);
    assert_eq!(
        item.unwrap().position, expected_pos,
        "Item {} has wrong position",
        id
    );
}

// ============================================================================
// Tests for the helpers themselves
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creates_empty_board() {
        let board = TestBoardBuilder::new().build();
        assert!(board.items.is_empty());
        assert_eq!(board.zoom, 1.0);
    }

    #[test]
    fn test_builder_with_text_items() {
        let board = TestBoardBuilder::new()
            .with_text_item("First", (0.0, 0.0))
            .with_text_item("Second", (100.0, 0.0))
            .build();

        assert_eq!(board.items.len(), 2);
    }

    #[test]
    fn test_builder_with_zoom() {
        let board = TestBoardBuilder::new().with_zoom(2.0).build();
        assert_eq!(board.zoom, 2.0);
    }

    #[test]
    fn test_builder_with_offset() {
        let board = TestBoardBuilder::new().with_offset(50.0, 75.0).build();
        assert_eq!(f32::from(board.canvas_offset.x), 50.0);
        assert_eq!(f32::from(board.canvas_offset.y), 75.0);
    }

    #[test]
    fn test_board_with_texts_helper() {
        let board = board_with_texts(&["A", "B", "C"]);
        assert_eq!(board.items.len(), 3);
    }

    #[test]
    fn test_positions_helper() {
        assert_eq!(positions::col(0), (0.0, 0.0));
        assert_eq!(positions::col(3), (300.0, 0.0));
        assert_eq!(positions::at(1, 2), (200.0, 100.0));
    }
}
