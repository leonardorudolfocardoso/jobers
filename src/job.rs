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

    /// Return a new JobStore with the job or an error if it already exists
    pub fn with_job(self, job: Job) -> Result<Self, JobError> {
        if self.jobs.contains_key(&job.name) {
            Err(JobError::AlreadyExists(job.name.clone()))
        } else {
            let mut jobs = self.jobs;
            jobs.insert(job.name.clone(), job);
            Ok(Self { jobs })
        }
    }

    pub fn get_job(&self, name: &str) -> Option<&Job> {
        self.jobs.get(name)
    }

    /// Return a new JobStore without the job or an error if not found
    pub fn without_job(self, name: &str) -> Result<Self, JobError> {
        if !self.jobs.contains_key(name) {
            return Err(JobError::NotFound(name.to_string()));
        }
        let mut jobs = self.jobs;
        jobs.remove(name);
        Ok(Self { jobs })
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

    pub fn clear(self) -> Self {
        Self::new()
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
    fn test_job_store_new() {
        let store = JobStore::new();
        assert_eq!(store.jobs().count(), 0);
    }

    #[test]
    fn test_with_job_adds_job() {
        let store = JobStore::new();
        let job = Job::new("test", "echo test");

        let result = store.with_job(job.clone());
        assert!(result.is_ok());

        let store = result.unwrap();
        assert_eq!(store.jobs().count(), 1);
        assert_eq!(store.get_job("test"), Some(&job));
    }

    #[test]
    fn test_with_job_rejects_duplicate() {
        let store = JobStore::new();
        let job1 = Job::new("test", "echo test");
        let job2 = Job::new("test", "echo duplicate");

        let store = store.with_job(job1).unwrap();
        let result = store.with_job(job2);

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
        let store = JobStore::new();
        let job1 = Job::new("job1", "echo 1");
        let job2 = Job::new("job2", "echo 2");

        let store = store.with_job(job1).unwrap();
        let store = store.with_job(job2).unwrap();

        let jobs: Vec<_> = store.jobs().collect();
        assert_eq!(jobs.len(), 2);
    }

    #[test]
    fn test_without_job_removes_existing() {
        let store = JobStore::new();
        let job = Job::new("test", "echo test");
        let store = store.with_job(job).unwrap();

        let result = store.without_job("test");
        assert!(result.is_ok());

        let store = result.unwrap();
        assert_eq!(store.jobs().count(), 0);
        assert_eq!(store.get_job("test"), None);
    }

    #[test]
    fn test_without_job_fails_for_missing() {
        let store = JobStore::new();
        let result = store.without_job("nonexistent");

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
    }

    #[test]
    fn test_is_empty() {
        let store = JobStore::new();
        assert!(store.is_empty());

        let store = store.with_job(Job::new("test", "echo test")).unwrap();
        assert!(!store.is_empty());

        let store = store.without_job("test").unwrap();
        assert!(store.is_empty());
    }

    #[test]
    fn test_jobs_sorted() {
        let store = JobStore::new()
            .with_job(Job::new("zebra", "cmd zebra"))
            .unwrap()
            .with_job(Job::new("apple", "cmd apple"))
            .unwrap()
            .with_job(Job::new("middle", "cmd middle"))
            .unwrap();

        let sorted = store.jobs_sorted();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].name, "apple");
        assert_eq!(sorted[1].name, "middle");
        assert_eq!(sorted[2].name, "zebra");
    }

    #[test]
    fn test_len() {
        let store = JobStore::new();
        assert_eq!(store.len(), 0);

        let store = store.with_job(Job::new("job1", "cmd1")).unwrap();
        assert_eq!(store.len(), 1);

        let store = store.with_job(Job::new("job2", "cmd2")).unwrap();
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_clear() {
        let store = JobStore::new()
            .with_job(Job::new("job1", "cmd1"))
            .unwrap()
            .with_job(Job::new("job2", "cmd2"))
            .unwrap();

        assert_eq!(store.len(), 2);

        let store = store.clear();
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());
    }
}
