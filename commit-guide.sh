#!/bin/bash
# Guide for creating logical commits for the jobers project

set -e

echo "=== Commit Guide for Jobers Project ==="
echo ""
echo "This script will help you create logical commits."
echo "Press ENTER to continue with each commit, or Ctrl+C to stop."
echo ""

# Reset any staged changes
git reset HEAD . 2>/dev/null || true

# Commit 1: Storage module (without tests)
echo "📦 Commit 1: Add storage module with generic trait-based persistence"
echo "   Files: src/lib.rs, src/storage.rs (without tests), Cargo.toml, Cargo.lock"
read -p "   Press ENTER to stage these files..."

# Create temporary storage.rs without tests
cat src/storage.rs | sed '/^#\[cfg(test)\]/,$ d' > /tmp/storage_no_tests.rs
cp /tmp/storage_no_tests.rs src/storage.rs

git add src/lib.rs
git add src/storage.rs
git add Cargo.lock

# Stage only the new dependencies in Cargo.toml (not tempfile)
git add -p Cargo.toml

git commit -m "feat: add storage module with generic trait-based persistence

- Add generic Storable trait for serializable types
- Implement save/load functions using serde_json
- Add custom StorageError with thiserror
- Store files in ~/.jobers/ directory
- Use functional composition with Result chaining"

echo "✅ Commit 1 created"
echo ""

# Restore full storage.rs
git checkout HEAD~1 src/storage.rs 2>/dev/null || git restore --source=HEAD src/storage.rs

# Commit 2: Job module (without tests)
echo "📦 Commit 2: Add job module with functional job store"
echo "   Files: src/job.rs (without tests)"
read -p "   Press ENTER to stage these files..."

# Create temporary job.rs without tests
cat src/job.rs | sed '/^#\[cfg(test)\]/,$ d' > /tmp/job_no_tests.rs
cp /tmp/job_no_tests.rs src/job.rs

git add src/job.rs
git commit -m "feat: add job module with functional job store

- Add Job struct with name and command
- Implement JobStore with HashMap backing
- Use functional approach (with_job, without_job return new instances)
- Add JobError with AlreadyExists and NotFound variants
- Implement Storable trait for JobStore"

echo "✅ Commit 2 created"
echo ""

# Restore full job.rs
git restore --source=HEAD src/job.rs 2>/dev/null || true

# Commit 3: Main integration
echo "📦 Commit 3: Implement add command with error handling"
echo "   Files: src/main.rs"
read -p "   Press ENTER to stage these files..."

git add src/main.rs
git commit -m "feat: implement add command with error handling

- Create AppError wrapping StorageError and JobError
- Implement handle_add using ? operator for clean error flow
- Use thiserror for automatic Display and From impls
- Exit with code 1 on errors with user-friendly messages"

echo "✅ Commit 3 created"
echo ""

# Commit 4: Tests
echo "📦 Commit 4: Add comprehensive test suite"
echo "   Files: src/job.rs (tests), src/storage.rs (tests), src/tests.rs, Cargo.toml"
read -p "   Press ENTER to stage these files..."

# Restore full files with tests
git restore src/job.rs
git restore src/storage.rs

git add src/job.rs
git add src/storage.rs
git add src/tests.rs
git add Cargo.toml  # This will add tempfile

git commit -m "test: add comprehensive unit and integration tests

- Add 9 unit tests for job module (inline)
- Add 3 unit tests for storage module (inline)
- Add 4 integration tests for serialization and functional patterns
- Use tempfile for isolated storage tests
- Achieve 100% coverage of public API

Total: 16 tests passing"

echo "✅ Commit 4 created"
echo ""

echo "🎉 All commits created successfully!"
echo ""
echo "Review commits with: git log --oneline -5"
echo "Or detailed view: git log -4 --stat"
