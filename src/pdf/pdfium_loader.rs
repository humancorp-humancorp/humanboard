//! PDFium library loader with platform-specific search paths.
//!
//! This module centralizes the logic for locating and loading the PDFium
//! dynamic library across different deployment scenarios.

use pdfium_render::prelude::*;
use std::path::PathBuf;

pub struct PdfiumLoader;

impl PdfiumLoader {
    /// Load the PDFium library from known search paths or system library.
    ///
    /// Search order:
    /// 1. `lib/libpdfium.dylib` in current working directory (development)
    /// 2. `lib/libpdfium.dylib` relative to executable
    /// 3. `Resources/lib/libpdfium.dylib` in macOS bundle
    /// 4. System library fallback
    pub fn load() -> Result<Pdfium, String> {
        let paths = Self::search_paths();
        for path in paths {
            if path.exists() {
                if let Ok(bindings) = Pdfium::bind_to_library(&path) {
                    return Ok(Pdfium::new(bindings));
                }
            }
        }
        Pdfium::bind_to_system_library()
            .map(Pdfium::new)
            .map_err(|e| format!("Failed to load pdfium: {:?}", e))
    }

    fn search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // Current working directory (development)
        if let Ok(cwd) = std::env::current_dir() {
            paths.push(cwd.join("lib/libpdfium.dylib"));
        }
        
        // Executable-relative path
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                paths.push(parent.join("lib/libpdfium.dylib"));
                
                // macOS bundle path
                if let Some(grandparent) = parent.parent() {
                    paths.push(grandparent.join("Resources/lib/libpdfium.dylib"));
                }
            }
        }
        
        paths
    }
}
