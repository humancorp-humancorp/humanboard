//! Snapshot tests using the insta crate.
//!
//! Snapshot testing captures complex output and stores it in `.snap` files,
//! making it easy to verify and update expected values. This approach is
//! particularly useful for:
//!
//! - Serialization formats (JSON, YAML, etc.)
//! - Complex data structures with many fields
//! - Output that changes frequently during development
//!
//! To update snapshots after intentional changes:
//! ```sh
//! cargo insta test --accept
//! ```
//!
//! Or review changes interactively:
//! ```sh
//! cargo insta review
//! ```

use humanboard::board_index::{BoardMetadata, StoredLocation};
use humanboard::settings::{AppSettings, SettingsContent};
use humanboard::types::{
    AggregationType, ArrowHead, CanvasItem, ChartConfig, ChartType, DataCell, DataColumn,
    DataOrigin, DataRow, DataSource, DataType, ItemContent, ShapeType, SortOrder,
};
use std::path::PathBuf;

// ============================================================================
// CanvasItem Serialization Tests
// ============================================================================

#[test]
fn snapshot_canvas_item_image() {
    let item = CanvasItem {
        id: 1,
        position: (100.0, 200.0),
        size: (800.0, 600.0),
        content: ItemContent::Image(PathBuf::from("/path/to/image.png")),
    };
    insta::assert_json_snapshot!("canvas_item_image", item);
}

#[test]
fn snapshot_canvas_item_video() {
    let item = CanvasItem {
        id: 2,
        position: (50.0, 100.0),
        size: (400.0, 300.0),
        content: ItemContent::Video(PathBuf::from("/path/to/video.mp4")),
    };
    insta::assert_json_snapshot!("canvas_item_video", item);
}

#[test]
fn snapshot_canvas_item_audio() {
    let item = CanvasItem {
        id: 3,
        position: (0.0, 0.0),
        size: (320.0, 160.0),
        content: ItemContent::Audio(PathBuf::from("/music/song.mp3")),
    };
    insta::assert_json_snapshot!("canvas_item_audio", item);
}

#[test]
fn snapshot_canvas_item_pdf() {
    let item = CanvasItem {
        id: 4,
        position: (300.0, 400.0),
        size: (180.0, 240.0),
        content: ItemContent::Pdf {
            path: PathBuf::from("/documents/report.pdf"),
            thumbnail: Some(PathBuf::from("/cache/report_thumb.png")),
        },
    };
    insta::assert_json_snapshot!("canvas_item_pdf", item);
}

#[test]
fn snapshot_canvas_item_text() {
    let item = CanvasItem {
        id: 5,
        position: (10.0, 20.0),
        size: (300.0, 100.0),
        content: ItemContent::Text("Hello, Humanboard!".to_string()),
    };
    insta::assert_json_snapshot!("canvas_item_text", item);
}

#[test]
fn snapshot_canvas_item_link() {
    let item = CanvasItem {
        id: 6,
        position: (150.0, 250.0),
        size: (300.0, 150.0),
        content: ItemContent::Link("https://github.com/humanboard-org".to_string()),
    };
    insta::assert_json_snapshot!("canvas_item_link", item);
}

#[test]
fn snapshot_canvas_item_youtube() {
    let item = CanvasItem {
        id: 7,
        position: (200.0, 300.0),
        size: (560.0, 315.0),
        content: ItemContent::YouTube("dQw4w9WgXcQ".to_string()),
    };
    insta::assert_json_snapshot!("canvas_item_youtube", item);
}

#[test]
fn snapshot_canvas_item_markdown() {
    let item = CanvasItem {
        id: 8,
        position: (400.0, 100.0),
        size: (200.0, 36.0),
        content: ItemContent::Markdown {
            path: PathBuf::from("/docs/README.md"),
            title: "README".to_string(),
            content: "# Hello\n\nThis is a test document.".to_string(),
        },
    };
    insta::assert_json_snapshot!("canvas_item_markdown", item);
}

