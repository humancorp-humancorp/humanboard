//! Input state machine - unified state management for all input interactions.
//!
//! This module replaces scattered boolean flags with a single explicit state machine,
//! making impossible states unrepresentable and improving code clarity.
//!
//! ## State Transitions
//!
//! ```text
//! Idle -> Panning              (middle mouse down on canvas)
//! Idle -> DraggingItems        (mouse down on selected item, not on resize corner)
//! Idle -> ResizingItem         (mouse down on item resize corner)
//! Idle -> MarqueeSelecting     (mouse down on empty canvas with select tool)
//! Idle -> Drawing              (mouse down with arrow/shape/text tool)
//! Idle -> SplitterDragging     (mouse down on preview splitter)
//!
//! Any -> Idle                  (mouse up - finalizes operation)
//! ```

use crate::app::SplitDirection;
use crate::types::ToolType;
use gpui::{Point, Pixels};

/// Unified input state for all mouse interactions.
///
/// Replaces the previous scattered boolean flags:
/// - `dragging: bool` -> `InputState::Panning`
/// - `dragging_item: Option<u64>` -> `InputState::DraggingItems`
/// - `resizing_item: Option<u64>` -> `InputState::ResizingItem`
/// - `dragging_splitter: bool` -> `InputState::SplitterDragging`
/// - `dragging_pane_splitter: bool` -> `InputState::SplitterDragging` with direction
/// - `marquee_start/marquee_current` -> `InputState::MarqueeSelecting`
/// - `drawing_start/drawing_current` -> `InputState::Drawing`
#[derive(Debug, Clone)]
pub enum InputState {
    /// No active input operation
    Idle,

    /// Canvas panning (middle mouse or space+drag)
    Panning {
        /// Last mouse position for delta calculation
        last_pos: Point<Pixels>,
    },

    /// Dragging one or more items
    DraggingItems {
        /// Primary item being dragged (the one under the cursor)
        primary_item: u64,
        /// Offset from item top-left to cursor position
        drag_offset: Point<Pixels>,
    },

    /// Resizing an item
    ResizingItem {
        /// Item being resized
        item_id: u64,
        /// Original size at start of resize
        start_size: (f32, f32),
        /// Mouse position at start of resize
        start_pos: Point<Pixels>,
        /// Original font size for text boxes (to scale with resize)
        original_font_size: Option<f32>,
    },

    /// Marquee/box selection
    MarqueeSelecting {
        /// Selection box start position
        start: Point<Pixels>,
        /// Current mouse position
        current: Point<Pixels>,
    },

    /// Drawing shapes, arrows, or text boxes
    Drawing {
        /// Tool being used
        tool: ToolType,
        /// Drawing start position
        start: Point<Pixels>,
        /// Current mouse position
        current: Point<Pixels>,
    },

    /// Dragging a splitter (canvas/preview or pane split)
    SplitterDragging {
        /// Direction of the splitter
        direction: SplitterDirection,
        /// Mouse position at start of drag
        drag_start: Point<Pixels>,
    },
}

impl Default for InputState {
    fn default() -> Self {
        Self::Idle
    }
}

/// Direction of splitter drag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitterDirection {
    /// Canvas/preview panel splitter (vertical or horizontal)
    CanvasPreview { direction: SplitDirection },
    /// Pane splitter between split preview panes
    PaneSplit { horizontal: bool },
}

impl InputState {
    /// Returns true if any drag operation is active
    pub fn is_dragging(&self) -> bool {
        matches!(
            self,
            Self::Panning { .. }
                | Self::DraggingItems { .. }
                | Self::ResizingItem { .. }
                | Self::SplitterDragging { .. }
        )
    }

    /// Returns true if currently resizing an item
    pub fn is_resizing(&self) -> bool {
        matches!(self, Self::ResizingItem { .. })
    }

    /// Returns true if currently dragging items
    pub fn is_dragging_items(&self) -> bool {
        matches!(self, Self::DraggingItems { .. })
    }

    /// Returns true if currently marquee selecting
    pub fn is_marquee_selecting(&self) -> bool {
        matches!(self, Self::MarqueeSelecting { .. })
    }

    /// Returns true if currently drawing
    pub fn is_drawing(&self) -> bool {
        matches!(self, Self::Drawing { .. })
    }

