//! Mouse and scroll input handling for the canvas.
//!
//! This module implements all mouse interaction logic for the Humanboard canvas,
//! including item selection, dragging, resizing, and drawing tools.
//!
//! ## Architecture
//!
//! The input system uses an explicit state machine (`InputState`) to track
//! the current interaction mode. This replaces scattered boolean flags and
//! makes impossible states unrepresentable.
//!
//! ## Modules
//!
//! - `state` - Input state machine enum and helper methods
//! - `mouse_down` - Mouse down event handling (selection, drag/resize start)
//! - `mouse_up` - Mouse up event handling (finalize operations, create items)
//! - `drag` - Mouse move handling (drag, resize, pan operations)
//! - `transform` - Canvas transformations (scroll, zoom, coordinate conversion)

pub mod coords;
mod state;
mod mouse_down;
mod mouse_up;
mod drag;
mod transform;

pub use state::{InputState, SplitterDirection};
