//! Chart data processing engine
//!
//! This module provides efficient chart data processing with caching support.
//! Heavy operations (grouping, aggregation, sorting) are performed here
//! rather than in the render path.

use crate::types::{AggregationType, ChartConfig, DataSource, SortOrder};
use gpui::Hsla;

/// Processed chart data ready for rendering
#[derive(Clone, Debug)]
pub struct ChartData {
    /// Data points with labels, values, and colors
    pub points: Vec<ChartPoint>,
    /// X-axis column name
    pub x_label: String,
    /// Y-axis column name  
    pub y_label: String,
    /// Maximum value for scaling
    pub max_value: f64,
    /// Minimum value for scaling
    pub min_value: f64,
}

/// A single data point in a chart
#[derive(Clone, Debug)]
pub struct ChartPoint {
    /// Label for this point (X-axis)
    pub label: String,
    /// Numeric value (Y-axis)
    pub value: f64,
    /// Color for this point/series
    pub color: Hsla,
}

/// Chart color palette - highly distinct colors for data visualization
pub const CHART_COLORS: [Hsla; 8] = [
    Hsla { h: 220.0 / 360.0, s: 0.85, l: 0.55, a: 1.0 },  // Bright Blue
    Hsla { h: 140.0 / 360.0, s: 0.75, l: 0.45, a: 1.0 },  // Green
    Hsla { h: 30.0 / 360.0,  s: 0.95, l: 0.55, a: 1.0 },  // Orange
    Hsla { h: 270.0 / 360.0, s: 0.75, l: 0.55, a: 1.0 },  // Violet/Purple
    Hsla { h: 0.0 / 360.0,   s: 0.80, l: 0.55, a: 1.0 },  // Red
    Hsla { h: 175.0 / 360.0, s: 0.75, l: 0.45, a: 1.0 },  // Cyan/Teal
    Hsla { h: 55.0 / 360.0,  s: 0.90, l: 0.50, a: 1.0 },  // Yellow
    Hsla { h: 320.0 / 360.0, s: 0.75, l: 0.55, a: 1.0 },  // Pink/Magenta
];

/// Maximum number of data points to display for readability
const MAX_CHART_POINTS: usize = 12;

/// Process raw data source into chart-ready format
///
/// This performs:
/// 1. Grouping by X column
/// 2. Aggregation of Y values
/// 3. Sorting according to config
/// 4. Color assignment
/// 5. Min/max calculation for scaling
pub fn process_chart_data(
    data_source: &DataSource,
    config: &ChartConfig,
) -> Option<ChartData> {
    let col_count = data_source.column_count();
    
    // Get column indices
    let x_col = config.x_column.unwrap_or(0);
    let y_col = if config.y_columns.is_empty() {
        if col_count > 1 { 1 } else { 0 }
    } else {
        config.y_columns[0]
    };
    
    // Get column names for labels
    let x_label = data_source.columns.get(x_col)
        .map(|c| c.name.clone())
        .unwrap_or_else(|| "X".to_string());
    let y_label = data_source.columns.get(y_col)
        .map(|c| c.name.clone())
        .unwrap_or_else(|| "Value".to_string());
    
    // Group raw values by label (X column), preserving insertion order
    let mut group_order: Vec<String> = Vec::new();
    let mut groups: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
    
    for row in &data_source.rows {
        let label = row.cells.get(x_col).map(|c| c.to_string()).unwrap_or_default();
        let value = row.cells.get(y_col).map(|c| c.to_f64()).unwrap_or(0.0);
        if !groups.contains_key(&label) {
            group_order.push(label.clone());
        }
        groups.entry(label).or_default().push(value);
    }
    
    if groups.is_empty() {
        return None;
    }
    
    // Apply aggregation in insertion order
    let mut points: Vec<(String, f64)> = group_order.into_iter().map(|label| {
        let values = groups.get(&label).unwrap();
        let aggregated = match config.aggregation {
            AggregationType::None => values.first().copied().unwrap_or(0.0),
            AggregationType::Sum => values.iter().sum(),
            AggregationType::Average => {
                if values.is_empty() { 0.0 }
                else { values.iter().sum::<f64>() / values.len() as f64 }
            }
            AggregationType::Count => values.len() as f64,
            AggregationType::Min => values.iter().cloned().fold(f64::INFINITY, f64::min),
            AggregationType::Max => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        };
        (label, aggregated)
    }).collect();
    
    // Apply sorting
    match config.sort_order {
        SortOrder::None => {} // Keep original insertion order
        SortOrder::LabelAsc => points.sort_by(|a, b| a.0.cmp(&b.0)),
        SortOrder::LabelDesc => points.sort_by(|a, b| b.0.cmp(&a.0)),
        SortOrder::ValueAsc => points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)),
        SortOrder::ValueDesc => points.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)),
    }
    
    // Convert to ChartPoints with colors, limit for readability
    let mut max_value = f64::NEG_INFINITY;
    let mut min_value = f64::INFINITY;
    
    let chart_points: Vec<ChartPoint> = points.into_iter()
        .take(MAX_CHART_POINTS)
        .enumerate()
        .map(|(i, (label, value))| {
            max_value = max_value.max(value);
            min_value = min_value.min(value);
            ChartPoint {
                label,
                value,
                color: CHART_COLORS[i % CHART_COLORS.len()],
            }
        })
        .collect();
    
    if chart_points.is_empty() {
        return None;
    }
    
    Some(ChartData {
        points: chart_points,
        x_label,
        y_label,
        max_value: if max_value == f64::NEG_INFINITY { 0.0 } else { max_value },
        min_value: if min_value == f64::INFINITY { 0.0 } else { min_value },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DataCell, DataColumn, DataOrigin, DataRow, DataType};

    fn create_test_data_source() -> DataSource {
        DataSource {
            id: 1,
            name: "Test".to_string(),
            columns: vec![
                DataColumn::new("Category", DataType::Text),
                DataColumn::new("Value", DataType::Number),
            ],
            rows: vec![
                DataRow::new(vec![DataCell::Text("A".to_string()), DataCell::Number(10.0)]),
                DataRow::new(vec![DataCell::Text("B".to_string()), DataCell::Number(20.0)]),
                DataRow::new(vec![DataCell::Text("A".to_string()), DataCell::Number(15.0)]), // Duplicate
            ],
            origin: DataOrigin::Manual,
            dirty: false,
        }
    }

    #[test]
    fn test_process_chart_data_groups_and_aggregates() {
        let ds = create_test_data_source();
        let config = ChartConfig::default(); // Sum aggregation
        
        let chart_data = process_chart_data(&ds, &config).unwrap();
        
        // Should have 2 groups: A (25) and B (20)
        assert_eq!(chart_data.points.len(), 2);
        assert_eq!(chart_data.points[0].label, "A");
        assert_eq!(chart_data.points[0].value, 25.0); // 10 + 15
        assert_eq!(chart_data.points[1].label, "B");
        assert_eq!(chart_data.points[1].value, 20.0);
    }

    #[test]
    fn test_process_chart_data_sorting() {
        let ds = create_test_data_source();
        let config = ChartConfig::default().with_sort_order(SortOrder::ValueDesc);
        
        let chart_data = process_chart_data(&ds, &config).unwrap();
        
        // Should be sorted by value descending: A (25), B (20)
        assert_eq!(chart_data.points[0].value, 25.0);
        assert_eq!(chart_data.points[1].value, 20.0);
    }
}