#[test]
fn snapshot_canvas_item_code() {
    let item = CanvasItem {
        id: 9,
        position: (500.0, 200.0),
        size: (200.0, 36.0),
        content: ItemContent::Code {
            path: PathBuf::from("/src/main.rs"),
            language: "rust".to_string(),
        },
    };
    insta::assert_json_snapshot!("canvas_item_code", item);
}

#[test]
fn snapshot_canvas_item_textbox() {
    let item = CanvasItem {
        id: 10,
        position: (100.0, 100.0),
        size: (200.0, 100.0),
        content: ItemContent::TextBox {
            text: "Editable text content".to_string(),
            font_size: 16.0,
            color: "#ffffff".to_string(),
        },
    };
    insta::assert_json_snapshot!("canvas_item_textbox", item);
}

#[test]
fn snapshot_canvas_item_arrow() {
    let item = CanvasItem {
        id: 11,
        position: (50.0, 50.0),
        size: (200.0, 100.0),
        content: ItemContent::Arrow {
            end_offset: (200.0, 100.0),
            color: "#ff5500".to_string(),
            thickness: 3.0,
            head_style: ArrowHead::Arrow,
        },
    };
    insta::assert_json_snapshot!("canvas_item_arrow", item);
}

#[test]
fn snapshot_canvas_item_shape_rectangle() {
    let item = CanvasItem {
        id: 12,
        position: (200.0, 200.0),
        size: (150.0, 100.0),
        content: ItemContent::Shape {
            shape_type: ShapeType::Rectangle,
            fill_color: Some("#3366cc".to_string()),
            border_color: "#ffffff".to_string(),
            border_width: 2.0,
        },
    };
    insta::assert_json_snapshot!("canvas_item_shape_rectangle", item);
}

#[test]
fn snapshot_canvas_item_shape_ellipse() {
    let item = CanvasItem {
        id: 13,
        position: (350.0, 200.0),
        size: (120.0, 120.0),
        content: ItemContent::Shape {
            shape_type: ShapeType::Ellipse,
            fill_color: None,
            border_color: "#00ff00".to_string(),
            border_width: 4.0,
        },
    };
    insta::assert_json_snapshot!("canvas_item_shape_ellipse", item);
}

#[test]
fn snapshot_canvas_item_table() {
    let item = CanvasItem {
        id: 14,
        position: (0.0, 500.0),
        size: (400.0, 300.0),
        content: ItemContent::Table {
            data_source_id: 100,
            show_headers: true,
            stripe: true,
        },
    };
    insta::assert_json_snapshot!("canvas_item_table", item);
}

#[test]
fn snapshot_canvas_item_chart() {
    let config = ChartConfig {
        chart_type: ChartType::Bar,
        x_column: Some(0),
        y_columns: vec![1, 2],
        title: Some("Sales by Region".to_string()),
        show_legend: true,
        aggregation: AggregationType::Sum,
        sort_order: SortOrder::ValueDesc,
    };
    let item = CanvasItem {
        id: 15,
        position: (500.0, 500.0),
        size: (400.0, 300.0),
        content: ItemContent::Chart {
            data_source_id: 100,
            source_item_id: Some(14),
            config,
        },
    };
    insta::assert_json_snapshot!("canvas_item_chart", item);
}

// ============================================================================
// ArrowHead Variants Tests
// ============================================================================

#[test]
fn snapshot_arrow_head_variants() {
    let variants = vec![
        ("none", ArrowHead::None),
        ("arrow", ArrowHead::Arrow),
        ("diamond", ArrowHead::Diamond),
        ("circle", ArrowHead::Circle),
    ];
    for (name, head) in variants {
        insta::assert_json_snapshot!(format!("arrow_head_{}", name), head);
    }
}

// ============================================================================
// ShapeType Variants Tests
// ============================================================================

#[test]
fn snapshot_shape_type_variants() {
    let variants = vec![
        ("rectangle", ShapeType::Rectangle),
        ("rounded_rect", ShapeType::RoundedRect),
        ("ellipse", ShapeType::Ellipse),
    ];
    for (name, shape) in variants {
        insta::assert_json_snapshot!(format!("shape_type_{}", name), shape);
    }
}

