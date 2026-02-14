//! Job execution history tracking.
//!
//! Tracks the last run of each job (status, timestamp, run count).
//! Only the most recent run details are kept.

use crate::storage::Storable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HistoryError {
    #[error("Failed to record run history for job '{0}'")]
    RecordFailed(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Success,
    Failure { exit_code: i32 },
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Success => write!(f, "Success"),
            Status::Failure { exit_code } => write!(f, "Failed (exit code: {})", exit_code),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub status: Status,
    pub timestamp: SystemTime,
}

impl Run {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            timestamp: SystemTime::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    last_run: Run,
    run_count: u32,
}

impl History {
    pub fn new(status: Status) -> Self {
        Self {
            last_run: Run::new(status),
            run_count: 1,
        }
    }

    /// Update the last run and increment counter.
    pub fn update_last_run(&mut self, status: Status) {
        self.last_run = Run::new(status);
        self.run_count += 1;
    }

    pub fn last_run(&self) -> &Run {
        &self.last_run
    }

    pub fn run_count(&self) -> u32 {
        self.run_count
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryStore {
    jobs: HashMap<String, History>,
}

impl HistoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the last run for a job.
    pub fn update_last_run(&mut self, job_name: impl Into<String>, status: Status) {
        self.jobs
            .entry(job_name.into())
            .and_modify(|history| history.update_last_run(status))
            .or_insert_with(|| History::new(status));
    }

    pub fn get(&self, job_name: &str) -> Option<&History> {
        self.jobs.get(job_name)
    }

    pub fn remove_job(&mut self, job_name: &str) {
        self.jobs.remove(job_name);
    }

    pub fn clear(&mut self) {
        self.jobs.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
}

impl Storable for HistoryStore {
    fn storage_filename() -> &'static str {
        "history.json"
    }
}

/// Format SystemTime as relative time string
pub fn format_timestamp(time: &SystemTime) -> String {
    match SystemTime::now().duration_since(*time) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / 86400;
            let hours = (secs % 86400) / 3600;
            let minutes = (secs % 3600) / 60;
            let seconds = secs % 60;

            if days > 0 {
                format!("{} days ago", days)
            } else if hours > 0 {
                format!("{} hours ago", hours)
            } else if minutes > 0 {
                format!("{} minutes ago", minutes)
            } else {
                format!("{} seconds ago", seconds)
            }
        }
        Err(_) => "Unknown time".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_display() {
        assert_eq!(Status::Success.to_string(), "Success");
        assert_eq!(
            Status::Failure { exit_code: 1 }.to_string(),
            "Failed (exit code: 1)"
        );
        assert_eq!(
            Status::Failure { exit_code: 127 }.to_string(),
            "Failed (exit code: 127)"
        );
    }

    #[test]
    fn test_run_creation() {
        let run = Run::new(Status::Success);
        assert_eq!(run.status, Status::Success);
        // timestamp should be recent
        assert!(run.timestamp.elapsed().unwrap().as_secs() < 1);
    }

    #[test]
    fn test_history_creation() {
        let history = History::new(Status::Success);
        assert_eq!(history.run_count(), 1);
        assert_eq!(history.last_run().status, Status::Success);
    }

    #[test]
    fn test_history_update_replaces_previous_run() {
        let mut history = History::new(Status::Success);
        history.update_last_run(Status::Failure { exit_code: 1 });

        assert_eq!(history.run_count(), 2);
        assert_eq!(history.last_run().status, Status::Failure { exit_code: 1 });
    }

    #[test]
    fn test_history_store_update_creates_first_run() {
        let mut store = HistoryStore::new();
        store.update_last_run("test", Status::Success);

        let history = store.get("test").unwrap();
        assert_eq!(history.run_count(), 1);
        assert_eq!(history.last_run().status, Status::Success);
    }

    #[test]
    fn test_history_store_update_replaces_existing_run() {
        let mut store = HistoryStore::new();
        store.update_last_run("test", Status::Success);
        store.update_last_run("test", Status::Failure { exit_code: 42 });

        let history = store.get("test").unwrap();
        assert_eq!(history.run_count(), 2);
        assert_eq!(history.last_run().status, Status::Failure { exit_code: 42 });
    }

    #[test]
    fn test_update_last_run_discards_previous_run_details() {
        let mut history = History::new(Status::Success);

        // First run was successful
        assert_eq!(history.last_run().status, Status::Success);
        assert_eq!(history.run_count(), 1);

        // Second run fails - this replaces the first run's details
        history.update_last_run(Status::Failure { exit_code: 42 });

        // Counter incremented
        assert_eq!(history.run_count(), 2);

        // Previous run's Success status is gone, replaced with Failure
        assert_eq!(history.last_run().status, Status::Failure { exit_code: 42 });

        // Note: We can no longer see that the first run was successful.
        // This is intentional - only the last run details are kept.
    }

    #[test]
    fn test_history_store_get_missing() {
        let store = HistoryStore::new();
        assert!(store.get("nonexistent").is_none());
    }

    #[test]
    fn test_history_store_remove_job() {
        let mut store = HistoryStore::new();
        store.update_last_run("test", Status::Success);
        store.remove_job("test");

        assert!(store.get("test").is_none());
    }

    #[test]
    fn test_history_store_clear() {
        let mut store = HistoryStore::new();
        store.update_last_run("job1", Status::Success);
        store.update_last_run("job2", Status::Success);
        store.clear();

        assert!(store.is_empty());
        assert!(store.get("job1").is_none());
        assert!(store.get("job2").is_none());
    }

    #[test]
    fn test_format_timestamp_recent() {
        let now = SystemTime::now();
        let formatted = format_timestamp(&now);
        // Should say seconds ago or minutes ago for very recent times
        assert!(formatted.contains("ago"));
    }
}
