//! Error types for data operations
//!
//! Provides unified error handling for all data loading and parsing operations.

use thiserror::Error;

// Re-export CSV limits from constants module for consistency
pub use crate::constants::{MAX_CSV_ROWS, MAX_CSV_SIZE_MB};

/// Errors that can occur during data operations
#[derive(Error, Debug)]
pub enum DataError {
    /// IO error from std::io
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// CSV parsing error
    #[error("CSV parse error: {0}")]
    Csv(String),

    /// JSON parsing error from serde_json
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    /// File is too large for eager loading
    #[error("File too large: {size_mb}MB (max {max_mb}MB)")]
    TooLarge { size_mb: u64, max_mb: usize },

    /// Too many rows for eager loading
    #[error("Too many rows: {rows} (max {max_rows})")]
    TooManyRows { rows: usize, max_rows: usize },

    /// File is empty
    #[error("Empty file")]
    EmptyFile,

    /// No columns found in data
    #[error("No columns found")]
    NoColumns,

    /// Invalid data format
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Polars error
    #[error("Polars error: {0}")]
    Polars(#[from] polars::error::PolarsError),

    /// Generic error message
    #[error("{0}")]
    Other(String),
}

/// Result type alias for data operations
pub type DataResult<T> = Result<T, DataError>;

impl From<String> for DataError {
    fn from(s: String) -> Self {
        DataError::Other(s)
    }
}

impl From<&str> for DataError {
    fn from(s: &str) -> Self {
        DataError::Other(s.to_string())
    }
}