// ============================================================================
// ChartType Variants Tests
// ============================================================================

#[test]
fn snapshot_chart_type_variants() {
    let variants = vec![
        ("line", ChartType::Line),
        ("bar", ChartType::Bar),
        ("area", ChartType::Area),
        ("pie", ChartType::Pie),
        ("scatter", ChartType::Scatter),
    ];
    for (name, chart) in variants {
        insta::assert_json_snapshot!(format!("chart_type_{}", name), chart);
    }
}

// ============================================================================
// ChartConfig Tests
// ============================================================================

#[test]
fn snapshot_chart_config_default() {
    let config = ChartConfig::default();
    insta::assert_json_snapshot!("chart_config_default", config);
}

#[test]
fn snapshot_chart_config_complex() {
    let config = ChartConfig::new(ChartType::Line)
        .with_columns(0, vec![1, 2, 3])
        .with_title("Revenue Trends")
        .with_aggregation(AggregationType::Average)
        .with_sort_order(SortOrder::LabelAsc);
    insta::assert_json_snapshot!("chart_config_complex", config);
}

// ============================================================================
// DataSource Tests
// ============================================================================

#[test]
fn snapshot_data_source_empty() {
    let source = DataSource::new_empty(1, "My Data".to_string());
    insta::assert_json_snapshot!("data_source_empty", source);
}

#[test]
fn snapshot_data_source_with_data() {
    let source = DataSource {
        id: 42,
        name: "Sales Report".to_string(),
        columns: vec![
            DataColumn::new("Region", DataType::Text),
            DataColumn::new("Q1 Sales", DataType::Number),
            DataColumn::new("Q2 Sales", DataType::Number),
        ],
        rows: vec![
            DataRow::new(vec![
                DataCell::Text("North".to_string()),
                DataCell::Number(15000.0),
                DataCell::Number(18500.0),
            ]),
            DataRow::new(vec![
                DataCell::Text("South".to_string()),
                DataCell::Number(12000.0),
                DataCell::Number(14200.0),
            ]),
        ],
        origin: DataOrigin::File {
            path: PathBuf::from("/data/sales.csv"),
            delimiter: ',',
        },
        dirty: false,
    };
    insta::assert_json_snapshot!("data_source_with_data", source);
}

// ============================================================================
// DataCell Variants Tests
// ============================================================================

#[test]
fn snapshot_data_cell_variants() {
    let variants: Vec<(&str, DataCell)> = vec![
        ("text", DataCell::Text("Hello".to_string())),
        ("number", DataCell::Number(42.5)),
        ("boolean_true", DataCell::Boolean(true)),
        ("boolean_false", DataCell::Boolean(false)),
        ("date", DataCell::Date("2024-01-15".to_string())),
        ("empty", DataCell::Empty),
    ];
    for (name, cell) in variants {
        insta::assert_json_snapshot!(format!("data_cell_{}", name), cell);
    }
}

// ============================================================================
// DataOrigin Variants Tests
// ============================================================================

#[test]
fn snapshot_data_origin_variants() {
    let variants: Vec<(&str, DataOrigin)> = vec![
        ("manual", DataOrigin::Manual),
        (
            "file_csv",
            DataOrigin::File {
                path: PathBuf::from("/data/file.csv"),
                delimiter: ',',
            },
        ),
        (
            "file_tsv",
            DataOrigin::File {
                path: PathBuf::from("/data/file.tsv"),
                delimiter: '\t',
            },
        ),
        (
            "json_with_path",
            DataOrigin::Json {
                path: Some(PathBuf::from("/data/file.json")),
            },
        ),
        ("json_no_path", DataOrigin::Json { path: None }),
        (
            "api",
            DataOrigin::Api {
                url: "https://api.example.com/data".to_string(),
                last_fetched: Some(1704067200),
            },
        ),
    ];
    for (name, origin) in variants {
        insta::assert_json_snapshot!(format!("data_origin_{}", name), origin);
    }
}

// ============================================================================
// Settings Serialization Tests
// ============================================================================

