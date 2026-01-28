//! Range request parsing utilities for HTTP media servers
//!
//! Provides safe parsing and validation of HTTP Range headers for audio/video streaming.

use std::ops::RangeInclusive;

/// Maximum safe range size (100MB) to prevent memory exhaustion
pub const MAX_RANGE_SIZE: u64 = 100 * 1024 * 1024;

/// Parsed and validated byte range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    /// Start byte (inclusive)
    pub start: u64,
    /// End byte (inclusive)
    pub end: u64,
    /// Total content length
    pub total_size: u64,
}

impl ByteRange {
    /// Create a new validated byte range
    ///
    /// Returns None if the range is invalid (end < start, or exceeds bounds)
    pub fn new(start: u64, end: u64, total_size: u64) -> Option<Self> {
        // Validate: end must be >= start
        if end < start {
            return None;
        }

        // Validate: start must be within file bounds
        if start >= total_size {
            return None;
        }

        // Clamp end to file size - 1
        let end = end.min(total_size - 1);

        // Validate: range size must not exceed maximum
        let range_size = end.saturating_sub(start).saturating_add(1);
        if range_size > MAX_RANGE_SIZE {
            return None;
        }

        Some(Self {
            start,
            end,
            total_size,
        })
    }

    /// Parse a Range header value like "bytes=0-1023"
    ///
    /// Returns None if parsing fails or range is invalid
    pub fn parse_header(range_header: &str, file_size: u64) -> Option<Self> {
        let range_spec = range_header.strip_prefix("bytes=")?;
        let parts: Vec<&str> = range_spec.split('-').collect();

        if parts.is_empty() {
            return None;
        }

        // Parse start position
        let start: u64 = parts[0].parse().ok()?;

        // Parse end position (defaults to end of file)
        let end: u64 = if parts.len() > 1 && !parts[1].is_empty() {
            parts[1].parse().ok()?
        } else {
            file_size.saturating_sub(1)
        };

        Self::new(start, end, file_size)
    }

    /// Get the length of this range in bytes
    pub fn length(&self) -> u64 {
        self.end.saturating_sub(self.start).saturating_add(1)
    }

    /// Get the range as a RangeInclusive for use with file operations
    pub fn as_range_inclusive(&self) -> RangeInclusive<u64> {
        self.start..=self.end
    }

    /// Format as a Content-Range header value
    pub fn format_content_range(&self) -> String {
        format!(
            "bytes {}-{}/{}",
            self.start, self.end, self.total_size
        )
    }
}

/// Safely calculate buffer size for a range, capping at maximum
pub fn safe_buffer_size(range_length: u64) -> usize {
    range_length.min(MAX_RANGE_SIZE) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_range() {
        let range = ByteRange::new(0, 1023, 10000).unwrap();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 1023);
        assert_eq!(range.length(), 1024);
    }

    #[test]
    fn test_range_clamped_to_file_size() {
        let range = ByteRange::new(0, 99999, 5000).unwrap();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 4999); // Clamped to file_size - 1
    }

    #[test]
    fn test_invalid_range_end_before_start() {
        // end < start should be rejected
        assert!(ByteRange::new(1000, 0, 10000).is_none());
    }

    #[test]
    fn test_invalid_range_start_beyond_file() {
        // start >= file_size should be rejected
        assert!(ByteRange::new(10000, 10010, 10000).is_none());
    }

    #[test]
    fn test_invalid_range_exceeds_max_size() {
        // Range exceeding MAX_RANGE_SIZE should be rejected
        assert!(ByteRange::new(0, MAX_RANGE_SIZE + 1, MAX_RANGE_SIZE + 2).is_none());
    }

    #[test]
    fn test_parse_header_valid() {
        let range = ByteRange::parse_header("bytes=0-1023", 10000).unwrap();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 1023);
    }

    #[test]
    fn test_parse_header_open_ended() {
        let range = ByteRange::parse_header("bytes=1000-", 5000).unwrap();
        assert_eq!(range.start, 1000);
        assert_eq!(range.end, 4999); // End of file
    }

    #[test]
    fn test_parse_header_invalid_format() {
        assert!(ByteRange::parse_header("invalid", 10000).is_none());
        assert!(ByteRange::parse_header("bytes=", 10000).is_none());
        assert!(ByteRange::parse_header("bytes=abc-def", 10000).is_none());
    }

    #[test]
    fn test_content_range_format() {
        let range = ByteRange::new(100, 200, 1000).unwrap();
        assert_eq!(range.format_content_range(), "bytes 100-200/1000");
    }

    #[test]
    fn test_safe_buffer_size() {
        assert_eq!(safe_buffer_size(1024), 1024);
        assert_eq!(safe_buffer_size(MAX_RANGE_SIZE + 1000), MAX_RANGE_SIZE as usize);
    }
}
