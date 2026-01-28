//! PDF rendering and thumbnail generation using pdfium.
//!
//! This module provides PDF handling for the canvas (not the preview panel):
//!
//! - `document` - PdfDocument for rendering pages on canvas items
//! - `thumbnail` - First-page thumbnail generation for canvas cards
//! - `pdfium_loader` - Shared PDFium library loading logic
//!
//! For the preview panel PDF viewer, see `webviews::PdfWebView`.

mod document;
mod pdfium_loader;
mod thumbnail;

pub use document::PdfDocument;
pub use pdfium_loader::PdfiumLoader;
pub use thumbnail::generate_pdf_thumbnail;
