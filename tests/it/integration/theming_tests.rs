//! Theming Integration Tests

use humanboard::notifications::{Toast, ToastVariant};

#[test]
fn test_toast_variant_colors() {
    // Test that all variants have distinct color mappings
    let success = ToastVariant::Success;
    let error = ToastVariant::Error;
    let info = ToastVariant::Info;
    let warning = ToastVariant::Warning;

    // Each variant should have an icon
    assert_eq!(success.icon(), "✓");
    assert_eq!(error.icon(), "✗");
    assert_eq!(info.icon(), "ℹ");
    assert_eq!(warning.icon(), "⚠");
}

#[test]
fn test_toast_variant_durations() {
    use std::time::Duration;

    assert_eq!(ToastVariant::Success.default_duration(), Duration::from_secs(3));
    assert_eq!(ToastVariant::Info.default_duration(), Duration::from_secs(3));
    assert_eq!(ToastVariant::Warning.default_duration(), Duration::from_secs(4));
    assert_eq!(ToastVariant::Error.default_duration(), Duration::from_secs(5));
}

#[test]
fn test_toast_creation() {
    let success = Toast::success("Operation completed");
    assert!(matches!(success.variant, ToastVariant::Success));

    let error = Toast::error("Something went wrong");
    assert!(matches!(error.variant, ToastVariant::Error));

    let info = Toast::info("Here's some information");
    assert!(matches!(info.variant, ToastVariant::Info));

    let warning = Toast::warning("Be careful!");
    assert!(matches!(warning.variant, ToastVariant::Warning));
}

#[test]
fn test_toast_custom_duration() {
    use std::time::Duration;

    let toast = Toast::info("Test").with_duration(Duration::from_secs(10));
    assert_eq!(toast.duration, Duration::from_secs(10));
}

/// This test verifies that the expiration logic works correctly over time.
/// It is marked as ignored because it requires actual time to pass,
/// making it slow and potentially flaky in CI environments.
///
/// To run: cargo test test_toast_expiry -- --ignored
#[test]
#[ignore]
fn test_toast_expiry() {
    use std::time::Duration;

    let toast = Toast::info("Quick").with_duration(Duration::from_millis(1));

    // Wait for expiry
    std::thread::sleep(Duration::from_millis(10));

    assert!(toast.is_expired());
}

#[test]
fn test_toast_opacity_fresh() {
    // Fresh toast should have full opacity
    let fresh = Toast::info("Fresh");
    assert_eq!(fresh.opacity(false), 1.0);
}

#[test]
fn test_toast_opacity_with_reduce_motion() {
    // With reduce_motion enabled, opacity should always be 1.0
    let toast = Toast::info("Test");
    assert_eq!(toast.opacity(true), 1.0);
}

/// This test verifies opacity fading over time.
/// It is marked as ignored because it requires actual time to pass.
///
/// To run: cargo test test_toast_opacity_fade_over_time -- --ignored
#[test]
#[ignore]
fn test_toast_opacity_fade_over_time() {
    use std::time::Duration;

    // Expired toast should have low/zero opacity
    let expired = Toast::info("Old").with_duration(Duration::from_millis(1));
    std::thread::sleep(Duration::from_millis(10));
    assert!(expired.opacity(false) < 0.1);
}

#[test]
fn test_toast_remaining_percent() {
    use std::time::Duration;

    let toast = Toast::info("Test").with_duration(Duration::from_secs(10));

    // Initially should be close to 100%
    let remaining = toast.remaining_percent();
    assert!(remaining > 0.99);
}
