# Test Inventory - Classification and Overview of All Tests

**Created Date**: 2025-11-06
**Total Number of Tests**: 153
**Coverage**: 68.95% (533/773 lines)

---

## Test Classification

### Test Pyramid Structure

| Test Type | Count | Location |
|---|---|---|
| **Unit Tests** | 120 | `src/` within `#[cfg(test)]` |
| **Integration Tests** | 33 | `tests/` directory |
| **E2E Tests** | 26 cases | `docs/tests/` (Manual) |
| **Total** | **153** | - |

---

## 1. Unit Tests (120)

### 1.1 cache/types.rs (11)

**Location**: `src/cache/types.rs` within `#[cfg(test)]` module

**Purpose**: Data structure serialization/deserialization, type conversion

| Test Name | Purpose | Importance |
|---|---|---|
| `test_gist_cache_serialization` | GistCache JSON conversion | ⭐⭐⭐ |
| `test_datetime_serialization` | Datetime serialization (without sub-seconds) | ⭐⭐⭐ |
| `test_datetime_deserialization` | Datetime deserialization | ⭐⭐⭐ |
| `test_gist_info_without_description` | Handling Gist without description | ⭐⭐ |
| `test_github_gist_to_gist_info` | GitHub API → Internal type conversion | ⭐⭐⭐ |
| `test_gist_file_clone` | GistFile clone | ⭐ |
| `test_cache_metadata` | Metadata structure | ⭐⭐ |
| Others | Basic type operations | ⭐ |

**Coverage**: 100% (15/15 lines)

**Windows Support**: ✅ Fully supported (no file I/O dependencies)

---

### 1.2 config.rs (5)

**Location**: `src/config.rs` within `#[cfg(test)]` module

**Purpose**: Configuration management, platform-specific path generation

| Test Name | Purpose | Importance |
|---|---|---|
| `test_config_new` | Config initialization | ⭐⭐⭐ |
| `test_config_default` | Default settings | ⭐⭐⭐ |
| `test_ensure_cache_dir` | Cache directory creation | ⭐⭐⭐ |
| `test_ensure_download_dir` | Download directory creation | ⭐⭐ |
| `test_cache_exists` | Cache file existence check | ⭐⭐ |

**Coverage**: 96.15% (25/26 lines)

**Windows Support**: ✅ Fully supported (tests platform-specific path generation)

---

### 1.3 cache/content.rs (18)

**Location**: `src/cache/content.rs` within `#[cfg(test)]` module

**Purpose**: Content cache read/write, directory management

| Test Name | Purpose | Importance |
|---|---|---|
| `test_write_and_read` | Basic read/write | ⭐⭐⭐ |
| `test_read_nonexistent_file` | Error handling for non-existent files | ⭐⭐⭐ |
| `test_write_creates_gist_directory` | Automatic directory creation | ⭐⭐⭐ |
| `test_overwrite_existing_file` | Overwriting existing file | ⭐⭐ |
| `test_multiple_files_in_same_gist` | Multiple file management | ⭐⭐⭐ |
| `test_delete_gist` | Deleting Gist cache | ⭐⭐ |
| `test_delete_gist_already_deleted` | Handling already deleted Gist | ⭐ |
| `test_list_cached_gists` | Listing cached Gists | ⭐⭐ |
| `test_list_cached_gists_when_no_cache_dir` | Handling no cache directory | ⭐⭐ |
| `test_list_cached_gists_with_file_in_contents_dir` | Handling unexpected files | ⭐⭐ |
| `test_total_size` | Calculating cache size | ⭐⭐ |
| `test_total_size_when_no_cache_dir` | Size when no directory | ⭐ |
| `test_clear_all` | Clearing all caches | ⭐⭐ |
| `test_clear_all_when_empty` | Clearing empty directory | ⭐ |
| `test_cache_path_generation` | Path generation accuracy | ⭐⭐ |
| `test_self_healing_unexpected_files` | Self-healing of anomalous files | ⭐⭐ |
| 2 Others | Edge cases | ⭐ |

