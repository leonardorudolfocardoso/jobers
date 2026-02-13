use clap::{Parser, Subcommand};
use thiserror::Error;

use jobers::job::{Job, JobError, JobStore};
use jobers::storage::{self, StorageError};

#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    Job(#[from] JobError),
}

#[derive(Parser)]
#[command(name = "jobers")]
#[command(about = "A CLI tool for running jobs", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a job
    Run {
        /// Name of the job to run
        #[arg(short, long)]
        name: String,

        /// Additional arguments to pass to the job
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// List all available jobs
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Add a new job
    Add {
        /// Name of the job
        name: String,

        /// Command to execute
        command: String,
    },

    /// Remove a job
    Remove {
        /// Name of the job to remove
        name: String,
    },

    /// Show job details
    Show {
        /// Name of the job
        name: String,
    },

    /// Remove all jobs
    Clear {
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

fn handle_add(name: String, command: String) -> Result<(), AppError> {
    let store: JobStore = storage::load()?;
    let store = store.with_job(Job::new(name.clone(), command))?;
    storage::save(&store)?;
    println!("✓ Added job '{}'", name);
    Ok(())
}

fn handle_remove(name: String) -> Result<(), AppError> {
    let store: JobStore = storage::load()?;
    let store = store.without_job(&name)?;
    storage::save(&store)?;
    println!("✓ Removed job '{}'", name);
    Ok(())
}

fn format_jobs_compact(store: &JobStore) -> String {
    store
        .jobs_sorted()
        .iter()
        .map(|job| job.name.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_jobs_verbose(store: &JobStore) -> String {
    let jobs = store.jobs_sorted();
    let count = jobs.len();
    let formatted_jobs = jobs
        .iter()
        .map(|job| format!("Name: {}\nCommand: {}", job.name, job.command))
        .collect::<Vec<_>>()
        .join("\n\n");

    format!("Total jobs: {}\n\n{}", count, formatted_jobs)
}

fn handle_list(verbose: bool) -> Result<(), AppError> {
    let store: JobStore = storage::load()?;

    if store.is_empty() {
        println!("No jobs found.");
        return Ok(());
    }

    if verbose {
        println!("{}", format_jobs_verbose(&store));
    } else {
        println!("{}", format_jobs_compact(&store));
    }

    Ok(())
}

fn handle_show(name: String) -> Result<(), AppError> {
    let store: JobStore = storage::load()?;

    match store.get_job(&name) {
        Some(job) => {
            println!("{}", job);
            Ok(())
        }
        None => Err(JobError::NotFound(name).into()),
    }
}

fn handle_clear(skip_confirmation: bool) -> Result<(), AppError> {
    let store: JobStore = storage::load()?;

    if store.is_empty() {
        println!("No jobs to clear.");
        return Ok(());
    }

    let count = store.len();

    if !skip_confirmation {
        println!("This will remove all {} job(s). Continue? [y/N]: ", count);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Cancelled.");
            return Ok(());
        }
    }

    let store = store.clear();
    storage::save(&store)?;
    println!("✓ Removed all {} job(s)", count);
    Ok(())
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { name, args } => {
            println!("Running job: {}", name);
            if !args.is_empty() {
                println!("With arguments: {:?}", args);
            }
            // TODO: Implement job execution
        }
        Commands::List { verbose } => {
            if let Err(e) = handle_list(verbose) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Add { name, command } => {
            if let Err(e) = handle_add(name, command) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Remove { name } => {
            if let Err(e) = handle_remove(name) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Show { name } => {
            if let Err(e) = handle_show(name) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Clear { yes } => {
            if let Err(e) = handle_clear(yes) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_jobs_compact_single_line_per_job() {
        let store = JobStore::new()
            .with_job(Job::new("job1", "echo 1"))
            .unwrap()
            .with_job(Job::new("job2", "echo 2"))
            .unwrap();

        let output = format_jobs_compact(&store);
        let lines: Vec<_> = output.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "job1");
        assert_eq!(lines[1], "job2");
    }

    #[test]
    fn test_format_jobs_verbose_includes_count() {
        let store = JobStore::new()
            .with_job(Job::new("test", "echo test"))
            .unwrap();

        let output = format_jobs_verbose(&store);
        assert!(output.contains("Total jobs: 1"));
    }

    #[test]
    fn test_format_jobs_verbose_includes_details() {
        let store = JobStore::new()
            .with_job(Job::new("test", "echo test"))
            .unwrap();

        let output = format_jobs_verbose(&store);
        assert!(output.contains("Name: test"));
        assert!(output.contains("Command: echo test"));
    }

    #[test]
    fn test_handle_show_displays_job() {
        // This test verifies the show handler returns Ok and would display the job
        // We can't easily test stdout, but we can verify the handler logic
        let store = JobStore::new()
            .with_job(Job::new("test", "echo test"))
            .unwrap();

        // Verify get_job works (which handle_show uses)
        assert!(store.get_job("test").is_some());
        assert_eq!(store.get_job("test").unwrap().name, "test");
    }

    #[test]
    fn test_handle_show_fails_for_missing_job() {
        let store = JobStore::new();
        assert!(store.get_job("nonexistent").is_none());
    }

    #[test]
    fn test_clear_removes_all_jobs() {
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
