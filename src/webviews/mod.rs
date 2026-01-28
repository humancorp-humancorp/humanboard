//! WebView-based media players and viewers.
//!
//! This module provides embedded web content viewers for various media types,
//! each implemented using GPUI's WebView component with local HTTP servers.
//!
//! ## Modules
//!
//! - `audio` - Audio player with metadata display (MP3, WAV, OGG, etc.)
//! - `video` - Video player with streaming (MP4, WebM, MOV, etc.)
//! - `youtube` - YouTube iframe embed player
//! - `pdf` - Native PDF viewer using platform rendering

mod audio;
mod pdf;
mod range_utils;
mod video;
mod youtube;

pub use audio::AudioWebView;
pub use pdf::PdfWebView;
pub use range_utils::*;
pub use video::VideoWebView;
pub use youtube::YouTubeWebView;
