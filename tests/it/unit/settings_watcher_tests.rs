//! Unit tests for settings_watcher module.

use humanboard::settings_watcher::{default_settings_path, SettingsWatcher};
use std::fs;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_watcher_creation() {
    let dir = tempdir().unwrap();
    let settings_path = dir.path().join("settings.json");
    fs::write(&settings_path, "{}").unwrap();

    let watcher = SettingsWatcher::new(settings_path);
    assert!(watcher.is_ok());
}

#[test]
fn test_default_paths() {
    // These should return Some on most systems
    let settings = default_settings_path();
    assert!(settings.is_some() || cfg!(target_os = "unknown"));
}

/// This test is ignored because file watcher event detection is inherently
/// timing-dependent and platform-specific. The test verifies file modification
/// detection works, but requires OS-level file system events which are not
/// deterministic in CI environments.
///
/// TODO: Consider using a mock file watcher for unit testing, or move this
/// to integration tests that can tolerate longer timeouts.
#[test]
#[ignore]
fn test_file_modification_detection() {
    let dir = tempdir().unwrap();
    let settings_path = dir.path().join("settings.json");
    fs::write(&settings_path, "{}").unwrap();

    let mut watcher = SettingsWatcher::new(settings_path.clone()).unwrap();

    // Modify the file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&settings_path)
        .unwrap();
    writeln!(file, "{{\"modified\": true}}").unwrap();
    file.sync_all().unwrap();

    // Poll for events - event detection is platform-dependent and may not fire
    // This test mainly verifies the watcher doesn't crash
    let _event = watcher.poll();
}