#[test]
fn snapshot_app_settings_default() {
    let settings = AppSettings::default();
    insta::assert_json_snapshot!("app_settings_default", settings);
}

#[test]
fn snapshot_settings_content_partial() {
    let content = SettingsContent {
        theme: Some("Catppuccin Mocha".to_string()),
        font: Some("JetBrainsMono Nerd Font".to_string()),
        font_size: Some(14.0),
        canvas_background: None,
        grid_size: Some(25.0),
        show_grid: Some(true),
        snap_to_grid: None,
        auto_save_interval: None,
        max_undo_history: None,
        zoom_sensitivity: Some(1.5),
        pan_sensitivity: None,
        onboarding_completed: Some(true),
        reduce_motion: Some("off".to_string()),
        high_contrast: None,
    };
    insta::assert_json_snapshot!("settings_content_partial", content);
}

#[test]
fn snapshot_settings_content_full() {
    let content = SettingsContent {
        theme: Some("Default Dark".to_string()),
        font: Some("Iosevka Nerd Font".to_string()),
        font_size: Some(14.0),
        canvas_background: Some("#1a1a1a".to_string()),
        grid_size: Some(20.0),
        show_grid: Some(false),
        snap_to_grid: Some(false),
        auto_save_interval: Some(30),
        max_undo_history: Some(100),
        zoom_sensitivity: Some(1.0),
        pan_sensitivity: Some(1.0),
        onboarding_completed: Some(false),
        reduce_motion: Some("system".to_string()),
        high_contrast: Some(false),
    };
    insta::assert_json_snapshot!("settings_content_full", content);
}

// ============================================================================
// BoardMetadata Tests
// ============================================================================

#[test]
fn snapshot_board_metadata() {
    // Use fixed values for deterministic snapshots
    let metadata = BoardMetadata {
        id: "abc123def456abc123def456abc12345".to_string(),
        name: "My Moodboard".to_string(),
        created_at: 1704067200, // 2024-01-01 00:00:00 UTC
        updated_at: 1704153600, // 2024-01-02 00:00:00 UTC
        storage_location: StoredLocation::Default,
        deleted_at: None,
    };
    insta::assert_json_snapshot!("board_metadata", metadata);
}

#[test]
fn snapshot_board_metadata_deleted() {
    let metadata = BoardMetadata {
        id: "xyz789xyz789xyz789xyz789xyz78901".to_string(),
        name: "Archived Board".to_string(),
        created_at: 1704067200,
        updated_at: 1704153600,
        storage_location: StoredLocation::ICloud,
        deleted_at: Some(1704240000), // 2024-01-03 00:00:00 UTC
    };
    insta::assert_json_snapshot!("board_metadata_deleted", metadata);
}

// ============================================================================
// StoredLocation Variants Tests
// ============================================================================

#[test]
fn snapshot_stored_location_variants() {
    let variants: Vec<(&str, StoredLocation)> = vec![
        ("default", StoredLocation::Default),
        ("icloud", StoredLocation::ICloud),
        (
            "custom",
            StoredLocation::Custom(PathBuf::from("/Users/user/Documents/Boards")),
        ),
    ];
    for (name, location) in variants {
        insta::assert_json_snapshot!(format!("stored_location_{}", name), location);
    }
}

// ============================================================================
// Complex Multi-Item Snapshot Tests
// ============================================================================

#[test]
fn snapshot_canvas_items_collection() {
    let items = vec![
        CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (200.0, 100.0),
            content: ItemContent::TextBox {
                text: "Title".to_string(),
                font_size: 24.0,
                color: "#ffffff".to_string(),
            },
        },
        CanvasItem {
            id: 2,
            position: (0.0, 120.0),
            size: (400.0, 300.0),
            content: ItemContent::Image(PathBuf::from("/images/diagram.png")),
        },
        CanvasItem {
            id: 3,
            position: (420.0, 120.0),
            size: (300.0, 150.0),
            content: ItemContent::Link("https://example.com".to_string()),
        },
    ];
    insta::assert_json_snapshot!("canvas_items_collection", items);
}

// ============================================================================
// String Output Snapshot Tests
// ============================================================================

