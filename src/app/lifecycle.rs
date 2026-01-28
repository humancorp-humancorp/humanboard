//! Application lifecycle - initialization and cleanup methods.

use super::{AppView, CmdPaletteMode, CountdownState, Humanboard, SettingsTab, StorageLocation};
use crate::animations::ModalAnimationState;
use crate::app::state::{CanvasState, NavigationState, PreviewState, SettingsState, SystemState, TextboxState, ToolState, UiState, WebViewManager, TableEditState};
use crate::background::BackgroundExecutor;
use crate::board_index::BoardIndex;
use crate::focus::FocusManager;
use crate::hit_testing::HitTester;
use crate::notifications::ToastManager;
use crate::perf::PerfMonitor;
use crate::settings::Settings;
use crate::settings_watcher::{SettingsEvent, SettingsWatcher};
use crate::types::ToolType;
use gpui::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

impl Humanboard {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let board_index = BoardIndex::load();

        // Check if onboarding has been completed
        let initial_view = if crate::settings::is_onboarding_completed() {
            AppView::Landing
        } else {
            AppView::Onboarding
        };

        Self {
            navigation: NavigationState {
                view: initial_view,
                board_index,
                editing_board_id: None,
                edit_input: None,
                deleting_board_id: None,
                show_create_board_modal: false,
                create_board_input: None,
                create_board_location: StorageLocation::default(),
                create_board_backdrop_clicked: false,
                show_trash: false,
                countdown: Some(CountdownState::until_midnight()),
            },
            canvas: CanvasState {
                board: None,
                selected_items: HashSet::new(),
                input_state: crate::input::InputState::default(),
                file_drop_rx: None,
                last_drop_pos: None,
            },
            preview: PreviewState {
                panel: None,
                dragging_tab: None,
                tab_drag_target: None,
                tab_drag_split_zone: None,
                tab_drag_position: None,
                tab_drag_pending: None,
                search: None,
                search_query: String::new(),
                search_matches: Vec::new(),
                search_current: 0,
                left_tab_scroll: ScrollHandle::new(),
                right_tab_scroll: ScrollHandle::new(),
                dragging_splitter: false,
                dragging_pane_splitter: false,
                splitter_drag_start: None,
            },
            settings: SettingsState {
                data: Settings::load(),
                show: false,
                backdrop_clicked: false,
                tab: SettingsTab::default(),
                theme_index: 0,
                theme_scroll: ScrollHandle::new(),
                font_index: 0,
                font_scroll: ScrollHandle::new(),
            },
            webviews: WebViewManager {
                youtube: HashMap::new(),
                audio: HashMap::new(),
                video: HashMap::new(),
                out_of_range_since: HashMap::new(),
            },
            tools: ToolState {
                selected: ToolType::default(),
                drawing_start: None,
                drawing_current: None,
            },
            ui: UiState {
                show_shortcuts: false,
                command_palette: None,
                pending_command: None,
                search_results: Vec::new(),
                selected_result: 0,
                cmd_palette_mode: CmdPaletteMode::default(),
                cmd_palette_scroll: ScrollHandle::new(),
                modal_focus_index: 0,
                toast_manager: ToastManager::new(),
                pan_animation: None,
                modal_animations: ModalAnimationState::default(),
            },
            system: SystemState {
                frame_times: Vec::with_capacity(60),
                last_frame: Instant::now(),
                frame_count: 0,
                focus: FocusManager::new(cx),
                hit_tester: HitTester::new(),
                perf_monitor: PerfMonitor::new(),
                background: BackgroundExecutor::with_default_workers(),
                settings_watcher: crate::settings_watcher::default_settings_path()
                    .and_then(|p| SettingsWatcher::new(p).ok()),
            },
            textbox: TextboxState {
                editing_id: None,
                input: None,
                pending_drag: None,
            },
            table: TableEditState {
                editing_cell: None,
                cell_input: None,
                editing_started_at: None,
                scroll_states: HashMap::new(),
                table_states: HashMap::new(),
            },
            chart_config_modal: None,
        }
    }

    /// Check for settings file changes and reload if needed.
    pub fn check_settings_reload(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut watcher) = self.system.settings_watcher {
            if let Some(event) = watcher.poll() {
                match event {
                    SettingsEvent::Modified | SettingsEvent::Created => {
                        tracing::info!("Settings file changed, reloading...");
                        // Reload settings
                        self.settings.data = Settings::load();
                        self.ui.toast_manager.push(crate::notifications::Toast::info("Settings reloaded"));
                        cx.notify();
                    }
                    SettingsEvent::Deleted => {
                        tracing::warn!("Settings file deleted");
                        self.ui.toast_manager
                            .push(crate::notifications::Toast::warning("Settings file deleted"));
                    }
                    SettingsEvent::Error(e) => {
                        tracing::error!("Settings watch error: {}", e);
                    }
                }
            }
        }
    }

    /// Returns true if a code editor is currently in edit mode
    pub fn is_code_editing(&self) -> bool {
        self.preview.panel
            .as_ref()
            .and_then(|p| p.tabs.get(p.active_tab))
            .map(|tab| tab.is_editing())
            .unwrap_or(false)
    }
}
