//! Coordinate conversion utilities for canvas interactions.
//!
//! This module provides centralized coordinate conversion functions to eliminate
//! duplicated formulas across input handling code.

use crate::constants::{DOCK_WIDTH, HEADER_HEIGHT};
use gpui::{Point, Pixels, point, px};

/// Context needed for coordinate conversions
pub struct CoordinateContext<'a> {
    pub canvas_offset: &'a Point<Pixels>,
    pub zoom: f32,
}

impl<'a> CoordinateContext<'a> {
    /// Create a new coordinate context
    #[inline]
    pub fn new(canvas_offset: &'a Point<Pixels>, zoom: f32) -> Self {
        Self {
            canvas_offset,
            zoom,
        }
    }
}

pub struct CoordinateConverter;

impl CoordinateConverter {
    /// Convert screen position to canvas position
    #[inline]
    pub fn screen_to_canvas(screen_pos: Point<Pixels>, ctx: &CoordinateContext<'_>) -> Point<Pixels> {
        point(
            px((f32::from(screen_pos.x) - DOCK_WIDTH - f32::from(ctx.canvas_offset.x)) / ctx.zoom),
            px((f32::from(screen_pos.y) - HEADER_HEIGHT - f32::from(ctx.canvas_offset.y)) / ctx.zoom),
        )
    }

    /// Convert canvas position to screen position
    #[inline]
    pub fn canvas_to_screen(canvas_pos: Point<Pixels>, ctx: &CoordinateContext<'_>) -> Point<Pixels> {
        point(
            px(f32::from(canvas_pos.x) * ctx.zoom + f32::from(ctx.canvas_offset.x) + DOCK_WIDTH),
            px(f32::from(canvas_pos.y) * ctx.zoom + f32::from(ctx.canvas_offset.y) + HEADER_HEIGHT),
        )
    }

    /// Convert a delta from screen to canvas (for drag operations)
    #[inline]
    pub fn delta_screen_to_canvas(delta: Point<Pixels>, zoom: f32) -> Point<Pixels> {
        point(
            px(f32::from(delta.x) / zoom),
            px(f32::from(delta.y) / zoom),
        )
    }

    /// Convert a delta from canvas to screen
    #[inline]
    pub fn delta_canvas_to_screen(delta: Point<Pixels>, zoom: f32) -> Point<Pixels> {
        point(
            px(f32::from(delta.x) * zoom),
            px(f32::from(delta.y) * zoom),
        )
    }
}
