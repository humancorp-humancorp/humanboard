//! Mouse up event handling - finalize operations, create drawn items.

use crate::app::Humanboard;
use crate::constants::{DEFAULT_FONT_SIZE, HEADER_HEIGHT, MIN_ARROW_SIZE, MIN_DRAW_DISTANCE, MIN_MARQUEE_SIZE};
use crate::input::coords::{CoordinateContext, CoordinateConverter};
use crate::types::{ArrowHead, DataSource, ItemContent, ShapeType, ToolType};
use gpui::*;

impl Humanboard {
    pub fn handle_mouse_up(
        &mut self,
        event: &MouseUpEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Only push history on mouse up if we were dragging/resizing
        let was_modifying = self.canvas.input_state.is_dragging() || self.canvas.input_state.is_resizing();

        if was_modifying {
            if let Some(ref mut board) = self.canvas.board {
                // Update spatial index for modified items
                if let Some(resize_id) = self.canvas.input_state.resizing_item() {
                    board.update_spatial_index(resize_id);
                } else {
                    // Update spatial index for all dragged items
                    for &item_id in &self.canvas.selected_items {
                        board.update_spatial_index(item_id);
                    }
                }

                board.push_history();
                if let Err(e) = board.flush_save() {
                    self.ui.toast_manager
                        .push(crate::notifications::Toast::error(format!(
                            "Save failed: {}",
                            e
                        )));
                }
            }
        }

        // Finalize marquee selection using spatial index for O(log n + k) query
        if let (Some(start), Some(end)) = (self.canvas.input_state.marquee_start(), self.canvas.input_state.marquee_current()) {
            if let Some(ref board) = self.canvas.board {
                let min_x = f32::from(start.x).min(f32::from(end.x));
                let max_x = f32::from(start.x).max(f32::from(end.x));
                let min_y = f32::from(start.y).min(f32::from(end.y));
                let max_y = f32::from(start.y).max(f32::from(end.y));

                // Only select if marquee has some size (not just a click)
                if (max_x - min_x) > MIN_MARQUEE_SIZE || (max_y - min_y) > MIN_MARQUEE_SIZE {
                    // Convert screen coordinates to canvas coordinates
                    let ctx = CoordinateContext::new(&board.canvas_offset, board.zoom);
                    let canvas_min = CoordinateConverter::screen_to_canvas(point(px(min_x), px(min_y)), &ctx);
                    let canvas_max = CoordinateConverter::screen_to_canvas(point(px(max_x), px(max_y)), &ctx);
                    let canvas_min_x = f32::from(canvas_min.x);
                    let canvas_max_x = f32::from(canvas_max.x);
                    let canvas_min_y = f32::from(canvas_min.y);
                    let canvas_max_y = f32::from(canvas_max.y);

                    // Query spatial index for items in rectangle
                    let intersecting_ids = board.query_items_in_rect(
                        canvas_min_x, canvas_min_y, canvas_max_x, canvas_max_y
                    );

                    for item_id in intersecting_ids {
                        if event.modifiers.shift {
                            if self.canvas.selected_items.contains(&item_id) {
                                self.canvas.selected_items.remove(&item_id);
                            } else {
                                self.canvas.selected_items.insert(item_id);
                            }
                        } else {
                            self.canvas.selected_items.insert(item_id);
                        }
                    }
                }
            }
        }

        // Finalize arrow/shape/text drawing
        if let Some(start) = self.tools.drawing_start {
            let end = event.position;
            let header_offset = HEADER_HEIGHT;

            let screen_width = (f32::from(end.x) - f32::from(start.x)).abs();
            let screen_height = (f32::from(end.y) - f32::from(start.y)).abs();

            // Only create if dragged at least MIN_DRAW_DISTANCE pixels
            if screen_width < MIN_DRAW_DISTANCE && screen_height < MIN_DRAW_DISTANCE {
                self.tools.drawing_start = None;
                self.tools.drawing_current = None;
                self.tools.selected = ToolType::Select;
                cx.notify();
                return;
            }

            let start_canvas = self.screen_to_canvas(start, header_offset);
            let end_canvas = self.screen_to_canvas(end, header_offset);

            let start_x = f32::from(start_canvas.x);
            let start_y = f32::from(start_canvas.y);
            let end_x = f32::from(end_canvas.x);
            let end_y = f32::from(end_canvas.y);

            let width = (end_x - start_x).abs().max(MIN_ARROW_SIZE);
            let height = (end_y - start_y).abs().max(MIN_ARROW_SIZE);
            let pos_x = start_x.min(end_x);
            let pos_y = start_y.min(end_y);

            match self.tools.selected {
                ToolType::Arrow => {
                    if let Some(ref mut board) = self.canvas.board {
                        let box_x = start_x.min(end_x);
                        let box_y = start_y.min(end_y);
                        let box_w = (end_x - start_x).abs().max(MIN_ARROW_SIZE);
                        let box_h = (end_y - start_y).abs().max(MIN_ARROW_SIZE);

                        let arrow_start = (start_x - box_x, start_y - box_y);
                        let arrow_end = (end_x - box_x, end_y - box_y);
                        let end_offset = (arrow_end.0 - arrow_start.0, arrow_end.1 - arrow_start.1);

                        let id = board.add_item(
                            point(px(box_x), px(box_y)),
                            ItemContent::Arrow {
                                end_offset,
                                color: "".to_string(),
                                thickness: 2.0,
                                head_style: ArrowHead::Arrow,
                            },
                        );
                        if let Some(item) = board.get_item_mut(id) {
                            item.size = (box_w, box_h);
                        }
                        self.canvas.selected_items.clear();
                        self.canvas.selected_items.insert(id);
                    }
                }
                ToolType::Shape => {
                    if let Some(ref mut board) = self.canvas.board {
                        let id = board.add_item(
                            point(px(pos_x), px(pos_y)),
                            ItemContent::Shape {
                                shape_type: ShapeType::Rectangle,
                                fill_color: None,
                                border_color: "".to_string(),
                                border_width: 2.0,
                            },
                        );
                        if let Some(item) = board.get_item_mut(id) {
                            item.size = (width, height);
                        }
                        self.canvas.selected_items.clear();
                        self.canvas.selected_items.insert(id);
                    }
                }
                ToolType::Text => {
                    if let Some(ref mut board) = self.canvas.board {
                        let id = board.add_item(
                            point(px(pos_x), px(pos_y)),
                            ItemContent::TextBox {
                                text: "".to_string(),
                                font_size: DEFAULT_FONT_SIZE,
                                color: "".to_string(),
                            },
                        );
                        if let Some(item) = board.get_item_mut(id) {
                            item.size = (width.max(100.0), height.max(40.0));
                        }
                        self.canvas.selected_items.clear();
                        self.canvas.selected_items.insert(id);
                        self.start_textbox_editing(id, window, cx);
                    }
                }
                ToolType::Table => {
                    if let Some(ref mut board) = self.canvas.board {
                        // Create an empty data source for manual data entry
                        let ds = DataSource::new_empty(
                            board.next_data_source_id,
                            "New Table".to_string(),
                        );
                        let ds_id = ds.id;
                        board.data_sources.insert(ds_id, ds);
                        board.next_data_source_id += 1;

                        // Create the table item
                        let id = board.add_item(
                            point(px(pos_x), px(pos_y)),
                            ItemContent::Table {
                                data_source_id: ds_id,
                                show_headers: true,
                                stripe: true,
                            },
                        );
                        if let Some(item) = board.get_item_mut(id) {
                            item.size = (width.max(300.0), height.max(200.0));
                        }
                        self.canvas.selected_items.clear();
                        self.canvas.selected_items.insert(id);
                    }
                }
                ToolType::Chart => {
                    // Charts are created from tables, not directly
                    // This case is kept for backwards compatibility but does nothing
                }
                _ => {}
            }

            self.tools.selected = ToolType::Select;
            self.tools.drawing_start = None;
            self.tools.drawing_current = None;
        }

        // Reset all drag/resize state
        self.canvas.input_state.reset();
        cx.notify();
    }
}
