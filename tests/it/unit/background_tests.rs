//! Unit tests for background module.

use humanboard::background::{BackgroundExecutor, TaskResult};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Helper to poll for task completion with a timeout.
/// This is much faster than sleeping because it checks frequently
/// and returns as soon as the condition is met.
fn wait_for_completion<F>(executor: &BackgroundExecutor, mut condition: F, timeout: Duration) -> bool
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    while start.elapsed() < timeout {
        executor.process_results();
        if condition() {
            return true;
        }
        // Yield to allow background thread to run
        std::thread::yield_now();
    }
    // One final process_results call
    executor.process_results();
    condition()
}

#[test]
fn test_executor_creation() {
    let executor = BackgroundExecutor::new(2);
    assert!(!executor.has_pending());
    assert_eq!(executor.pending_count(), 0);
}

#[test]
fn test_spawn_and_complete() {
    let executor = BackgroundExecutor::new(1);
    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = Arc::clone(&completed);

    executor.spawn(
        "test_task",
        || Ok(42),
        move |result: TaskResult<i32>| {
            assert_eq!(result.unwrap(), 42);
            completed_clone.store(true, Ordering::SeqCst);
        },
    );

    // Poll until completed or timeout (should be nearly instant)
    let success = wait_for_completion(
        &executor,
        || completed.load(Ordering::SeqCst),
        Duration::from_secs(1),
    );

    assert!(success, "Task should have completed");
    assert!(completed.load(Ordering::SeqCst));
}

#[test]
fn test_error_handling() {
    let executor = BackgroundExecutor::new(1);
    let got_error = Arc::new(AtomicBool::new(false));
    let got_error_clone = Arc::clone(&got_error);

    executor.spawn(
        "failing_task",
        || Err::<(), _>("intentional error".to_string()),
        move |result: TaskResult<()>| {
            assert!(result.is_err());
            got_error_clone.store(true, Ordering::SeqCst);
        },
    );

    // Poll until completed or timeout
    let success = wait_for_completion(
        &executor,
        || got_error.load(Ordering::SeqCst),
        Duration::from_secs(1),
    );

    assert!(success, "Error callback should have been called");
    assert!(got_error.load(Ordering::SeqCst));
}

#[test]
fn test_multiple_tasks() {
    let executor = BackgroundExecutor::new(2);
    let counter = Arc::new(Mutex::new(0));

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        executor.spawn(
            &format!("task_{}", i),
            move || Ok(i),
            move |result: TaskResult<i32>| {
                if result.is_ok() {
                    *counter.lock().unwrap() += 1;
                }
            },
        );
    }

    // Poll until all tasks complete or timeout
    let success = wait_for_completion(
        &executor,
        || *counter.lock().unwrap() == 5,
        Duration::from_secs(2),
    );

    assert!(success, "All 5 tasks should have completed");
    assert_eq!(*counter.lock().unwrap(), 5);
}
