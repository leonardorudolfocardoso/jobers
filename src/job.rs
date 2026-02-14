use crate::storage::Storable;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobError {
    #[error("Job '{0}' already exists")]
    AlreadyExists(String),
    #[error("Job '{0}' not found")]
    NotFound(String),
    #[error("Failed to execute job '{0}': {1}")]
    ExecutionFailed(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Job {
    pub name: String,
    pub command: String,
}

impl Job {
    pub fn new(name: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
        }
    }

    /// Build the full command by appending additional arguments
    pub fn build_command(&self, args: &[String]) -> String {
        if args.is_empty() {
            self.command.clone()
        } else {
            format!("{} {}", self.command, args.join(" "))
        }
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job: {}\nCommand: {}", self.name, self.command)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JobStore {
    jobs: HashMap<String, Job>,
}

impl JobStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a job or return an error if it already exists
    pub fn add_job(&mut self, job: Job) -> Result<(), JobError> {
        use std::collections::hash_map::Entry;

        match self.jobs.entry(job.name.clone()) {
            Entry::Vacant(e) => {
                e.insert(job);
                Ok(())
            }
            Entry::Occupied(_) => Err(JobError::AlreadyExists(job.name)),
        }
    }

    pub fn get_job(&self, name: &str) -> Option<&Job> {
        self.jobs.get(name)
    }

    /// Remove a job or return an error if not found
    pub fn remove_job(&mut self, name: &str) -> Result<(), JobError> {
        self.jobs
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| JobError::NotFound(name.to_string()))
    }

    pub fn jobs(&self) -> impl Iterator<Item = &Job> {
        self.jobs.values()
    }

    pub fn jobs_sorted(&self) -> Vec<&Job> {
        let mut jobs: Vec<_> = self.jobs.values().collect();
        jobs.sort_by_key(|job| &job.name);
        jobs
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn clear(&mut self) {
        self.jobs.clear();
    }
}

impl Storable for JobStore {
    fn storage_filename() -> &'static str {
        "jobs.json"
    }
}

#[cfg(test)]
mod tests {
    use super::{Job, JobError, JobStore};

    #[test]
    fn test_job_creation() {
        let job = Job::new("test", "echo hello");
        assert_eq!(job.name, "test");
        assert_eq!(job.command, "echo hello");
    }

    #[test]
    fn test_job_display() {
        let job = Job::new("my-job", "echo test");
        let output = format!("{}", job);
        assert_eq!(output, "Job: my-job\nCommand: echo test");
    }

    #[test]
    fn test_build_command_no_args() {
        let job = Job::new("test", "echo hello");
        let cmd = job.build_command(&[]);
        assert_eq!(cmd, "echo hello");
    }

    #[test]
    fn test_build_command_with_args() {
        let job = Job::new("backup", "rsync -av");
        let cmd = job.build_command(&["src".to_string(), "dest".to_string()]);
        assert_eq!(cmd, "rsync -av src dest");
    }

    #[test]
    fn test_build_command_single_arg() {
        let job = Job::new("list", "ls");
        let cmd = job.build_command(&["-la".to_string()]);
        assert_eq!(cmd, "ls -la");
    }

    #[test]
    fn test_job_store_new() {
        let store = JobStore::new();
        assert_eq!(store.jobs().count(), 0);
    }

    #[test]
    fn test_add_job_adds_job() {
        let mut store = JobStore::new();
        let job = Job::new("test", "echo test");

        let result = store.add_job(job.clone());
        assert!(result.is_ok());

        assert_eq!(store.jobs().count(), 1);
        assert_eq!(store.get_job("test"), Some(&job));
    }

    #[test]
    fn test_add_job_rejects_duplicate() {
        let mut store = JobStore::new();
        let job1 = Job::new("test", "echo test");
        let job2 = Job::new("test", "echo duplicate");

        store.add_job(job1).unwrap();
        let result = store.add_job(job2);

        assert!(result.is_err());
        match result.unwrap_err() {
            JobError::AlreadyExists(name) => assert_eq!(name, "test"),
            _ => panic!("Expected AlreadyExists error"),
        }
    }

    #[test]
    fn test_get_job_returns_none_for_missing() {
        let store = JobStore::new();
        assert_eq!(store.get_job("nonexistent"), None);
    }

    #[test]
    fn test_jobs_iterator() {
        let mut store = JobStore::new();
        let job1 = Job::new("job1", "echo 1");
        let job2 = Job::new("job2", "echo 2");

        store.add_job(job1).unwrap();
        store.add_job(job2).unwrap();

        let jobs: Vec<_> = store.jobs().collect();
        assert_eq!(jobs.len(), 2);
    }

    #[test]
    fn test_remove_job_removes_existing() {
        let mut store = JobStore::new();
        let job = Job::new("test", "echo test");
        store.add_job(job).unwrap();

        let result = store.remove_job("test");
        assert!(result.is_ok());

        assert_eq!(store.jobs().count(), 0);
        assert_eq!(store.get_job("test"), None);
    }

    #[test]
    fn test_remove_job_fails_for_missing() {
        let mut store = JobStore::new();
        let result = store.remove_job("nonexistent");

        assert!(result.is_err());
        match result.unwrap_err() {
            JobError::NotFound(name) => assert_eq!(name, "nonexistent"),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_job_error_display() {
        let err = JobError::AlreadyExists("test".to_string());
        assert_eq!(err.to_string(), "Job 'test' already exists");

        let err = JobError::NotFound("missing".to_string());
        assert_eq!(err.to_string(), "Job 'missing' not found");

        let err = JobError::ExecutionFailed("test".to_string(), "command not found".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to execute job 'test': command not found"
        );
    }

    #[test]
    fn test_is_empty() {
        let mut store = JobStore::new();
        assert!(store.is_empty());

        store.add_job(Job::new("test", "echo test")).unwrap();
        assert!(!store.is_empty());

        store.remove_job("test").unwrap();
        assert!(store.is_empty());
    }

    #[test]
    fn test_jobs_sorted() {
        let mut store = JobStore::new();
        store.add_job(Job::new("zebra", "cmd zebra")).unwrap();
        store.add_job(Job::new("apple", "cmd apple")).unwrap();
        store.add_job(Job::new("middle", "cmd middle")).unwrap();

        let sorted = store.jobs_sorted();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].name, "apple");
        assert_eq!(sorted[1].name, "middle");
        assert_eq!(sorted[2].name, "zebra");
    }

    #[test]
    fn test_len() {
        let mut store = JobStore::new();
        assert_eq!(store.len(), 0);

        store.add_job(Job::new("job1", "cmd1")).unwrap();
        assert_eq!(store.len(), 1);

        store.add_job(Job::new("job2", "cmd2")).unwrap();
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut store = JobStore::new();
        store.add_job(Job::new("job1", "cmd1")).unwrap();
        store.add_job(Job::new("job2", "cmd2")).unwrap();

        assert_eq!(store.len(), 2);

        store.clear();
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());
    }
}
