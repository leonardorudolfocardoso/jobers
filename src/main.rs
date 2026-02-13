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
            println!("Listing jobs...");
            if verbose {
                println!("(verbose mode)");
            }
            // TODO: Implement job listing
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
            println!("Showing details for job: {}", name);
            // TODO: Implement job details display
        }
    }

    Ok(())
}