**Coverage**: 83.54% (66/79 lines)

**Windows Support**: ✅ Fully supported (cross-platform with tempfile)

---

### 1.4 cli.rs (21)

**Location**: `src/cli.rs` within `#[cfg(test)]` module

**Purpose**: CLI argument parsing, interpreter analysis, cache commands

| Test Name | Purpose | Importance |
|---|---|---|
| `test_parse_interpreter_bash` | Bash interpreter parsing | ⭐⭐⭐ |
| `test_parse_interpreter_python` | Python interpreter parsing | ⭐⭐⭐ |
| `test_parse_interpreter_node` | Node interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_ruby` | Ruby interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_php` | PHP interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_perl` | Perl interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_pwsh` | PowerShell Core interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_powershell` | Windows PowerShell interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_ts_node` | ts-node interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_deno` | Deno interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_bun` | Bun interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_uv` | uv interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_poetry` | Poetry interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_sh` | sh interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_zsh` | zsh interpreter parsing | ⭐⭐ |
| `test_parse_interpreter_python_alias` | python3 alias | ⭐⭐ |
| `test_parse_interpreter_none` | No interpreter | ⭐⭐ |
| `test_parse_interpreter_custom_invalid` | Invalid custom interpreter | ⭐⭐⭐ |
| `test_format_bytes` | Byte formatting | ⭐⭐ |
| `test_format_bytes_edge_cases` | Byte boundary values | ⭐⭐ |
| `test_format_bytes_multiple_gb` | GB unit display | ⭐ |
| `test_handle_cache_command_list_empty` | Empty cache list | ⭐⭐ |
| `test_handle_cache_command_list_with_cache` | Cache list display | ⭐⭐⭐ |
| `test_handle_cache_command_list_no_metadata` | List without metadata | ⭐⭐ |
| `test_handle_cache_command_size` | Cache size display | ⭐⭐ |
| `test_handle_cache_command_clean` | Cache clean (unimplemented) | ⭐ |
| `test_run_gist_cache_not_found` | Cache not found error | ⭐⭐⭐ |
| `test_run_gist_no_results` | No search results error | ⭐⭐⭐ |
| `test_run_gist_with_filename_search` | Filename search mode | ⭐⭐⭐ |
| `test_run_gist_with_description_search` | Description search mode | ⭐⭐⭐ |
| `test_print_run_help` | Help display | ⭐ |

**Coverage**: 78.16% (161/206 lines)

**Windows Support**: ✅ Fully supported

---

### 1.5 search/query.rs (26)

**Location**: `src/search/query.rs` within `#[cfg(test)]` module

**Purpose**: Search functionality, ID/filename/description search

| Test Name | Purpose | Importance |
|---|---|---|
| `test_search_by_id` | ID search | ⭐⭐⭐ |
| `test_search_id_no_match` | ID search no match | ⭐⭐ |
| `test_search_by_filename` | Filename search | ⭐⭐⭐ |
| `test_search_by_filename_case_insensitive` | Case-insensitive filename search | ⭐⭐⭐ |
| `test_search_filename_multiple_files` | Multiple file Gist search | ⭐⭐⭐ |
| `test_search_by_description` | Description search | ⭐⭐⭐ |
| `test_search_by_description_case_insensitive` | Case-insensitive description search | ⭐⭐⭐ |
| `test_search_by_description_no_description` | Gist without description | ⭐⭐ |
| `test_search_both` | Filename + description search | ⭐⭐⭐ |
| `test_search_both_filename_only_match` | Filename only match | ⭐⭐ |
| `test_search_both_description_only_match` | Description only match | ⭐⭐ |
| `test_search_both_multiple_matches` | Multiple matches | ⭐⭐ |
| `test_search_both_no_description` | Both search when no description | ⭐⭐ |
| `test_search_mode_auto_detects_id` | Auto: ID detection | ⭐⭐⭐ |
| `test_search_mode_auto_detects_both` | Auto: Keyword detection | ⭐⭐⭐ |
| `test_search_mode_auto_with_short_hex` | Auto: 31-char hex | ⭐⭐⭐ |
| `test_search_mode_auto_with_non_hex` | Auto: Non-hexadecimal | ⭐⭐⭐ |
| `test_search_with_auto_mode_id` | Auto mode: ID | ⭐⭐ |
| `test_search_with_auto_mode_keyword` | Auto mode: Keyword | ⭐⭐ |
| `test_search_no_results` | No search results | ⭐⭐⭐ |
| `test_search_empty_gist_list` | Empty Gist list | ⭐⭐ |
| `test_select_from_single_result` | Selection of single result | ⭐⭐ |
| `test_select_from_empty_results` | Selection from empty results | ⭐⭐ |
| 3 Others | Edge cases | ⭐ |

