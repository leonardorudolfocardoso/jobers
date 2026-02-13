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
        let store = JobStore::new();
        let job1 = Job::new("job1", "echo test1");
        let job2 = Job::new("job2", "echo test2");

        let store = store.with_job(job1.clone()).unwrap();
        let store = store.with_job(job2.clone()).unwrap();

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
    let store = JobStore::new();
    let job1 = Job::new("job1", "echo 1");
    let job2 = Job::new("job2", "echo 2");

    let store = store.with_job(job1).unwrap();
    let store = store.with_job(job2).unwrap();

    let json = serde_json::to_string(&store).unwrap();
    let deserialized: JobStore = serde_json::from_str(&json).unwrap();

    assert_eq!(store.jobs().count(), deserialized.jobs().count());
}

#[test]
fn test_functional_job_store_operations() {
    // Test the functional nature of JobStore
    let store1 = JobStore::new();
    let job = Job::new("test", "echo test");

    // with_job returns a new store, original is consumed
    let store2 = store1.with_job(job.clone()).unwrap();
    assert_eq!(store2.jobs().count(), 1);

    // without_job returns a new store
    let store3 = store2.without_job("test").unwrap();
    assert_eq!(store3.jobs().count(), 0);
}

#[test]
fn test_list_formatting_integration() {
    with_temp_storage(|temp| {
        // Create a job store with multiple jobs (unsorted)
        let store = JobStore::new()
            .with_job(Job::new("zebra", "cmd zebra"))
            .unwrap()
            .with_job(Job::new("apple", "cmd apple"))
            .unwrap()
            .with_job(Job::new("middle", "cmd middle"))
            .unwrap();

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