    /// Get the drawing start position, if drawing
    pub fn drawing_start(&self) -> Option<Point<Pixels>> {
        match self {
            Self::Drawing { start, .. } => Some(*start),
            _ => None,
        }
    }

    /// Get the drawing current position, if drawing
    pub fn drawing_current(&self) -> Option<Point<Pixels>> {
        match self {
            Self::Drawing { current, .. } => Some(*current),
            _ => None,
        }
    }

    /// Returns true if currently panning the canvas
    pub fn is_panning(&self) -> bool {
        matches!(self, Self::Panning { .. })
    }

    /// Returns true if currently dragging any splitter
    pub fn is_splitter_dragging(&self) -> bool {
        matches!(self, Self::SplitterDragging { .. })
    }

    /// Get the item ID being dragged, if any
    pub fn dragged_item_id(&self) -> Option<u64> {
        match self {
            Self::DraggingItems { primary_item, .. } => Some(*primary_item),
            _ => None,
        }
    }

    /// Get the item ID being resized, if any
    pub fn resized_item_id(&self) -> Option<u64> {
        match self {
            Self::ResizingItem { item_id, .. } => Some(*item_id),
            _ => None,
        }
    }

    /// Returns true if the state is Idle
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Reset to Idle state
    pub fn reset(&mut self) {
        *self = Self::Idle;
    }

    /// Start dragging items
    pub fn start_dragging(&mut self, item_id: u64, offset: Point<Pixels>) {
        *self = Self::DraggingItems {
            primary_item: item_id,
            drag_offset: offset,
        };
    }

    /// Start resizing an item
    pub fn start_resizing(
        &mut self,
        item_id: u64,
        start_size: (f32, f32),
        start_pos: Point<Pixels>,
        original_font_size: Option<f32>,
    ) {
        *self = Self::ResizingItem {
            item_id,
            start_size,
            start_pos,
            original_font_size,
        };
    }

    /// Start splitter dragging
    pub fn start_splitter_drag(&mut self, drag_start: Point<Pixels>, direction: SplitDirection) {
        *self = Self::SplitterDragging {
            direction: SplitterDirection::CanvasPreview { direction },
            drag_start,
        };
    }

    /// Start pane splitter dragging
    pub fn start_pane_splitter_drag(&mut self, drag_start: Point<Pixels>, horizontal: bool) {
        *self = Self::SplitterDragging {
            direction: SplitterDirection::PaneSplit { horizontal },
            drag_start,
        };
    }

    /// Start marquee selection
    pub fn start_marquee(&mut self, start: Point<Pixels>) {
        *self = Self::MarqueeSelecting { start, current: start };
    }

    /// Update marquee current position
    pub fn set_marquee_current(&mut self, current: Point<Pixels>) {
        if let Self::MarqueeSelecting { current: c, .. } = self {
            *c = current;
        }
    }

    /// Get marquee start position
    pub fn marquee_start(&self) -> Option<Point<Pixels>> {
        match self {
            Self::MarqueeSelecting { start, .. } => Some(*start),
            _ => None,
        }
    }

    /// Get marquee current position
    pub fn marquee_current(&self) -> Option<Point<Pixels>> {
        match self {
            Self::MarqueeSelecting { current, .. } => Some(*current),
            _ => None,
        }
    }

    /// Get the item ID being resized
    pub fn resizing_item(&self) -> Option<u64> {
        match self {
            Self::ResizingItem { item_id, .. } => Some(*item_id),
            _ => None,
        }
    }

    /// Get resize start size
    pub fn resize_start_size(&self) -> Option<(f32, f32)> {
        match self {
            Self::ResizingItem { start_size, .. } => Some(*start_size),
            _ => None,
        }
    }

    /// Get resize start position
    pub fn resize_start_pos(&self) -> Option<Point<Pixels>> {
        match self {
            Self::ResizingItem { start_pos, .. } => Some(*start_pos),
            _ => None,
        }
    }

    /// Get original font size for resize
    pub fn resize_start_font_size(&self) -> Option<f32> {
        match self {
            Self::ResizingItem { original_font_size, .. } => *original_font_size,
            _ => None,
        }
    }

    /// Get drag offset
    pub fn drag_offset(&self) -> Option<Point<Pixels>> {
        match self {
            Self::DraggingItems { drag_offset, .. } => Some(*drag_offset),
            _ => None,
        }
    }

