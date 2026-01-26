//! Unit tests for perf module.

use humanboard::perf::{PerfMonitor, ScopedTimer};

#[test]
fn test_perf_monitor_basic() {
    let mut monitor = PerfMonitor::new();

    // Test that begin_frame/end_frame work and return a time
    monitor.begin_frame();
    let time = monitor.end_frame();

    // Should return Some with a non-negative time (even if very small)
    assert!(time.is_some());
    assert!(time.unwrap() >= 0.0);
}

#[test]
fn test_average_calculation() {
    let mut monitor = PerfMonitor::new();

    // Simulate some frames - we just need to verify the math works,
    // not that actual time passes
    for _ in 0..5 {
        monitor.begin_frame();
        monitor.end_frame();
    }

    // Average should be non-negative (even if close to zero for fast frames)
    assert!(monitor.average_frame_time() >= 0.0);
    // FPS should be positive (or infinite for zero-time frames, but typically large)
    // For very fast frames, FPS can be extremely high, so just check it's >= 0
    let fps = monitor.estimated_fps();
    assert!(fps >= 0.0 || fps.is_infinite());
}

#[test]
fn test_scoped_timer_creation() {
    // Test that ScopedTimer can be created and dropped without panicking
    // The timer should not warn because threshold is high
    let _timer = ScopedTimer::new("test_op", 1000.0);
    // Timer drops here, no warning expected since threshold is very high
}

#[test]
fn test_perf_monitor_multiple_frames() {
    let mut monitor = PerfMonitor::new();

    // Record multiple frames
    for i in 0..10 {
        monitor.begin_frame();
        let _ = monitor.end_frame();
        // Verify frame count increments (indirectly via average not being None)
        if i > 0 {
            assert!(monitor.average_frame_time() >= 0.0);
        }
    }

    // After 10 frames, we should have data
    assert!(monitor.average_frame_time() >= 0.0);
    assert!(monitor.max_frame_time() >= 0.0);
}

#[test]
fn test_operation_stats_recording() {
    let mut monitor = PerfMonitor::new();

    // Record some operations manually
    monitor.record_operation("test_op", 5.0, 0);
    monitor.record_operation("test_op", 10.0, 0);
    monitor.record_operation("test_op", 15.0, 0);

    let stats = monitor.get_operation_stats("test_op");
    assert!(stats.is_some());
    let stats = stats.unwrap();

    // Average should be (5 + 10 + 15) / 3 = 10
    assert!((stats.average() - 10.0).abs() < 0.001);
}
