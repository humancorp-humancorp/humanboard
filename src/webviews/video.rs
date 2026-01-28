//! WebView-based video player with HTTP streaming.
//!
//! This module provides a video player implemented as a WebView with a local
//! HTTP server that supports range requests for seeking and streaming.
//!
//! ## Architecture
//!
//! Each video player spawns a local HTTP server on a unique port that serves:
//! - HTML page with native video element
//! - Video data with HTTP range request support for seeking
//!
//! ## Supported Formats
//!
//! MP4, WebM, MOV, AVI, MKV

use gpui::*;
use gpui_component::webview::WebView;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tiny_http::{Header, Response, Server, StatusCode};
use tracing::error;
use wry::WebViewBuilder;

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19950);

/// Helper to create HTTP headers, returning None if the bytes are invalid
fn create_header(name: &[u8], value: &[u8]) -> Option<Header> {
    Header::from_bytes(name, value).ok()
}

/// WebView-based video player with local HTTP server supporting range requests
pub struct VideoWebView {
    pub webview_entity: Entity<WebView>,
    pub video_path: PathBuf,
    shutdown_flag: Arc<AtomicBool>,
    server_thread: Option<JoinHandle<()>>,
}

impl VideoWebView {
    pub fn new(video_path: PathBuf, window: &mut Window, cx: &mut App) -> Result<Self, String> {
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let video_path_clone = video_path.clone();
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        // Channel for server startup synchronization
        let (tx, rx) = mpsc::channel();

        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => {
                    let _ = tx.send(Ok(()));
                    s
                }
                Err(e) => {
                    error!("Failed to start video server on port {}: {}", port, e);
                    let _ = tx.send(Err(e.to_string()));
                    return;
                }
            };

            let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        html, body { width: 100%; height: 100%; background: #000; overflow: hidden; }
        video { width: 100%; height: 100%; object-fit: contain; }
    </style>
</head>
<body>
    <video controls>
        <source src="/video" type="video/mp4">
    </video>
</body>
</html>"#
                .to_string();

            loop {
                if shutdown_flag_clone.load(Ordering::Relaxed) {
                    break;
                }

                match server.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(Some(request)) => {
                        let url = request.url();
                        if url.starts_with("/video") {
                            // Handle range requests for video streaming
                            Self::serve_video_file(&video_path_clone, request);
                        } else {
                            let mut response = Response::from_string(&html);
                            if let Some(header) =
                                create_header(&b"Content-Type"[..], &b"text/html"[..])
                            {
                                response = response.with_header(header);
                            }
                            let _ = request.respond(response);
                        }
                    }
                    Ok(None) => {}
                    Err(_) => break,
                }
            }
        });

        // Wait for server to start with timeout
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return Err(format!("Failed to start server: {}", e)),
            Err(_) => return Err("Server startup timeout".to_string()),
        }

        let url = format!("http://127.0.0.1:{}/", port);

        #[cfg(any(
            target_os = "macos",
            target_os = "windows",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview = WebViewBuilder::new()
            .with_url(&url)
            .build_as_child(window)
            .map_err(|e| format!("Failed to create WebView: {:?}", e))?;

        #[cfg(not(any(
            target_os = "macos",
            target_os = "windows",
            target_os = "ios",
            target_os = "android"
        )))]
        return Err("WebView not supported on this platform".to_string());

        let webview_entity = cx.new(|cx| WebView::new(webview, window, cx));

        Ok(Self {
            webview_entity,
            video_path,
            shutdown_flag,
            server_thread: Some(server_thread),
        })
    }

    /// Hide the webview (should be called before dropping to prevent orphaned UI)
    pub fn hide(&self, cx: &mut App) {
        self.webview_entity.update(cx, |wv, _| wv.hide());
    }

    fn serve_video_file(path: &PathBuf, request: tiny_http::Request) {
        let mut file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => {
                let _ = request.respond(Response::empty(StatusCode(404)));
                return;
            }
        };

        let file_size = match file.metadata() {
            Ok(m) => m.len(),
            Err(_) => {
                let _ = request.respond(Response::empty(StatusCode(500)));
                return;
            }
        };

        let mime = match path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .as_deref()
        {
            Some("mp4") => "video/mp4",
            Some("webm") => "video/webm",
            Some("mov") => "video/quicktime",
            Some("avi") => "video/x-msvideo",
            Some("mkv") => "video/x-matroska",
            _ => "video/mp4",
        };

        // Check for Range header
        let range_header = request
            .headers()
            .iter()
            .find(|h| h.field.as_str() == "Range" || h.field.as_str() == "range")
            .map(|h| h.value.as_str().to_string());

        if let Some(range) = range_header {
            use crate::webviews::ByteRange;
            
            if let Some(byte_range) = ByteRange::parse_header(&range, file_size) {
                // Seek to start position
                if file.seek(SeekFrom::Start(byte_range.start)).is_err() {
                    let _ = request.respond(Response::empty(StatusCode(500)));
                    return;
                }

                // Read the requested range with safe buffer size
                let length = byte_range.length();
                let mut buffer = vec![0u8; length as usize];
                if file.read_exact(&mut buffer).is_err() {
                    // Try reading what we can
                    let _ = file.seek(SeekFrom::Start(byte_range.start));
                    buffer.clear();
                    let _ = file.take(length).read_to_end(&mut buffer);
                }

                let mut response = Response::from_data(buffer).with_status_code(StatusCode(206));
                if let Some(h) = create_header(&b"Content-Type"[..], mime.as_bytes()) {
                    response = response.with_header(h);
                }
                if let Some(h) = create_header(
                    &b"Content-Range"[..],
                    byte_range.format_content_range().as_bytes(),
                ) {
                    response = response.with_header(h);
                }
                if let Some(h) = create_header(&b"Accept-Ranges"[..], &b"bytes"[..]) {
                    response = response.with_header(h);
                }
                if let Some(h) =
                    create_header(&b"Content-Length"[..], length.to_string().as_bytes())
                {
                    response = response.with_header(h);
                }

                let _ = request.respond(response);
                return;
            }
            // If range parsing fails, fall through to serve entire file (HTTP 200)
        }

        // No range request - serve entire file
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            let _ = request.respond(Response::empty(StatusCode(500)));
            return;
        }

        let mut response = Response::from_data(buffer);
        if let Some(h) = create_header(&b"Content-Type"[..], mime.as_bytes()) {
            response = response.with_header(h);
        }
        if let Some(h) = create_header(&b"Accept-Ranges"[..], &b"bytes"[..]) {
            response = response.with_header(h);
        }
        if let Some(h) = create_header(&b"Content-Length"[..], file_size.to_string().as_bytes()) {
            response = response.with_header(h);
        }

        let _ = request.respond(response);
    }
}

impl Drop for VideoWebView {
    fn drop(&mut self) {
        self.shutdown_flag.store(true, Ordering::SeqCst);
        if let Some(handle) = self.server_thread.take() {
            let _ = handle.join();
        }
    }
}
