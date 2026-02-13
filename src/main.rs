use clap::{Parser, Subcommand};

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

fn main() {
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
            println!("Adding job '{}' with command: {}", name, command);
            // TODO: Implement job addition
        }
        Commands::Remove { name } => {
            println!("Removing job: {}", name);
            // TODO: Implement job removal
        }
        Commands::Show { name } => {
            println!("Showing details for job: {}", name);
            // TODO: Implement job details display
        }
    }
}
