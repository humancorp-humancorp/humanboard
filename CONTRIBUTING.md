# Contributing to Humanboard

Thanks for your interest in contributing! This guide will help you navigate the codebase.

## Project Structure

```
src/
├── app/                 # Application state and methods
├── data/                # Data parsing (CSV, JSON, lazy loading)
├── input/               # Mouse and keyboard input handling
├── render/              # UI rendering (canvas, dock, overlays)
│   └── overlays/        # Modal dialogs and popups
├── webviews/            # Embedded web content (YouTube, audio, video, PDF)
├── pdf/                 # PDF handling (viewer, thumbnails)
└── [core modules]       # Board logic, types, settings, etc.
```

### Core Modules

| Module | Purpose |
|--------|---------|
| `app/` | Main application state (`Humanboard` struct) and all its methods |
| `board.rs` | Canvas board state, items, undo/redo history |
| `types.rs` | Shared type definitions (`CanvasItem`, `ItemContent`, etc.) |
| `actions.rs` | Keyboard shortcut actions (Zed-style action system) |
| `settings.rs` | User preferences and configuration |
| `focus.rs` | Keyboard focus management |

### Rendering

| Module | Purpose |
|--------|---------|
| `render/canvas.rs` | Canvas and item drawing |
| `render/dock.rs` | Left-side tool dock |
| `render/preview.rs` | Right-side preview panel |
| `render/overlays/` | Command palette, settings modal, shortcuts |

### Input Handling

| Module | Purpose |
|--------|---------|
| `input/mouse_down.rs` | Selection, drag/resize start |
| `input/mouse_up.rs` | Finalize operations, create drawn items |
| `input/drag.rs` | Drag, resize, pan operations |
| `input/transform.rs` | Zoom, scroll, coordinate conversion |

### Data

| Module | Purpose |
|--------|---------|
| `data/csv_parser.rs` | CSV file parsing |
| `data/json_parser.rs` | JSON file parsing |
| `data/lazy_source.rs` | Polars-backed lazy loading for large datasets |

## Naming Conventions

- **`*_handlers.rs`** - Event handler implementations
- **`preview_*.rs`** - Preview panel related code (tabs, panes, search, webviews)
- **Feature modules** - Group related files with consistent prefixes

## Where to Add New Features

| Feature Type | Location |
|--------------|----------|
| New tool | `types.rs` (add `ToolType`), `render/dock.rs`, `input/` |
| New file format | `data/` module |
| New canvas item | `types.rs` (`ItemContent`), `render/canvas.rs` |
| New keyboard shortcut | `actions.rs`, bind in `render/mod.rs` |
| New settings | `settings.rs`, `app/settings_handlers.rs` |
| New webview embed | `webviews/` module |

## Code Style

- Module-level doc comments (`//!`) at the top of each file
- Use `///` for public function/struct documentation
- Keep files focused - split when a file exceeds ~500-700 lines
- Follow existing patterns in similar modules

## Running Tests

```bash
cargo test                    # Run all tests
cargo test --lib              # Library tests only
cargo test unit::             # Unit tests only
cargo test integration::      # Integration tests only
```

## Building

```bash
cargo build                   # Debug build
cargo build --release         # Release build
cargo build --features profiling  # With performance profiling
./build-app.sh                # Build macOS .app bundle
```

## Architecture Notes

Humanboard uses [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui), the UI framework from Zed editor:

- **Reactive rendering** - UI re-renders when state changes
- **Focus contexts** - Priority-based keyboard focus handling
- **Actions** - Type-safe commands bound to keyboard shortcuts

The main application struct is `Humanboard` in `src/app/state.rs`. Its methods are split across `src/app/` submodules to keep files manageable.
