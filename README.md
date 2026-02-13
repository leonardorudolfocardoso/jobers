# Jobers

A lightweight, functional CLI tool for managing and running shell commands as jobs.

## Overview

Jobers lets you save frequently used commands as named "jobs" and run them with optional arguments. Think of it as bookmarks for your shell commands, but with the flexibility to parameterize them at runtime.

Built in Rust with a focus on simplicity, type safety, and functional programming patterns.

## Features

- **Job Management** - Add, list, show, remove, and clear jobs
- **Shell Execution** - Run jobs through your system shell (supports pipes, redirects, variables)
- **Runtime Arguments** - Pass additional arguments to jobs when running them
- **Exit Code Propagation** - Job exit codes propagate for scripting integration
- **Cross-Platform** - Works on Unix (Linux, macOS) and Windows
- **Persistent Storage** - Jobs stored as JSON in `~/.jobers/jobs.json`
- **Type-Safe** - Comprehensive error handling with descriptive messages

## Installation

### From Source

```bash
git clone https://github.com/yourusername/jobers.git
cd jobers
cargo build --release
sudo cp target/release/jobers /usr/local/bin/
```

## Quick Start

```bash
# Add a job
jobers add hello "echo Hello, World!"

# List jobs
jobers list

# Run a job
jobers run hello

# Show job details
jobers show hello

# Remove a job
jobers remove hello
```

## Usage

### Add a Job

Save a command as a named job:

```bash
jobers add <name> <command>
```

**Examples:**

```bash
# Simple command
jobers add greet "echo Hello"

# Command with pipes
jobers add search "grep -r TODO . | head -5"

# Parameterizable command (use with arguments later)
jobers add backup "rsync -av"
```

### List Jobs

Display all saved jobs:

```bash
# Compact format (names only)
jobers list

# Verbose format (with commands)
jobers list --verbose
jobers list -v
```

### Run a Job

Execute a saved job:

```bash
jobers run <name> [args...]
```

**Examples:**

```bash
# Run without arguments
jobers run greet

# Run with arguments
jobers add copy "rsync -av"
jobers run copy /source /destination

# Exit code propagation
jobers add test "cargo test"
jobers run test && echo "Tests passed!"
```

**Notes:**
- Commands execute through the system shell (`/bin/sh` on Unix, `cmd` on Windows)
- Arguments are appended to the stored command
- Exit codes are propagated (success returns 0, failures return non-zero)

### Show Job Details

Display detailed information about a job:

```bash
jobers show <name>
```

**Example:**

```bash
$ jobers show backup
Job: backup
Command: rsync -av
```

### Remove a Job

Delete a job:

```bash
jobers remove <name>
```

**Example:**

```bash
jobers remove backup
✓ Removed job 'backup'
```

### Clear All Jobs

Remove all saved jobs:

```bash
# Interactive confirmation
jobers clear

# Skip confirmation
jobers clear --yes
jobers clear -y
```

## Use Cases

### 1. Complex Commands

Save complex commands you use frequently:

```bash
jobers add logs "docker-compose logs -f --tail=100 | grep ERROR"
jobers run logs
```

### 2. Parameterized Commands

Create reusable command templates:

```bash
jobers add deploy "kubectl apply -f"
jobers run deploy deployment.yaml
jobers run deploy service.yaml
```

### 3. Multi-Step Workflows

Combine with shell scripting:

```bash
jobers add build "cargo build --release"
jobers add test "cargo test"

# In a script
jobers run test && jobers run build && echo "Ready to deploy"
```

### 4. Project-Specific Tasks

Store project-specific commands:

```bash
jobers add dev "npm run dev"
jobers add lint "npm run lint -- --fix"
jobers add db-reset "psql -U postgres -c 'DROP DATABASE mydb; CREATE DATABASE mydb;'"
```

### 5. Common File Operations

Save file operation patterns:

```bash
jobers add find-large "find . -type f -size +100M"
jobers add clean-logs "find /var/log -name '*.log' -mtime +30 -delete"
```

## Architecture

Jobers follows clean architecture principles with clear separation of concerns:

```
src/
├── lib.rs           # Library entry point
├── main.rs          # CLI application (presentation layer)
├── job.rs           # Domain model (Job, JobStore, JobError)
├── storage.rs       # Storage abstraction (Storable trait)
└── tests.rs         # Integration tests
```

**Key Design Patterns:**

- **Functional Programming** - Immutable operations, method chaining
- **Error Handling** - Result types with thiserror for ergonomic errors
- **Dependency Inversion** - Storage trait for decoupling
- **Type Safety** - Comprehensive error types, no unwrap() in production code

## Technical Details

### Storage

Jobs are stored in `~/.jobers/jobs.json`:

```json
{
  "jobs": {
    "hello": {
      "name": "hello",
      "command": "echo Hello, World!"
    }
  }
}
```

### Error Handling

Jobers uses comprehensive error handling:

- `JobError::AlreadyExists` - Job name already in use
- `JobError::NotFound` - Job doesn't exist
- `JobError::ExecutionFailed` - Command execution failed
- `StorageError::*` - File I/O or serialization errors

### Shell Execution

Commands execute through the system shell:
- **Unix/Linux/macOS**: `/bin/sh -c "command"`
- **Windows**: `cmd /c "command"`

This enables:
- Pipes: `ls | grep test`
- Redirects: `echo hello > file.txt`
- Variables: `echo $HOME`
- Globs: `rm *.tmp`

## Development

### Prerequisites

- Rust 2024 Edition or later
- Cargo

### Build

```bash
cargo build
```

### Test

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_handle_run
```

### Code Quality

The project maintains high code quality standards:

- **30 tests** with comprehensive coverage
- **No unwrap()** in production code
- **Functional patterns** throughout
- **Type-safe** error handling
- **Clean architecture** with clear layers

## Dependencies

### Production
- `clap` - CLI argument parsing
- `serde` + `serde_json` - Serialization
- `dirs` - Home directory detection
- `thiserror` - Error handling

### Development
- `tempfile` - Temporary files for testing

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Code Style** - Follow existing functional patterns
2. **Tests** - Add tests for new features
3. **Errors** - Use proper error types, no unwrap() in production code
4. **Commits** - Use Conventional Commits format

## Roadmap

Potential future enhancements:

- [ ] Job categories/tags
- [ ] Environment variable substitution
- [ ] Working directory support
- [ ] Job history/logs
- [ ] Import/export job collections
- [ ] Shell completion (bash, zsh, fish)
- [ ] Job templates with placeholders

## License

MIT License - see LICENSE file for details

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [clap](https://github.com/clap-rs/clap) - Command-line argument parser
- [serde](https://serde.rs/) - Serialization framework

---

**Note:** Jobers was built as a learning project exploring functional programming patterns in Rust, with assistance from Claude (Anthropic's AI assistant).
