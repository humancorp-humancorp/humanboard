//! Single test binary entry point.
//!
//! This consolidates all tests into a single binary following matklad's best practices,
//! reducing linking overhead from 3x to 1x.
//!
//! Structure:
//! - board: Board module tests (undo/redo, items, history)
//! - integration: Multi-component workflow tests
//! - unit: Single-component unit tests

mod board;
mod integration;
mod unit;
