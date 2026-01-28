//! Data parsing and handling module
//!
//! This module provides parsers for various data formats that can be
//! used to populate tables and charts on the canvas.
//!
//! ## Performance
//!
//! For large datasets (10k+ rows), use `LazyDataSource` with polars backend
//! which provides:
//! - Lazy evaluation (only loads what's needed)
//! - Virtual scrolling support
//! - Chunk caching for smooth scroll performance
//!
//! ## Error Handling
//!
//! All data operations return `DataResult<T>` which uses the `DataError` type.
//! Common errors include:
//! - `TooLarge`: File exceeds size limits
//! - `TooManyRows`: Dataset exceeds row limits
//! - `Io`: File system errors
//! - `Csv`/`Json`: Parse errors

mod chart_engine;
mod csv_parser;
mod error;
mod json_parser;
mod lazy_source;
mod table_delegate;

pub use chart_engine::*;
pub use csv_parser::*;
pub use error::*;
pub use json_parser::*;
pub use lazy_source::*;
pub use table_delegate::*;
