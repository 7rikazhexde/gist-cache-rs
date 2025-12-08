# Code Coverage Measurement Guide

## Current Coverage Status

**Overall Coverage**: 68.95% (533/773 lines)
**Number of Automated Tests**: 153 (Unit 120 + Integration 33)
**Target**: 60-70% ✅ Achieved

### Module-wise Coverage

| Module | Coverage | Covered/Total | Status |
|---|---|---|---|
| `cache/types.rs` | 100.00% | 15/15 | ✅ Perfect |
| `config.rs` | 96.15% | 25/26 | ✅ Excellent |
| `cache/content.rs` | 83.54% | 66/79 | ✅ Good |
| `cli.rs` | 78.16% | 161/206 | ✅ Good |
| `search/query.rs` | 70.59% | 48/68 | 🟡 Good |
| `cache/update.rs` | 62.24% | 89/143 | 🟡 Improvement needed |
| `execution/runner.rs` | 19.88% | 34/171 | 🔴 Many external dependencies |
| `github/api.rs` | 8.33% | 5/60 | 🔴 Many external dependencies |
| `error.rs` | 0.00% | 0/1 | 🟡 No test required |
| `main.rs` | 0.00% | 0/4 | 🟡 Verified by E2E |

---

## Coverage Measurement Commands

### Basic Measurement

```bash
# Display coverage to standard output
cargo tarpaulin --out Stdout

# Generate HTML report
cargo tarpaulin --out Html --output-dir coverage

# Generate both
cargo tarpaulin --out Html --out Stdout --output-dir coverage
```

### Detailed Measurement

```bash
# With detailed output (coverage information per line)
cargo tarpaulin --out Stdout --verbose

# Timeout setting (for large projects)
cargo tarpaulin --out Stdout --timeout 120

# Display only the last 100 lines (abbreviates long output)
cargo tarpaulin --out Stdout 2>&1 | tail -100
```

### Module-wise Measurement

```bash
# Test only specific modules
cargo tarpaulin --out Stdout --lib

# Exclude specific files
cargo tarpaulin --out Stdout --exclude-files 'tests/*'

# Run only specific tests
cargo tarpaulin --out Stdout --test integration_test
```

### Measurement via justfile

```bash
# If coverage task is already added to justfile
just coverage

# Or directly
just check  # lint + test (no coverage)
```

---

## Coverage Design Philosophy

### High Coverage Modules (70% or more)

Core business logic maintains high coverage:

- **cache/types.rs (100%)**: Data structures, serialization
- **config.rs (96%)**: Configuration management, platform-specific paths
- **cache/content.rs (84%)**: Content cache management
- **cli.rs (78%)**: CLI argument processing
- **search/query.rs (71%)**: Search logic

### Medium Coverage Modules (50-70%)

Affected by external dependencies, but business logic covered by mocks:

- **cache/update.rs (62%)**: Tested with mocked GitHubClient

### Low Coverage Modules (less than 50%)

Highly dependent on external processes/commands, quality ensured by integration tests:

- **execution/runner.rs (20%)**: Actual execution of bash, python, etc. → 12 languages verified by integration tests
- **github/api.rs (8%)**: `gh` command execution → Replaced by MockGitHubClient
- **main.rs (0%)**: Entry point → Verified by E2E tests
- **error.rs (0%)**: Simple type definition → Tested at runtime

---

## How to Read Coverage Reports

### Interpretation of Standard Output

```bash
|| Uncovered Lines:
|| src/cache/content.rs: 88, 90, 116, 118, ...
```

This indicates which lines in each file are not covered.

### HTML Report

In the HTML report (`coverage/index.html`):

- Green: Covered lines
- Red: Uncovered lines
- Gray: Unexecutable lines (comments, blank lines, etc.)

To open in a browser:

```bash
# For Linux
xdg-open coverage/index.html

# For macOS
open coverage/index.html

# For Windows
start coverage/index.html
```

---

## Usage in CI/CD

### GitHub Actions

```yaml
- name: Run tests with coverage
  run: cargo tarpaulin --out Xml --output-dir coverage

- name: Upload coverage to Codecov
  uses: codecov/codecov-action@v3
  with:
    files: ./coverage/cobertura.xml
```

---

## Troubleshooting

### cargo-tarpaulin is not installed

```bash
cargo install cargo-tarpaulin
```

### Timeout error

```bash
# Extend timeout
cargo tarpaulin --out Stdout --timeout 300
```

### Out of memory

```bash
# Limit parallel execution
cargo tarpaulin --out Stdout --jobs 1
```

---

## References

- [cargo-tarpaulin Official Documentation](https://github.com/xd009642/tarpaulin)
- [TESTING.md](./TESTING.md) - Test Strategy and Execution Guide
- [TEST_INVENTORY.md](./TEST_INVENTORY.md) - Classification and Overview of All Tests

---

**Last Updated**: 2025-11-06
**Current Coverage**: 68.95%
**Number of Automated Tests**: 153
**Covered Lines**: 533/773 lines
