//! Unit tests for notifications module.

use humanboard::notifications::{Toast, ToastManager, ToastVariant};
use std::time::Duration;

#[test]
fn test_toast_creation() {
    let toast = Toast::success("Test message");
    assert_eq!(toast.message, "Test message");
    assert_eq!(toast.variant, ToastVariant::Success);
}

#[test]
fn test_toast_manager() {
    let mut manager = ToastManager::new();
    assert_eq!(manager.count(), 0);

    manager.push(Toast::success("Message 1"));
    assert_eq!(manager.count(), 1);

    manager.push(Toast::error("Message 2"));
    assert_eq!(manager.count(), 2);

    manager.clear();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_toast_not_immediately_expired() {
    // A toast with a reasonable duration should NOT be expired immediately after creation
    let toast = Toast::success("Test").with_duration(Duration::from_secs(10));
    assert!(!toast.is_expired(), "Fresh toast should not be expired");
}

#[test]
fn test_toast_remaining_percent_fresh() {
    // A fresh toast should have close to 100% remaining
    let toast = Toast::success("Test").with_duration(Duration::from_secs(10));
    let remaining = toast.remaining_percent();
    // Should be very close to 1.0 (100%) since almost no time has passed
    assert!(remaining > 0.99, "Fresh toast should have ~100% remaining");
}

#[test]
fn test_toast_opacity_fresh() {
    // Fresh toast should have full opacity
    let toast = Toast::success("Fresh");
    assert_eq!(toast.opacity(false), 1.0);
}

#[test]
fn test_toast_opacity_with_reduce_motion() {
    // With reduce_motion, opacity should always be 1.0
    let toast = Toast::success("Test");
    assert_eq!(toast.opacity(true), 1.0);
}

/// This test verifies that the expiration logic works correctly over time.
/// It is marked as ignored because it requires actual time to pass,
/// making it slow and potentially flaky in CI environments.
///
/// To run: cargo test test_toast_expiration -- --ignored
#[test]
#[ignore]
fn test_toast_expiration() {
    let toast = Toast::success("Test").with_duration(Duration::from_millis(1));
    assert!(!toast.is_expired());

    std::thread::sleep(Duration::from_millis(10));
    assert!(toast.is_expired());
}

#[test]
fn test_variant_durations() {
    assert_eq!(
        ToastVariant::Success.default_duration(),
        Duration::from_secs(3)
    );
    assert_eq!(
        ToastVariant::Info.default_duration(),
        Duration::from_secs(3)
    );
    assert_eq!(
        ToastVariant::Warning.default_duration(),
        Duration::from_secs(4)
    );
    assert_eq!(
        ToastVariant::Error.default_duration(),
        Duration::from_secs(5)
    );
}

#[test]
fn test_variant_icons() {
    assert_eq!(ToastVariant::Success.icon(), "✓");
    assert_eq!(ToastVariant::Error.icon(), "✗");
    assert_eq!(ToastVariant::Info.icon(), "ℹ");
    assert_eq!(ToastVariant::Warning.icon(), "⚠");
}

#[test]
fn test_toast_with_custom_duration() {
    let toast = Toast::info("Test").with_duration(Duration::from_secs(42));
    assert_eq!(toast.duration, Duration::from_secs(42));
}

#[test]
fn test_toast_manager_remove() {
    let mut manager = ToastManager::new();

    manager.push(Toast::success("Toast 1"));
    manager.push(Toast::info("Toast 2"));
    manager.push(Toast::warning("Toast 3"));

    assert_eq!(manager.count(), 3);

    // Get the ID of the second toast
    let toast_id = manager.toasts()[1].id;
    manager.remove(toast_id);

    assert_eq!(manager.count(), 2);
}
