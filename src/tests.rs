use crate::history::{HistoryStore, Status};
use crate::job::{Job, JobStore};
use crate::storage::Storable;
use std::fs;
use tempfile::TempDir;

/// Helper to create a temporary storage directory and override the home dir
fn with_temp_storage<F>(test: F)
where
    F: FnOnce(&TempDir),
{
    let temp = TempDir::new().unwrap();
    test(&temp);
}

#[test]
fn test_save_and_load_job_store() {
    with_temp_storage(|temp| {
        // Create a job store with some jobs
        let mut store = JobStore::new();
        let job1 = Job::new("job1", "echo test1");
        let job2 = Job::new("job2", "echo test2");

        store.add_job(job1.clone()).unwrap();
        store.add_job(job2.clone()).unwrap();

        // Save to a temp location
        let path = temp.path().join(JobStore::storage_filename());
        let json = serde_json::to_string_pretty(&store).unwrap();
        fs::write(&path, json).unwrap();

        // Load it back
        let loaded_store: JobStore = serde_json::from_str(
            &fs::read_to_string(&path).unwrap()
        ).unwrap();

        // Verify
        assert_eq!(loaded_store.jobs().count(), 2);
        assert_eq!(loaded_store.get_job("job1"), Some(&job1));
        assert_eq!(loaded_store.get_job("job2"), Some(&job2));
    });
}

#[test]
fn test_job_serialization() {
    let job = Job::new("test-job", "echo hello");

    let json = serde_json::to_string(&job).unwrap();
    let deserialized: Job = serde_json::from_str(&json).unwrap();

    assert_eq!(job, deserialized);
}

#[test]
fn test_job_store_serialization() {
    let mut store = JobStore::new();
    let job1 = Job::new("job1", "echo 1");
    let job2 = Job::new("job2", "echo 2");

    store.add_job(job1).unwrap();
    store.add_job(job2).unwrap();

    let json = serde_json::to_string(&store).unwrap();
    let deserialized: JobStore = serde_json::from_str(&json).unwrap();

    assert_eq!(store.jobs().count(), deserialized.jobs().count());
}

#[test]
fn test_job_store_mutation_operations() {
    // Test the mutable nature of JobStore
    let mut store = JobStore::new();
    let job = Job::new("test", "echo test");

    // add_job mutates the store
    store.add_job(job.clone()).unwrap();
    assert_eq!(store.jobs().count(), 1);

    // remove_job mutates the store
    store.remove_job("test").unwrap();
    assert_eq!(store.jobs().count(), 0);
}

#[test]
fn test_list_formatting_integration() {
    with_temp_storage(|temp| {
        // Create a job store with multiple jobs (unsorted)
        let mut store = JobStore::new();
        store.add_job(Job::new("zebra", "cmd zebra")).unwrap();
        store.add_job(Job::new("apple", "cmd apple")).unwrap();
        store.add_job(Job::new("middle", "cmd middle")).unwrap();

        // Save to temp storage
        let path = temp.path().join(JobStore::storage_filename());
        let json = serde_json::to_string_pretty(&store).unwrap();
        fs::write(&path, json).unwrap();

        // Load it back and verify listing works
        let loaded_store: JobStore =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();

        // Use jobs_sorted() to get sorted jobs
        let jobs = loaded_store.jobs_sorted();

        // Verify sorted order and data integrity
        assert_eq!(jobs.len(), 3);
        assert_eq!(jobs[0].name, "apple");
        assert_eq!(jobs[0].command, "cmd apple");
        assert_eq!(jobs[1].name, "middle");
        assert_eq!(jobs[1].command, "cmd middle");
        assert_eq!(jobs[2].name, "zebra");
        assert_eq!(jobs[2].command, "cmd zebra");
    });
}

#[test]
fn test_history_tracking() {
    with_temp_storage(|_temp| {
        // Create and record run
        let mut history_store = HistoryStore::new();
        history_store.update_last_run("test", Status::Success);

        // Verify initial state
        let history = history_store.get("test").unwrap();
        assert_eq!(history.run_count(), 1);
        assert_eq!(history.last_run().status, Status::Success);

        // Record another run (replaces previous run details)
        history_store.update_last_run("test", Status::Failure { exit_code: 1 });
        let history = history_store.get("test").unwrap();
        assert_eq!(history.run_count(), 2);
        // Previous Success status is replaced with Failure
        assert_eq!(history.last_run().status, Status::Failure { exit_code: 1 });
    });
}

#[test]
fn test_history_persistence() {
    with_temp_storage(|temp| {
        // Save history
        let mut history_store = HistoryStore::new();
        history_store.update_last_run("job1", Status::Success);
        history_store.update_last_run("job2", Status::Failure { exit_code: 42 });

        let path = temp.path().join(HistoryStore::storage_filename());
        let json = serde_json::to_string_pretty(&history_store).unwrap();
        fs::write(&path, json).unwrap();

        // Load and verify
        let loaded: HistoryStore =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();

        assert!(loaded.get("job1").is_some());
        assert_eq!(loaded.get("job1").unwrap().run_count(), 1);
        assert!(loaded.get("job2").is_some());
    });
}