**Coverage**: 70.59% (48/68 lines)

**Windows Support**: ✅ Fully supported

---

### 1.6 cache/update.rs (16)

**Location**: `src/cache/update.rs` within `#[cfg(test)]` module

**Purpose**: Cache update logic, differential update, MockGitHubClient

| Test Name | Purpose | Importance |
|---|---|---|
| `test_updater_new` | CacheUpdater initialization | ⭐⭐ |
| `test_save_and_load_cache` | Cache save/load | ⭐⭐⭐ |
| `test_load_cache_missing_file` | Handling missing file | ⭐⭐⭐ |
| `test_save_cache_invalid_json` | Detecting invalid JSON | ⭐⭐ |
| `test_update_force_with_mock` | Force update (Mock) | ⭐⭐⭐ |
| `test_update_differential_with_mock` | Differential update (Mock) | ⭐⭐⭐ |
| `test_update_with_no_changes` | Handling no updates | ⭐⭐ |
| `test_update_with_rate_limit_warning` | Rate limit warning | ⭐⭐⭐ |
| `test_update_auth_failure` | Authentication failure error | ⭐⭐⭐ |
| `test_update_with_gist_modification_deletes_cache` | Cache deletion upon Gist modification | ⭐⭐⭐ |
| `test_cache_updater_with_verbose` | Verbose mode | ⭐⭐ |
| `test_update_verbose_mode` | Verbose detailed logs | ⭐⭐ |
| `test_update_differential_with_existing_cache_verbose` | Verbose differential update | ⭐⭐ |
| `test_update_with_low_rate_limit_verbose` | Verbose low rate limit | ⭐⭐ |
| `test_update_gist_modification_verbose` | Verbose Gist update | ⭐⭐ |
| `test_update_force_verbose_without_existing_cache` | Verbose new cache | ⭐⭐ |

**Coverage**: 62.24% (89/143 lines)

**Windows Support**: ✅ Fully supported

---

### 1.7 execution/runner.rs (10)

**Location**: `src/execution/runner.rs` within `#[cfg(test)]` module

**Purpose**: Basic functionality of ScriptRunner (file selection, option settings)

| Test Name | Purpose | Importance |
|---|---|---|
| `test_runner_new` | ScriptRunner initialization | ⭐⭐ |
| `test_run_options` | RunOptions settings | ⭐⭐ |
| `test_run_options_combinations` | Option combinations | ⭐⭐ |
| `test_run_options_preview_mode` | Preview mode | ⭐⭐ |
| `test_run_options_download_mode` | Download mode | ⭐⭐ |
| `test_select_main_file_single_file` | Single file selection | ⭐⭐⭐ |
| `test_select_main_file_multiple_files` | Multiple file selection | ⭐⭐⭐ |
| `test_select_main_file_by_interpreter` | Interpreter-specific selection | ⭐⭐⭐ |
| `test_select_main_file_with_explicit_filename` | Explicit filename specification | ⭐⭐⭐ |
| `test_runner_with_different_interpreters` | Behavior of each interpreter | ⭐⭐ |
| `test_display_info` | Information display | ⭐ |

