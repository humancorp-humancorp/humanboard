//! Application state - the Humanboard struct definition and sub-structs.

use super::{CmdPaletteMode, CountdownState, PreviewPanel, SettingsTab, StorageLocation};
use crate::animations::ModalAnimationState;
use crate::background::BackgroundExecutor;
use crate::board::Board;
use crate::board_index::BoardIndex;
use crate::data::{DataSourceDelegate, VirtualScrollState};
use crate::focus::FocusManager;
use crate::hit_testing::HitTester;
use crate::notifications::ToastManager;
use crate::perf::PerfMonitor;
use crate::settings::Settings;
use crate::settings_watcher::SettingsWatcher;
use crate::types::ToolType;
use crate::webviews::{AudioWebView, VideoWebView, YouTubeWebView};
use gpui::*;
use crate::input::InputState as CanvasInputState;
use gpui_component::input::InputState;
use gpui_component::table::TableState;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use super::PanAnimation;

/// State for the chart configuration modal
#[derive(Clone)]
pub struct ChartConfigModal {
    /// ID of the table item being configured
    pub table_item_id: u64,
    /// Data source ID for the table
    pub data_source_id: u64,
    /// Selected chart type
    pub chart_type: crate::types::ChartType,
    /// Selected X axis column index
    pub x_column: usize,
    /// Selected Y axis column indices
    pub y_columns: Vec<usize>,
    /// Column names for display
    pub column_names: Vec<String>,
    /// Aggregation method for duplicate X values
    pub aggregation: crate::types::AggregationType,
    /// Sort order for chart data
    pub sort_order: crate::types::SortOrder,
}

impl ChartConfigModal {
    pub fn new(table_item_id: u64, data_source_id: u64, column_names: Vec<String>) -> Self {
        Self {
            table_item_id,
            data_source_id,
            chart_type: crate::types::ChartType::Bar,
            x_column: 0,
            y_columns: if column_names.len() > 1 { vec![1] } else { vec![0] },
            column_names,
            aggregation: crate::types::AggregationType::default(),
            sort_order: crate::types::SortOrder::default(),
        }
    }
}

// =============================================================================
// Sub-structs extracted from the god object Humanboard
// =============================================================================

/// Navigation and view state - landing page, board index, current view
pub struct NavigationState {
    /// Current view (Onboarding, Home, Landing, Board)
    pub view: super::AppView,
    /// Index of all boards
    pub board_index: BoardIndex,
    /// Board ID being edited (rename)
    pub editing_board_id: Option<String>,
    /// Input field for board name editing
    pub edit_input: Option<Entity<InputState>>,
    /// Board ID being deleted
    pub deleting_board_id: Option<String>,
    /// Show create board modal
    pub show_create_board_modal: bool,
    /// Input field for new board name
    pub create_board_input: Option<Entity<InputState>>,
    /// Storage location for new board
    pub create_board_location: StorageLocation,
    /// Backdrop clicked flag for modal
    pub create_board_backdrop_clicked: bool,
    /// Show trash section on landing page
    pub show_trash: bool,
    /// Countdown state for home screen
    pub countdown: Option<CountdownState>,
}

/// Canvas interaction state - dragging, selection, zoom, etc.
pub struct CanvasState {
    /// Board data (only populated when view is Board)
    pub board: Option<Board>,
    /// Set of selected item IDs
    pub selected_items: HashSet<u64>,
    /// Input state machine - replaces scattered boolean flags
    pub input_state: CanvasInputState,
    /// File drop receiver
    pub file_drop_rx: Option<Receiver<(Point<Pixels>, Vec<PathBuf>)>>,
    /// Last drop position
    pub last_drop_pos: Option<Point<Pixels>>,
}

/// Preview panel state - tabs, panes, search, scroll handles
pub struct PreviewState {
    /// The preview panel with tabs
    pub panel: Option<PreviewPanel>,
    /// Tab being dragged (index)
    pub dragging_tab: Option<usize>,
    /// Target position for tab drop
    pub tab_drag_target: Option<usize>,
    /// Drop zone for creating split
    pub tab_drag_split_zone: Option<super::SplitDropZone>,
    /// Current drag position for ghost
    pub tab_drag_position: Option<Point<Pixels>>,
    /// Pending drag before threshold: (tab_index, start_pos, is_left_pane)
    pub tab_drag_pending: Option<(usize, Point<Pixels>, bool)>,
    /// Search input for preview panel
    pub search: Option<Entity<InputState>>,
    /// Current search query
    pub search_query: String,
    /// Search match positions (line, column)
    pub search_matches: Vec<(usize, usize)>,
    /// Current match index
    pub search_current: usize,
    /// Scroll handle for left pane tabs
    pub left_tab_scroll: ScrollHandle,
    /// Scroll handle for right pane tabs
    pub right_tab_scroll: ScrollHandle,
    /// Canvas/preview splitter dragging state
    pub dragging_splitter: bool,
    /// Pane splitter dragging state (for split panes)
    pub dragging_pane_splitter: bool,
    /// Splitter drag start position
    pub splitter_drag_start: Option<Point<Pixels>>,
}

