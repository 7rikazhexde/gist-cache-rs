# Default recipe (run when no recipe specified)
default: check

# Run all checks (format, lint, test)
check: fmt-check lint test

# Check code formatting without making changes
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter with all warnings as errors
lint:
    cargo clippy --all-targets -- -D warnings

# Run tests with minimal output
test:
    cargo test --quiet

# Format code automatically
fmt:
    cargo fmt --all

# Run all tests with verbose output
test-verbose:
    cargo test -- --nocapture

# Build release binary
build-release:
    cargo build --release

# CI check with stricter settings
ci-check:
    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings
    RUSTFLAGS="-D warnings" cargo test --all