**Coverage**: 19.88% → 69.01% (improved by integration tests)

**Windows Support**: ✅ Fully supported (excluding execution part)

---

### 1.8 github/api.rs (1)

**Location**: `src/github/api.rs` within `#[cfg(test)]` module

**Purpose**: Initialization of GitHubApi struct (execution tests are #[ignore])

| Test Name | Purpose | Importance |
|---|---|---|
| `test_api_structure` | Basic structure functionality | ⭐ |
| `test_check_auth_when_authenticated` (#[ignore]) | gh authentication check | ⭐⭐⭐ |
| `test_get_user` (#[ignore]) | Get username | ⭐⭐ |
| `test_check_rate_limit` (#[ignore]) | Rate limit check | ⭐⭐ |
| `test_fetch_gists_without_since` (#[ignore]) | Get all Gists | ⭐⭐⭐ |
| `test_fetch_gists_with_since` (#[ignore]) | Get differential Gists | ⭐⭐⭐ |

**Coverage**: 8.33% (5/60 lines)

**Windows Support**: ⚠️ Partial support (requires gh CLI)

---

### 1.9 github/client.rs (5)

**Location**: `src/github/client.rs` within `#[cfg(test)]` module

**Purpose**: MockGitHubClient operation verification

| Test Name | Purpose | Importance |
|---|---|---|
| `test_mock_check_auth_success` | Mock authentication success | ⭐⭐ |
| `test_mock_get_user` | Mock user retrieval | ⭐⭐ |
| `test_mock_check_rate_limit` | Mock rate limit | ⭐⭐ |
| `test_mock_fetch_gists` | Mock Gist retrieval | ⭐⭐ |
| `test_mock_fetch_gist_content` | Mock content retrieval | ⭐⭐ |

**Coverage**: 100% (MockGitHubClient implementation)

**Windows Support**: ✅ Fully supported

---

### 1.10 error.rs (4)

**Location**: `src/error.rs` within `#[cfg(test)]` module

**Purpose**: Error type conversion, display

| Test Name | Purpose | Importance |
|---|---|---|
| `test_error_display` | Error display | ⭐⭐ |
| `test_error_from_io` | IO error conversion | ⭐⭐ |
| `test_error_from_json` | JSON error conversion | ⭐⭐ |
| `test_error_from_reqwest` | HTTP error conversion | ⭐ |
| `test_result_type_alias` | Result type alias | ⭐ |

**Coverage**: 0% (Error types tested at runtime)

**Windows Support**: ✅ Fully supported

---

## 2. Integration Tests (12)

### 2.1 tests/integration_test.rs (6)

**Location**: `tests/integration_test.rs`

**Purpose**: Verification of actual interpreter execution

**Dependencies**: bash, python3, node (Unix environment recommended)

| Test Name | Purpose | fixture | Importance |
|---|---|---|---|
| `test_execute_bash_script` | Bash execution | hello.sh | ⭐⭐⭐ |
| `test_execute_python_script` | Python execution | hello.py | ⭐⭐⭐ |
| `test_execute_node_script` | Node.js execution | hello.js | ⭐⭐ |
| `test_execute_with_arguments` | Argument passing | args_echo.sh | ⭐⭐⭐ |
| `test_execute_failing_script` | Error detection | error_exit.sh | ⭐⭐⭐ |
| `test_preview_mode_does_not_execute` | Preview mode | hello.sh | ⭐⭐⭐ |

**Contribution to Coverage**: execution/runner.rs +30-40%

**Windows Support**: ⚠️ Conditional

- bash: Requires Git Bash or WSL
- python3: Requires Windows Python
- node: Requires Windows Node.js

---

### 2.2 tests/runner_test.rs (6)

**Location**: `tests/runner_test.rs`

**Purpose**: Detailed verification of ScriptRunner operations

**Dependencies**: bash (Unix environment recommended)

| Test Name | Purpose | fixture | Importance |
|---|---|---|---|
| `test_download_mode_creates_file` | Download feature | hello.sh | ⭐⭐⭐ |
| `test_preview_with_download_mode` | Preview + Download | hello.sh | ⭐⭐ |
| `test_cache_creation_after_execution` | Cache creation | hello.sh | ⭐⭐⭐ |
| `test_multiple_files_gist` | Multiple file selection | hello.sh | ⭐⭐⭐ |
| `test_force_file_based_execution` | File-based execution | hello.sh | ⭐⭐ |
| `test_script_with_empty_arguments` | Empty argument handling | hello.sh | ⭐⭐ |

**Contribution to Coverage**: execution/runner.rs +10-15%

**Windows Support**: ⚠️ Conditional (requires bash)

| `args_echo.sh` | Argument passing test | `echo "Arguments: $@"` |
| `error_exit.sh` | Error test | `exit 1` |

**Windows Support**: ⚠️ Newline code attention (LF recommended)

---

## 3. E2E Tests (26 Cases, Manual)

### 3.1 E2E Test Overview

**What is E2E Testing?**: Verification of complete user workflow by actually executing the binary from the command line

**Reason for Not Implemented Yet**: Lower priority than integration tests

**Planned Implementation**:

```bash
tests/e2e/
├── test_full_workflow.sh        # Complete flow of update → run
├── test_cache_management.sh     # cache list/size/clear
└── test_option_combinations.sh  # --force, --preview, --download
```

**Expected Benefits**:

- Full assurance of user experience
- Integration verification between modules
- Automation of manual tests

---

## Test Classification Summary

| Classification | Number of Tests | Coverage Contribution | Windows Support | Status |
|---|---|---|---|---|
| **Unit Tests** | 120 | 57-60% | ✅ Full | ✅ Completed |
| **Integration Tests** | 12 | +10-11% | ⚠️ Conditional | ✅ Completed |
| **E2E Tests** | 26 cases | +2-3% | ⚠️ Conditional | 📝 Completed |
| **Total** | 132 | 68.18% | - | - |

---

## Test Importance Criteria

| Mark | Meaning | Description |
|---|---|---|
| ⭐⭐⭐ | Essential | Directly leads to data loss or critical bugs |
| ⭐⭐ | Recommended | Affects user experience or major features |
| ⭐ | Optional | Edge cases or rare situations |

---

## Coverage Contribution

### High Contribution (10% or more)

- **Integration Tests (tests/integration_test.rs + runner_test.rs)**: +10.87%
  - Covers execution part of execution/runner.rs

### Medium Contribution (5-10%)

- **cli.rs Unit Tests**: Contributes approx. 5-6% to overall
- **search/query.rs Unit Tests**: Contributes approx. 4-5% to overall

### Low Contribution (1-5%)

- Individual unit tests for each module

---

## Test Implementation History

### Phase 1-5 (Unit Tests): 120

- cache/types.rs: 11
- config.rs: 5
- cache/content.rs: 18
- cli.rs: 21
- search/query.rs: 26
- cache/update.rs: 16
- execution/runner.rs: 10
- github/client.rs: 5
- error.rs: 4
- Others: 4

**Achieved Coverage**: 57.31%

### Phase 6 (Integration Tests): 12

- tests/integration_test.rs: 6
- tests/runner_test.rs: 6
- tests/fixtures/: 5 files

**Achieved Coverage**: 68.18% (+10.87%)

### Phase 7 (E2E): 26 Cases (Manual Execution)

**Expected Coverage**: 70-71% (+2-3%)

---

**Last Updated**: 2025-11-06
**Coverage**: 68.95% (533/773 lines)
