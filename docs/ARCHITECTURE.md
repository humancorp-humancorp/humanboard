# Humanboard Architecture

This document provides a high-level overview of how Humanboard is structured.

## Overview

Humanboard is a Miro-style infinite canvas application built with [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui), the GPU-accelerated UI framework from Zed editor.

```
┌─────────────────────────────────────────────────────────────┐
│                        Humanboard                           │
├─────────────┬─────────────────────────────┬─────────────────┤
│  Tool Dock  │         Canvas              │  Preview Panel  │
│  (left)     │    (infinite scroll)        │  (right, opt)   │
├─────────────┴─────────────────────────────┴─────────────────┤
│                      Footer Bar                             │
└─────────────────────────────────────────────────────────────┘
```

## Core Data Flow

```
User Input → Input Handlers → App State → Render
     │                            │
     │                            ▼
     │                     Board (persistence)
     │                            │
     └──── Actions ◄──────────────┘
```

1. **User Input**: Mouse/keyboard events captured by GPUI
2. **Input Handlers**: `src/input/` processes events, updates state
3. **App State**: `Humanboard` struct holds all application state
4. **Board**: Persists canvas items to disk, manages undo/redo
5. **Render**: GPUI re-renders UI when state changes

## Key Components

### Humanboard (`src/app/`)

The main application struct. Methods are split across submodules:

```
app/
├── state.rs           # Humanboard struct definition
├── lifecycle.rs       # Init, cleanup, window management
├── board_management.rs    # Create, open, save boards
├── settings_methods.rs    # Theme, font preferences
├── preview_*.rs       # Preview panel logic (5 files)
├── textbox.rs         # Text editing
└── ...
```

### Board (`src/board.rs`)

Canvas state and persistence:

- **Items**: `HashMap<u64, CanvasItem>` for O(1) lookup
- **Spatial Index**: R-tree for efficient hit testing and culling
- **Undo/Redo**: Delta-based history with periodic snapshots
- **Auto-save**: Debounced saves (500ms) to avoid disk thrashing

### Types (`src/types.rs`)

Core data structures:

```rust
CanvasItem {
    id: u64,
    position: Point,
    size: Size,
    content: ItemContent,  // Image, Text, PDF, Video, etc.
}

ItemContent {
    Image { path, ... }
    Text(String)
    Markdown { path, content, ... }
    Pdf { path, ... }
    Video { path, ... }
    // ... more variants
}
```

### Rendering (`src/render/`)

Modular rendering split by area:

```
render/
├── mod.rs      # Main Render impl, action bindings
├── canvas.rs   # Canvas items, selection, drawing tools
├── dock.rs     # Left tool dock
├── preview.rs  # Right preview panel
└── overlays/   # Command palette, settings, modals
```

### Input (`src/input/`)

Mouse and keyboard handling:

```
input/
├── mouse_down.rs  # Start selection, drag, resize
├── mouse_up.rs    # Finalize drawing, push history
├── drag.rs        # Move items, pan canvas
└── transform.rs   # Zoom, coordinate conversion
```

### Focus System (`src/focus.rs`)

Priority-based focus contexts (inspired by Zed):

```
FocusContext::CommandPalette  (highest)
FocusContext::Modal
FocusContext::TextEditing
FocusContext::Preview
FocusContext::Canvas          (default)
```

Higher priority contexts capture keyboard input first.

### Actions (`src/actions.rs`)

Type-safe keyboard commands:

```rust
actions!(humanboard, [
    Undo, Redo,
    DeleteSelected,
    ZoomIn, ZoomOut,
    // ...
]);
```

Bound to keys in `render/mod.rs` using GPUI's `on_action()`.

## File Formats

### Board Files (`.board`)

JSON files containing:
- Canvas offset and zoom level
- All canvas items with positions/sizes
- Data sources for tables/charts

### Settings (`~/.humanboard/settings.json`)

User preferences with hot-reloading support.

## Performance Considerations

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Item lookup | O(1) | HashMap by ID |
| Hit testing | O(log n) | R-tree spatial index |
| Undo/redo | O(k) | k = affected items |
| Render | O(visible) | Viewport culling |

Enable profiling: `cargo build --features profiling`

## Module Dependency Graph

```
lib.rs
├── app/          (uses: board, types, render, input, focus, actions)
├── board.rs      (uses: types, spatial_index)
├── render/       (uses: app, types, focus, actions)
├── input/        (uses: app, types)
├── types.rs      (standalone)
├── actions.rs    (standalone)
└── focus.rs      (standalone)
```

Lower modules should not depend on higher ones.
