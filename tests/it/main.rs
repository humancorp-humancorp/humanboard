//! Single test binary entry point.
//!
//! This consolidates all tests into a single binary following matklad's best practices,
//! reducing linking overhead from 3x to 1x.
//!
//! Structure:
//! - board: Board module tests (undo/redo, items, history)
//! - integration: Multi-component workflow tests
//! - unit: Single-component unit tests
//! - helpers: Test utilities, builders, and fixtures

mod board;
mod helpers;
mod integration;
mod unit;

// Re-export helpers for use in test modules
#[allow(unused_imports)]
pub use helpers::*;
