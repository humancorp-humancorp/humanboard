# Humanboard Roadmap

## Overview

This document outlines planned features and improvements for Humanboard. Features are organized by priority and development status.

---

## Status Legend

- 🔴 **Working** - Currently in active development
- 🟡 **Next Up** - Planned for next release cycle
- 🟢 **Backlog** - Under consideration for future releases
- ✅ **Shipped** - Completed and available

---

## 🔴 Working

| Feature | Description |
|---------|-------------|
| Data table cell editing | Direct inline editing of table cells on canvas |
| Chart data caching | Cache processed chart data to avoid reprocessing on every render |
| Theme-aware chart colors | Charts adapt to light/dark theme automatically |
| HTTP range request validation | Safe range requests for audio/video streaming |
| Settings UI component library | Reusable components for settings panels |

---

## 🟡 Next Up

| Feature | Description |
|---------|-------------|
| Zotero integration | Import and manage academic references |
| Web cards | Embed live web content as canvas items |
| Multiple spaces | Support for multiple workspace areas |
| Mobile PDF annotations | Full PDF highlighting and notes on mobile |
| AI tutor assistant | AI-powered help for organizing and analyzing content |
| Command palette fuzzy search | Better fuzzy matching for commands |
| Markdown syntax highlighting | Code blocks with proper syntax highlighting |
| Async file operations | Non-blocking file I/O for better performance |

---

## 🟢 Backlog

| Feature | Description |
|---------|-------------|
| Real-time collaboration | Multiple users editing same board simultaneously |
| Version history | Track and restore previous board states |
| Web clipper | Browser extension to capture web content |
| Mobile app (full) | Complete mobile experience with whiteboard editing |
| Plugin system | Third-party extensions and integrations |
| Advanced search | Full-text search across all content |
| Templates | Pre-made layouts and structures |
| Automation rules | Trigger actions based on events |
| Export options | PDF, Markdown, JSON export formats |
| Keyboard shortcuts | Comprehensive shortcut system |
| Focus mode | Distraction-free editing environment |
| Presentation mode | Present boards as slide decks |

---

## ✅ Recently Shipped

### v0.2.0 (2026-01-27)
- **Architecture**: Refactored monolithic state into focused sub-structs
- **Fix**: WebView thread leaks - proper cleanup on drop
- **Fix**: Input state machine - explicit state management
- **Fix**: CSV safety limits - prevent OOM on large files
- **Fix**: Secure UUID generation - cryptographically safe IDs
- **Fix**: Command states - proper undo/redo availability
- **Data**: Chart processing engine extracted from render path
- **UI**: Settings sidebar component extraction
- **Media**: Range request validation for streaming

### v0.1.0 (Initial Release)
- Infinite canvas with zoom and pan
- Support for images, videos, audio, PDFs
- Data tables with CSV/JSON import
- Charts with aggregation and sorting
- Web views (YouTube, generic web)
- Markdown cards
- Basic theming
- Command palette
- Undo/redo system
- Auto-save

---

## Completed Internal Improvements

### Performance
- Chart data processing moved out of render hot path
- Spatial index for O(log n) hit testing
- Viewport culling for canvas items
- Debounced auto-save (500ms)

### Code Quality
- God object refactored into 10 focused structs
- Unified error types with `thiserror`
- Coordinate conversion deduplicated
- Modal backdrop pattern extracted
- PDFium loader deduplicated

### Safety
- WebView server thread proper join on drop
- CSV file size limits (100MB) and row limits (100K)
- HTTP range request validation (100MB max)
- Bounds checking on table column access

---

## How to Contribute

1. Check **Backlog** for "good first issue" labels
2. Review **Working** to avoid duplicate effort
3. Submit feature requests via GitHub Issues
4. Join roadmap discussions

---

*Last updated: 2026-01-27*
