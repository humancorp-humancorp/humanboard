//! Board management methods - create, open, edit, delete, trash operations

use super::{AppView, Humanboard, StorageLocation};
use crate::board::Board;
use crate::board_index::BoardIndex;
use crate::focus::FocusContext;
use gpui::*;
use gpui_component::input::InputState;

impl Humanboard {
    // ==================== Landing Page Methods ====================

    /// Show the create board modal with input field
    pub fn show_create_board_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.system.focus.focus(FocusContext::Modal, window);
        self.reset_modal_focus(); // Reset focus index for Tab cycling

        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Enter board name..."));

        // Focus the input
        input.update(cx, |state, cx| {
            state.focus(window, cx);
        });

        self.navigation.create_board_input = Some(input);
        self.navigation.create_board_location = StorageLocation::default();
        self.navigation.show_create_board_modal = true;
        self.ui.modal_animations.open_create_board();
        cx.notify();
    }

    /// Close the create board modal without creating
    pub fn close_create_board_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Start close animation - modal will be hidden when animation completes
        self.ui.modal_animations.close_create_board();
        self.system.focus.release(FocusContext::Modal, window);
        cx.notify();
    }

    /// Clean up create board modal after close animation completes
    pub fn finish_close_create_board(&mut self) {
        self.navigation.show_create_board_modal = false;
        self.navigation.create_board_input = None;
        self.navigation.create_board_location = StorageLocation::default();
    }

    /// Set the storage location for the new board
    pub fn set_create_board_location(&mut self, location: StorageLocation, cx: &mut Context<Self>) {
        self.navigation.create_board_location = location;
        cx.notify();
    }

    /// Create a new board with custom name and storage location
    pub fn confirm_create_board(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = self
            .navigation.create_board_input
            .as_ref()
            .map(|input| input.read(cx).text().to_string())
            .unwrap_or_default();

        let name = if name.trim().is_empty() {
            "Untitled Board".to_string()
        } else {
            name.trim().to_string()
        };

        let location = std::mem::take(&mut self.navigation.create_board_location);
        let location_name = location.display_name().to_owned();

        // Create the board with custom location
        let metadata = self.navigation.board_index.create_board_at(name, location);

        // Close modal immediately (no animation when confirming - we're navigating away)
        self.navigation.show_create_board_modal = false;
        self.navigation.create_board_input = None;
        self.ui.modal_animations.create_board = None;
        self.system.focus.release(FocusContext::Modal, window);
        self.ui.toast_manager
            .push(crate::notifications::Toast::success(format!(
                "Board created in {}",
                location_name
            )));

        // Open the new board
        self.open_board(metadata.id, cx);
    }

    /// Quick create (backwards compatible) - creates with default name and location
    pub fn create_new_board(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Show modal instead of directly creating
        self.show_create_board_modal(window, cx);
    }

    pub fn open_board(&mut self, id: String, cx: &mut Context<Self>) {
        self.navigation.board_index.touch_board(&id);
        let board = Board::load(id.clone());
        self.canvas.board = Some(board);
        self.navigation.view = AppView::Board(id);
        cx.notify();
    }

    pub fn go_home(&mut self, cx: &mut Context<Self>) {
        // Force save current board before leaving
        if let Some(ref mut board) = self.canvas.board {
            if let Err(e) = board.flush_save() {
                self.ui.toast_manager
                    .push(crate::notifications::Toast::error(format!(
                        "Save failed: {}",
                        e
                    )).with_action(crate::notifications::ToastAction::retry()));
            }
        }
        self.canvas.board = None;
        // Clean up preview panel resources before dropping
        if let Some(ref mut preview) = self.preview.panel {
            preview.cleanup(cx);
        }
        self.preview.panel = None;
        self.webviews.youtube.clear(); // Clear YouTube WebViews when leaving board
        self.webviews.audio.clear(); // Clear Audio WebViews when leaving board
        self.webviews.video.clear(); // Clear Video WebViews when leaving board
        self.navigation.view = AppView::Landing;
        self.canvas.selected_items.clear();
        // Reload index to get any changes
        self.navigation.board_index = BoardIndex::load();
        cx.notify();
    }

    pub fn start_editing_board(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(meta) = self.navigation.board_index.get_board(&id) {
            // Set focus context to Landing for input
            self.system.focus.focus(FocusContext::Landing, window);

            let name = meta.name.clone();
            let input = cx.new(|cx| InputState::new(window, cx).default_value(name));
            // Focus the input so user can type immediately
            input.update(cx, |state, cx| {
                state.focus(window, cx);
            });
            self.navigation.edit_input = Some(input);
            self.navigation.editing_board_id = Some(id);
            cx.notify();
        }
    }

    pub fn finish_editing_board(&mut self, cx: &mut Context<Self>) {
        if let Some(ref id) = self.navigation.editing_board_id.clone() {
            if let Some(ref input) = self.navigation.edit_input {
                let new_name = input.read(cx).value().to_string();
                if !new_name.trim().is_empty() {
                    self.navigation.board_index.rename_board(id, new_name);
                }
            }
        }
        self.navigation.editing_board_id = None;
        self.navigation.edit_input = None;
        cx.notify();
    }

    pub fn cancel_editing(&mut self, cx: &mut Context<Self>) {
        self.navigation.editing_board_id = None;
        self.navigation.edit_input = None;
        cx.notify();
    }

    pub fn confirm_delete_board(&mut self, id: impl Into<String>, cx: &mut Context<Self>) {
        self.navigation.deleting_board_id = Some(id.into());
        cx.notify();
    }

    /// Soft delete - moves to trash
    pub fn delete_board(&mut self, id: &str, cx: &mut Context<Self>) {
        self.navigation.board_index.delete_board(id);
        self.navigation.deleting_board_id = None;
        self.ui.toast_manager
            .push(crate::notifications::Toast::info("Board moved to trash"));
        cx.notify();
    }

    pub fn cancel_delete(&mut self, cx: &mut Context<Self>) {
        self.navigation.deleting_board_id = None;
        cx.notify();
    }

    /// Restore a board from trash
    pub fn restore_board(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.navigation.board_index.restore_board(id) {
            self.ui.toast_manager
                .push(crate::notifications::Toast::success("Board restored"));
        }
        cx.notify();
    }

    /// Permanently delete a board (no recovery)
    pub fn permanently_delete_board(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.navigation.board_index.permanently_delete_board(id) {
            self.ui.toast_manager.push(crate::notifications::Toast::info(
                "Board permanently deleted",
            ));
        }
        cx.notify();
    }

    /// Empty all boards from trash
    pub fn empty_trash(&mut self, cx: &mut Context<Self>) {
        let count = self.navigation.board_index.empty_trash();
        if count > 0 {
            self.ui.toast_manager
                .push(crate::notifications::Toast::info(format!(
                    "Permanently deleted {} board(s)",
                    count
                )));
        }
        // Hide trash section if empty
        if self.navigation.board_index.trashed_boards().is_empty() {
            self.navigation.show_trash = false;
        }
        cx.notify();
    }

    /// Toggle trash section visibility
    pub fn toggle_trash(&mut self, cx: &mut Context<Self>) {
        // Don't toggle if a modal is open
        if self.settings.show || self.navigation.show_create_board_modal {
            return;
        }
        self.navigation.show_trash = !self.navigation.show_trash;
        cx.notify();
    }

    // ==================== Onboarding Methods ====================

    /// Complete onboarding and transition to landing page
    pub fn complete_onboarding(&mut self, cx: &mut Context<Self>) {
        // Mark onboarding as completed in settings
        if let Err(e) = crate::settings::mark_onboarding_completed() {
            tracing::error!("Failed to mark onboarding completed: {}", e);
        }

        // Transition to landing page
        self.navigation.view = AppView::Landing;
        cx.notify();
    }
}