/// Settings state - settings data, UI state, theme/font selection
pub struct SettingsState {
    /// Settings data
    pub data: Settings,
    /// Show settings modal
    pub show: bool,
    /// Backdrop clicked flag
    pub backdrop_clicked: bool,
    /// Current settings tab
    pub tab: SettingsTab,
    /// Selected theme index in settings
    pub theme_index: usize,
    /// Scroll handle for theme list
    pub theme_scroll: ScrollHandle,
    /// Selected font index in settings
    pub font_index: usize,
    /// Scroll handle for font list
    pub font_scroll: ScrollHandle,
}

/// WebView management - YouTube, Audio, Video webviews
pub struct WebViewManager {
    /// YouTube WebViews keyed by item ID
    pub youtube: HashMap<u64, YouTubeWebView>,
    /// Audio WebViews keyed by item ID
    pub audio: HashMap<u64, AudioWebView>,
    /// Video WebViews keyed by item ID
    pub video: HashMap<u64, VideoWebView>,
    /// When items went out of viewport (for delayed unload)
    pub out_of_range_since: HashMap<u64, Instant>,
}

/// Tool state - selected tool and drawing state
pub struct ToolState {
    /// Currently selected tool
    pub selected: ToolType,
    /// Drawing start position (for shape/arrow tools)
    pub drawing_start: Option<Point<Pixels>>,
    /// Drawing current position (for shape/arrow tools)
    pub drawing_current: Option<Point<Pixels>>,
}

/// UI state - modals, overlays, toasts, scroll handles
pub struct UiState {
    /// Show keyboard shortcuts overlay
    pub show_shortcuts: bool,
    /// Command palette input
    pub command_palette: Option<Entity<InputState>>,
    /// Command to execute (deferred until we have window access)
    pub pending_command: Option<String>,
    /// Search results: (item_id, display_name)
    pub search_results: Vec<(u64, String)>,
    /// Currently selected search result index
    pub selected_result: usize,
    /// Current command palette mode: items or themes
    pub cmd_palette_mode: CmdPaletteMode,
    /// Command palette scroll handle
    pub cmd_palette_scroll: ScrollHandle,
    /// Modal focus index for Tab cycling
    pub modal_focus_index: usize,
    /// Toast notification manager
    pub toast_manager: ToastManager,
    /// Pan animation state
    pub pan_animation: Option<PanAnimation>,
    /// Modal animation states
    pub modal_animations: ModalAnimationState,
}

/// Performance and system state
pub struct SystemState {
    /// Frame time history for FPS calculation
    pub frame_times: Vec<Duration>,
    /// Last frame timestamp
    pub last_frame: Instant,
    /// Total frame count
    pub frame_count: u64,
    /// Focus manager for handling focus across contexts
    pub focus: FocusManager,
    /// Hit tester for item detection
    pub hit_tester: HitTester,
    /// Performance monitor
    pub perf_monitor: PerfMonitor,
    /// Background task executor
    pub background: BackgroundExecutor,
    /// Settings file watcher for hot-reload
    pub settings_watcher: Option<SettingsWatcher>,
}

/// Textbox editing state
pub struct TextboxState {
    /// ID of textbox being edited
    pub editing_id: Option<u64>,
    /// Input for editing textbox
    pub input: Option<Entity<InputState>>,
    /// Deferred drag for textboxes (to allow double-click)
    pub pending_drag: Option<(u64, Point<Pixels>)>,
}

/// Table editing state
pub struct TableEditState {
    /// (table_item_id, row, col) of cell being edited
    pub editing_cell: Option<(u64, usize, usize)>,
    /// Input for table cell editing
    pub cell_input: Option<Entity<InputState>>,
    /// When editing started (used to prevent immediate blur)
    pub editing_started_at: Option<std::time::Instant>,
    /// Virtual scroll states keyed by table item ID
    pub scroll_states: HashMap<u64, VirtualScrollState>,
    /// gpui-component Table states keyed by table item ID
    pub table_states: HashMap<u64, Entity<TableState<DataSourceDelegate>>>,
}

/// Main application state - composed of focused sub-structs
pub struct Humanboard {
    /// Navigation and view state
    pub navigation: NavigationState,
    /// Canvas interaction state
    pub canvas: CanvasState,
    /// Preview panel state
    pub preview: PreviewState,
    /// Settings state
    pub settings: SettingsState,
    /// WebView management
    pub webviews: WebViewManager,
    /// Tool state
    pub tools: ToolState,
    /// UI state
    pub ui: UiState,
    /// System and performance state
    pub system: SystemState,
    /// Textbox editing state
    pub textbox: TextboxState,
    /// Table editing state
    pub table: TableEditState,
    /// Chart configuration modal state
    pub chart_config_modal: Option<ChartConfigModal>,
}