    /// Get dragging item ID (alias for dragged_item_id)
    pub fn dragging_item(&self) -> Option<u64> {
        self.dragged_item_id()
    }

    /// Returns true if currently dragging the canvas/preview splitter
    pub fn is_canvas_splitter_dragging(&self) -> bool {
        match self {
            Self::SplitterDragging { direction, .. } => matches!(direction, SplitterDirection::CanvasPreview { .. }),
            _ => false,
        }
    }

    /// Returns true if currently dragging the pane splitter
    pub fn is_pane_splitter_dragging(&self) -> bool {
        match self {
            Self::SplitterDragging { direction, .. } => matches!(direction, SplitterDirection::PaneSplit { .. }),
            _ => false,
        }
    }

    /// Returns true if currently panning the canvas (middle mouse drag)
    pub fn is_canvas_panning(&self) -> bool {
        matches!(self, Self::Panning { .. })
    }

    /// Get last mouse position (for panning)
    pub fn last_mouse_pos(&self) -> Option<Point<Pixels>> {
        match self {
            Self::Panning { last_pos } => Some(*last_pos),
            _ => None,
        }
    }

    /// Update last mouse position (for panning)
    pub fn update_last_mouse_pos(&mut self, pos: Point<Pixels>) {
        if let Self::Panning { last_pos } = self {
            *last_pos = pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_is_idle() {
        let state: InputState = Default::default();
        assert!(state.is_idle());
        assert!(!state.is_dragging());
    }

    #[test]
    fn test_is_dragging_variants() {
        let pos = Point::new(gpui::px(0.0), gpui::px(0.0));

        assert!(!InputState::Idle.is_dragging());
        assert!(InputState::Panning { last_pos: pos }.is_dragging());
        assert!(
            InputState::DraggingItems {
                primary_item: 1,
                drag_offset: pos,
            }
            .is_dragging()
        );
        assert!(
            InputState::ResizingItem {
                item_id: 1,
                start_size: (100.0, 100.0),
                start_pos: pos,
                original_font_size: None,
            }
            .is_dragging()
        );
        assert!(
            InputState::SplitterDragging {
                direction: SplitterDirection::CanvasPreview {
                    direction: SplitDirection::Vertical,
                },
                drag_start: pos,
            }
            .is_dragging()
        );

        // Non-dragging states
        assert!(!InputState::MarqueeSelecting { start: pos, current: pos }.is_dragging());
        assert!(
            !InputState::Drawing {
                tool: ToolType::Arrow,
                start: pos,
                current: pos,
            }
            .is_dragging()
        );
    }

    #[test]
    fn test_state_queries() {
        let pos = Point::new(gpui::px(0.0), gpui::px(0.0));

        assert!(InputState::Panning { last_pos: pos }.is_panning());
        assert!(
            InputState::DraggingItems {
                primary_item: 1,
                drag_offset: pos,
            }
            .is_dragging_items()
        );
        assert!(
            InputState::ResizingItem {
                item_id: 1,
                start_size: (100.0, 100.0),
                start_pos: pos,
                original_font_size: None,
            }
            .is_resizing()
        );
        assert!(InputState::MarqueeSelecting { start: pos, current: pos }.is_marquee_selecting());
        assert!(
            InputState::Drawing {
                tool: ToolType::Shape,
                start: pos,
                current: pos,
            }
            .is_drawing()
        );
    }

    #[test]
    fn test_item_id_extraction() {
        let pos = Point::new(gpui::px(0.0), gpui::px(0.0));

        let drag_state = InputState::DraggingItems {
            primary_item: 42,
            drag_offset: pos,
        };
        assert_eq!(drag_state.dragged_item_id(), Some(42));
        assert_eq!(drag_state.resized_item_id(), None);

        let resize_state = InputState::ResizingItem {
            item_id: 99,
            start_size: (100.0, 100.0),
            start_pos: pos,
            original_font_size: None,
        };
        assert_eq!(resize_state.resized_item_id(), Some(99));
        assert_eq!(resize_state.dragged_item_id(), None);
    }

    #[test]
    fn test_reset() {
        let pos = Point::new(gpui::px(0.0), gpui::px(0.0));
        let mut state = InputState::Panning { last_pos: pos };

        state.reset();
        assert!(state.is_idle());
    }
}
