//! Table cell editing and state management methods

use crate::app::Humanboard;
use crate::data::DataSourceDelegate;
use crate::types::{DataCell, ItemContent};
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::table::TableState;
use std::sync::Arc;

impl Humanboard {
    /// Start editing a table cell
    pub fn start_table_cell_editing(
        &mut self,
        table_id: u64,
        row: usize,
        col: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Get current cell value
        let current_value = if let Some(ref board) = self.canvas.board {
            if let Some(item) = board.items.iter().find(|i| i.id == table_id) {
                if let ItemContent::Table { data_source_id, .. } = &item.content {
                    if let Some(ds) = board.data_sources.get(data_source_id) {
                        ds.rows.get(row)
                            .and_then(|r| r.cells.get(col))
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Create input state with current value
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(current_value.clone())
        });

        self.table.editing_cell = Some((table_id, row, col));
        self.table.cell_input = Some(input_state.clone());
        self.table.editing_started_at = Some(std::time::Instant::now());

        // Focus the input after it's mounted
        // Use multiple deferred calls to ensure focus happens after all mouse events
        let input_clone = input_state.clone();
        window.defer(cx, move |window, cx| {
            window.defer(cx, move |window, cx| {
                window.defer(cx, move |window, cx| {
                    input_clone.update(cx, |state, cx| {
                        state.focus(window, cx);
                    });
                });
            });
        });

        // Subscribe to input events
        cx.subscribe(
            &input_state,
            |this, _input, event: &gpui_component::input::InputEvent, cx| {
                match event {
                    gpui_component::input::InputEvent::PressEnter { .. } => {
                        // Save on Enter
                        this.finish_table_cell_editing(cx);
                    }
                    gpui_component::input::InputEvent::Blur => {
                        // Save on blur (click outside), but ignore blur events that happen
                        // immediately after starting editing (within 100ms)
                        const MIN_EDIT_DURATION: std::time::Duration = std::time::Duration::from_millis(100);
                        let should_save = this.table.editing_started_at
                            .map(|t| t.elapsed() >= MIN_EDIT_DURATION)
                            .unwrap_or(true);
                        if should_save {
                            this.finish_table_cell_editing(cx);
                        }
                    }
                    _ => {}
                }
            },
        )
        .detach();

        cx.notify();
    }

    /// Finish editing a table cell and save the value
    pub fn finish_table_cell_editing(&mut self, cx: &mut Context<Self>) {
        let Some((table_id, row, col)) = self.table.editing_cell.take() else {
            return;
        };

        let Some(input) = self.table.cell_input.take() else {
            return;
        };
        
        // Clear the editing timestamp
        self.table.editing_started_at = None;

        // Get the new value from the input
        let new_value = input.read(cx).text().to_string();

        // Update the data source
        let mut updated_ds_id = None;
        if let Some(ref mut board) = self.canvas.board {
            // Find the data source ID from the table item
            let data_source_id = board.items.iter()
                .find(|i| i.id == table_id)
                .and_then(|item| {
                    if let ItemContent::Table { data_source_id, .. } = &item.content {
                        Some(*data_source_id)
                    } else {
                        None
                    }
                });

            if let Some(ds_id) = data_source_id {
                if let Some(ds) = board.data_sources.get_mut(&ds_id) {
                    // Update the cell
                    if let Some(data_row) = ds.rows.get_mut(row) {
                        if let Some(cell) = data_row.cells.get_mut(col) {
                            // Try to parse as number, otherwise keep as text
                            *cell = if let Ok(n) = new_value.parse::<f64>() {
                                DataCell::Number(n)
                            } else if new_value.is_empty() {
                                DataCell::Empty
                            } else {
                                DataCell::Text(new_value)
                            };
                        }
                    }
                    // Mark data source as dirty (has unsaved changes to file)
                    ds.mark_dirty();
                }

                // Mark as modified
                board.push_history();
                let _ = board.flush_save();
                
                updated_ds_id = Some(ds_id);
            }
        }
        
        // Sync updated data source to preview panel if it's open
        if let Some(ds_id) = updated_ds_id {
            if let Some(ref board) = self.canvas.board {
                if let Some(ds) = board.data_sources.get(&ds_id) {
                    self.sync_data_source_to_preview(ds_id, ds.clone(), cx);
                }
            }
        }

        cx.notify();
    }
    
    /// Sync a data source update to any open preview tabs
    fn sync_data_source_to_preview(&mut self, data_source_id: u64, data_source: crate::types::DataSource, cx: &mut Context<Self>) {
        use crate::app::PreviewTab;
        
        if let Some(ref mut preview) = self.preview.panel {
            // Sync to left pane tabs
            for tab in preview.tabs.iter_mut() {
                if let PreviewTab::Table { data_source_id: id, table_state: Some(state), .. } = tab {
                    if *id == data_source_id {
                        state.update(cx, |table_state, _cx| {
                            table_state.delegate_mut().set_data_source(std::sync::Arc::new(data_source.clone()));
                        });
                    }
                }
            }
            
            // Sync to right pane tabs
            for tab in preview.right_tabs.iter_mut() {
                if let PreviewTab::Table { data_source_id: id, table_state: Some(state), .. } = tab {
                    if *id == data_source_id {
                        state.update(cx, |table_state, _cx| {
                            table_state.delegate_mut().set_data_source(std::sync::Arc::new(data_source.clone()));
                        });
                    }
                }
            }
        }
    }

    /// Cancel table cell editing without saving
    pub fn cancel_table_cell_editing(&mut self, cx: &mut Context<Self>) {
        self.table.editing_cell = None;
        self.table.cell_input = None;
        self.table.editing_started_at = None;
        cx.notify();
    }

    /// Ensure TableState entities exist for all table items.
    /// Call this before rendering to ensure all tables have their state initialized.
    /// Takes zoom to calculate actual pixel widths for columns.
    pub fn ensure_table_states(&mut self, zoom: f32, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ref board) = self.canvas.board else {
            return;
        };

        // Collect table items that need states (with their size for column width calculation)
        // Use actual pixel width (canvas size * zoom)
        let tables_needing_state: Vec<(u64, u64, f32)> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::Table { data_source_id, .. } = &item.content {
                    if !self.table.table_states.contains_key(&item.id) {
                        Some((item.id, *data_source_id, item.size.0 * zoom))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Create states for tables that need them
        for (table_id, data_source_id, pixel_width) in tables_needing_state {
            if let Some(ds) = board.data_sources.get(&data_source_id) {
                let delegate = DataSourceDelegate::with_width(Arc::new(ds.clone()), pixel_width);
                let state = cx.new(|cx| TableState::new(delegate, window, cx));
                self.table.table_states.insert(table_id, state);
            }
        }

        // Update column widths for existing tables if their size or zoom changed
        let existing_tables: Vec<(u64, f32)> = board
            .items
            .iter()
            .filter_map(|item| {
                if matches!(&item.content, ItemContent::Table { .. }) {
                    Some((item.id, item.size.0 * zoom))
                } else {
                    None
                }
            })
            .collect();

        for (table_id, pixel_width) in &existing_tables {
            if let Some(state) = self.table.table_states.get(table_id) {
                state.update(cx, |table_state, _cx| {
                    table_state.delegate_mut().set_container_width(*pixel_width);
                });
            }
        }

        // Clean up states for tables that no longer exist
        let existing_table_ids: std::collections::HashSet<u64> = existing_tables.iter().map(|(id, _)| *id).collect();
        self.table.table_states.retain(|id, _| existing_table_ids.contains(id));
    }

    /// Update the data source for a specific table's TableState.
    /// Call this when the underlying data changes.
    pub fn update_table_data(&mut self, table_id: u64, cx: &mut Context<Self>) {
        let Some(ref board) = self.canvas.board else {
            return;
        };

        // Find the table item and its data source
        let data_source = board.items.iter().find_map(|item| {
            if item.id == table_id {
                if let ItemContent::Table { data_source_id, .. } = &item.content {
                    board.data_sources.get(data_source_id).cloned()
                } else {
                    None
                }
            } else {
                None
            }
        });

        if let Some(ds) = data_source {
            if let Some(state) = self.table.table_states.get(&table_id) {
                state.update(cx, |table_state, _cx| {
                    table_state.delegate_mut().set_data_source(Arc::new(ds));
                });
            }
        }
    }
}
