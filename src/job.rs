use crate::storage::Storable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

    pub fn without_job(self, name: &str) -> (Self, Option<Job>) {
        let mut jobs = self.jobs;
        let removed = jobs.remove(name);
        (Self { jobs }, removed)
    }

    pub fn jobs(&self) -> impl Iterator<Item = &Job> {
        self.jobs.values()
    }
}

impl Storable for JobStore {
    fn storage_filename() -> &'static str {
        "jobs.json"
    }
}
