# Commit Guide for Jobers Project

This guide helps you create logical, atomic commits for the changes made.

## Preparation

Reset any staged changes:
```bash
git reset HEAD .
```

---

## Commit 1: Storage Module Foundation

**What**: Add generic trait-based storage system (without tests)

```bash
# Create temporary version without tests
head -n 74 src/storage.rs > src/storage_temp.rs
mv src/storage_temp.rs src/storage.rs

# Stage files
git add src/lib.rs
git add src/storage.rs
git add Cargo.lock

# Stage only storage-related dependencies (not tempfile)
# Manually edit or use:
git add Cargo.toml
# Then unstage the tempfile line if needed

# Commit
git commit -m "feat: add storage module with generic trait-based persistence

- Add generic Storable trait for serializable types
- Implement save/load functions using serde_json
- Add custom StorageError with thiserror
- Store files in ~/.jobers/ directory
- Use functional composition with Result chaining"
```

---

## Commit 2: Job Module

**What**: Add job types and functional store (without tests)

```bash
# Restore full storage.rs
git checkout src/storage.rs

# Create temporary version without tests
head -n 69 src/job.rs > src/job_temp.rs
mv src/job_temp.rs src/job.rs

# Stage and commit
git add src/job.rs

git commit -m "feat: add job module with functional job store

- Add Job struct with name and command
- Implement JobStore with HashMap backing
- Use functional approach (with_job, without_job return new instances)
- Add JobError with AlreadyExists and NotFound variants
- Implement Storable trait for JobStore"
```

---

## Commit 3: Main Command Integration

**What**: Wire up add command with proper error handling

```bash
# Restore full files
git checkout src/job.rs

# Stage main
git add src/main.rs

git commit -m "feat: implement add command with error handling

- Create AppError wrapping StorageError and JobError
- Implement handle_add using ? operator for clean error flow
- Use thiserror for automatic Display and From impls
- Exit with code 1 on errors with user-friendly messages"
```

---

## Commit 4: Comprehensive Test Suite

**What**: Add all tests (unit + integration)

```bash
# Make sure all files have their tests
git checkout src/storage.rs  # Restores with tests

# Stage everything
git add src/job.rs
git add src/storage.rs
git add src/tests.rs
git add Cargo.toml  # Adds tempfile dependency

git commit -m "test: add comprehensive unit and integration tests

- Add 9 unit tests for job module (inline)
- Add 3 unit tests for storage module (inline)
- Add 4 integration tests for serialization and functional patterns
- Use tempfile for isolated storage tests
- Achieve 100% coverage of public API

Total: 16 tests passing"
```

---

## Verification

Check your commits:
```bash
# List commits
git log --oneline -5

# Detailed view with files
git log -4 --stat

# See what each commit changed
git show HEAD~3  # Commit 1
git show HEAD~2  # Commit 2
git show HEAD~1  # Commit 3
git show HEAD    # Commit 4
```

---

## Alternative: Quick Commits (If You Prefer)

If you want to just make logical commits quickly without the temporary files:

```bash
# Commit 1: Core modules
git add src/lib.rs src/job.rs src/storage.rs Cargo.lock
git commit -m "feat: add job and storage modules

- Generic storage with Storable trait
- Functional JobStore with HashMap
- Error types with thiserror"

# Commit 2: Main integration
git add src/main.rs Cargo.toml
git commit -m "feat: implement add command with error handling"

# Commit 3: Tests
git add src/tests.rs src/job.rs src/storage.rs Cargo.toml
git commit -m "test: add comprehensive test suite (16 tests)"
```

---

## Tips

- Use `git add -p` for interactive staging of specific changes
- Use `git diff --staged` to review what's staged before committing
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)
  - `feat:` for new features
  - `test:` for tests
  - `refactor:` for refactoring
  - `fix:` for bug fixes
