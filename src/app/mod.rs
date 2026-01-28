//! Application module - the main Humanboard application state and logic.
//!
//! This module is organized into several submodules:
//! - `types` - Enums, structs, and type definitions
//! - `state` - The Humanboard struct definition and sub-structs
//! - `lifecycle` - Initialization and cleanup methods
//! - `board_management` - Board CRUD operations
//! - `settings_handlers` - Theme, font, and settings management
//! - `command_palette_handlers` - Command palette functionality
//! - `preview_core` - Core preview panel operations
//! - `preview_webviews` - YouTube, Audio, Video webview management
//! - `preview_tabs` - Tab close, drag, and history management
//! - `preview_panes` - Tab switching and pane split management
//! - `preview_search` - Find in file functionality
//! - `textbox` - Textbox editing and utility methods

mod types;
mod state;
mod lifecycle;
mod board_management;
mod settings_handlers;
mod command_palette_handlers;
mod preview_core;
mod preview_webviews;
mod preview_tabs;
mod preview_panes;
mod preview_search;
mod textbox;
mod error_recovery;
mod data_viz;
mod table_editing;

pub use types::*;
pub use state::{ChartConfigModal, Humanboard};

// Re-export sub-structs for use in other modules
pub use state::{
    NavigationState, CanvasState, PreviewState, SettingsState,
    WebViewManager, ToolState, UiState, SystemState, TextboxState, TableEditState,
};