#[test]
fn snapshot_type_labels() {
    // Create items with sufficient lifetime
    let image = ItemContent::Image(PathBuf::new());
    let video = ItemContent::Video(PathBuf::new());
    let audio = ItemContent::Audio(PathBuf::new());
    let pdf = ItemContent::Pdf {
        path: PathBuf::new(),
        thumbnail: None,
    };
    let text = ItemContent::Text(String::new());
    let link = ItemContent::Link(String::new());
    let youtube = ItemContent::YouTube(String::new());
    let markdown = ItemContent::Markdown {
        path: PathBuf::new(),
        title: String::new(),
        content: String::new(),
    };
    let code_rust = ItemContent::Code {
        path: PathBuf::new(),
        language: "rust".to_string(),
    };
    let code_python = ItemContent::Code {
        path: PathBuf::new(),
        language: "python".to_string(),
    };

    let labels: Vec<(&str, &str)> = vec![
        ("image", image.type_label()),
        ("video", video.type_label()),
        ("audio", audio.type_label()),
        ("pdf", pdf.type_label()),
        ("text", text.type_label()),
        ("link", link.type_label()),
        ("youtube", youtube.type_label()),
        ("markdown", markdown.type_label()),
        ("code_rust", code_rust.type_label()),
        ("code_python", code_python.type_label()),
    ];

    // Build a string representation
    let output: String = labels
        .iter()
        .map(|(name, label)| format!("{}: {}", name, label))
        .collect::<Vec<_>>()
        .join("\n");

    insta::assert_snapshot!("type_labels", output);
}

#[test]
fn snapshot_display_names() {
    let items: Vec<(&str, String)> = vec![
        (
            "image",
            ItemContent::Image(PathBuf::from("/photos/vacation.jpg")).display_name(),
        ),
        (
            "video",
            ItemContent::Video(PathBuf::from("/videos/demo.mp4")).display_name(),
        ),
        (
            "text",
            ItemContent::Text("Hello World".to_string()).display_name(),
        ),
        (
            "link",
            ItemContent::Link("https://example.com/page".to_string()).display_name(),
        ),
        (
            "youtube",
            ItemContent::YouTube("abc123".to_string()).display_name(),
        ),
    ];

    let output: String = items
        .iter()
        .map(|(name, display)| format!("{}: {}", name, display))
        .collect::<Vec<_>>()
        .join("\n");

    insta::assert_snapshot!("display_names", output);
}

#[test]
fn snapshot_default_sizes() {
    let sizes: Vec<(&str, (f32, f32))> = vec![
        ("text", ItemContent::Text(String::new()).default_size()),
        ("video", ItemContent::Video(PathBuf::new()).default_size()),
        ("audio", ItemContent::Audio(PathBuf::new()).default_size()),
        (
            "pdf",
            ItemContent::Pdf {
                path: PathBuf::new(),
                thumbnail: None,
            }
            .default_size(),
        ),
        ("link", ItemContent::Link(String::new()).default_size()),
        (
            "youtube",
            ItemContent::YouTube(String::new()).default_size(),
        ),
        (
            "textbox",
            ItemContent::TextBox {
                text: String::new(),
                font_size: 16.0,
                color: String::new(),
            }
            .default_size(),
        ),
        (
            "shape",
            ItemContent::Shape {
                shape_type: ShapeType::Rectangle,
                fill_color: None,
                border_color: String::new(),
                border_width: 1.0,
            }
            .default_size(),
        ),
        (
            "table",
            ItemContent::Table {
                data_source_id: 0,
                show_headers: true,
                stripe: true,
            }
            .default_size(),
        ),
        (
            "chart",
            ItemContent::Chart {
                data_source_id: 0,
                source_item_id: None,
                config: ChartConfig::default(),
            }
            .default_size(),
        ),
    ];

    let output: String = sizes
        .iter()
        .map(|(name, (w, h))| format!("{}: {}x{}", name, w, h))
        .collect::<Vec<_>>()
        .join("\n");

    insta::assert_snapshot!("default_sizes", output);
}
