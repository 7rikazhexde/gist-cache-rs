# Test Inventory - Classification and Overview of All Tests

**Created Date**: 2025-11-06
**Last Updated**: 2026-08-20 (test counts and test lists only — see coverage note below)
**Total Number of Tests**: 251 (Unit 190 + Integration 61); 26 additional manual E2E cases
**Coverage**: 68.95% (533/773 lines) — carried over from 2025-11-06, **not re-measured** in this update (`cargo-tarpaulin` was unavailable in the environment this update was made from); treat per-module coverage figures below as stale

---

## Test Classification

### Test Pyramid Structure

| Test Type | Count | Location |
|---|---|---|
| **Unit Tests** | 190 | `src/` within `#[cfg(test)]` |
| **Integration Tests** | 61 | `tests/` directory |
| **E2E Tests** | 26 cases | `docs/tests/` (Manual) |
| **Total** | **251** | - |

---

## 1. Unit Tests (190)

### 1.1 cache/types.rs (6)

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

**Coverage**: 100% (15/15 lines) — stale, see note above

**Windows Support**: ✅ Fully supported (no file I/O dependencies)

---

### 1.2 config.rs (8)

**Location**: `src/config.rs` within `#[cfg(test)]` module

**Purpose**: Configuration management, platform-specific path generation, interpreter mapping

| Test Name | Purpose | Importance |
|---|---|---|
| `test_config_new` | Config initialization | ⭐⭐⭐ |
| `test_config_default` | Default settings | ⭐⭐⭐ |
| `test_config_persistence` | Save/load round-trip to disk | ⭐⭐⭐ |
| `test_ensure_cache_dir` | Cache directory creation | ⭐⭐⭐ |
| `test_ensure_download_dir` | Download directory creation | ⭐⭐ |
| `test_cache_exists` | Cache file existence check | ⭐⭐ |
| `test_legacy_single_interpreter_config` | Backward-compat single-interpreter config | ⭐⭐⭐ |
| `test_set_nested_interpreter_config` | Extension-based interpreter mapping (`defaults.interpreter.<ext>`) | ⭐⭐⭐ |

**Coverage**: 96.15% (25/26 lines) — stale, see note above

**Windows Support**: ✅ Fully supported (tests platform-specific path generation)

---

### 1.3 cache/content.rs (23)

**Location**: `src/cache/content.rs` within `#[cfg(test)]` module

**Purpose**: Content cache read/write, directory management, `cache clean`/`cache size`

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
| `test_calculate_dir_size` | Directory size helper backing `cache size` | ⭐⭐ |
| `test_clear_all` | Clearing all caches | ⭐⭐ |
| `test_clear_all_when_empty` | Clearing empty directory | ⭐ |
| `test_cache_path_generation` | Path generation accuracy | ⭐⭐ |
| `test_self_healing_unexpected_files` | Self-healing of anomalous files | ⭐⭐ |
| `test_clean_with_dry_run` | `cache clean --dry-run` preview (no deletion) | ⭐⭐⭐ |
| `test_clean_with_older_than` | `cache clean --older-than` criterion | ⭐⭐⭐ |
| `test_clean_with_orphaned` | `cache clean --orphaned` criterion | ⭐⭐⭐ |
| `test_clean_with_both_criteria` | Combined `--older-than` + `--orphaned` | ⭐⭐ |
| `test_clean_with_no_criteria` | No criteria specified (no-op/error) | ⭐⭐ |
| `test_clean_when_nothing_matches` | No entries match the given criteria | ⭐ |

**Coverage**: 83.54% (66/79 lines) — stale, see note above

**Windows Support**: ✅ Fully supported (cross-platform with tempfile)

---

### 1.4 cli.rs (45)

**Location**: `src/cli.rs` within `#[cfg(test)]` module

**Purpose**: CLI argument parsing, interpreter detection/resolution, shell completions, cache/run subcommands

| Test Name | Purpose | Importance |
|---|---|---|
| `test_parse_interpreter_bash` | Bash interpreter parsing | ⭐⭐⭐ |
| `test_parse_interpreter_python` | Python interpreter parsing | ⭐⭐⭐ |
| `test_parse_interpreter_python_alias` | python3 alias | ⭐⭐ |
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
| `test_parse_interpreter_none` | No interpreter specified | ⭐⭐ |
| `test_parse_interpreter_custom_invalid` | Invalid custom interpreter | ⭐⭐⭐ |
| `test_detect_shebang_env_format` | Shebang detection, `#!/usr/bin/env <x>` form | ⭐⭐⭐ |
| `test_detect_shebang_direct_path` | Shebang detection, direct interpreter path form | ⭐⭐⭐ |
| `test_detect_shebang_no_shebang` | No shebang present | ⭐⭐ |
| `test_get_file_extension` | Extension extraction from filename | ⭐⭐ |
| `test_language_to_interpreter` | tokei-detected language → interpreter mapping | ⭐⭐⭐ |
| `test_detect_interpreter_from_filename` | Filename heuristics (e.g. `Makefile`) | ⭐⭐⭐ |
| `test_detect_interpreter_from_config` | User config (extension-based mapping) takes priority | ⭐⭐⭐ |
| `test_format_bytes` | Byte formatting | ⭐⭐ |
| `test_format_bytes_edge_cases` | Byte boundary values | ⭐⭐ |
| `test_format_bytes_multiple_gb` | GB unit display | ⭐ |
| `test_generate_completions_bash` | Bash completion script generation | ⭐⭐⭐ |
| `test_generate_completions_zsh` | Zsh completion script generation | ⭐⭐ |
| `test_generate_completions_fish` | Fish completion script generation | ⭐⭐ |
| `test_generate_completions_powershell` | PowerShell completion script generation | ⭐⭐ |
| `test_shell_enum_values` | `Shell` enum variant equality | ⭐ |
| `test_handle_cache_command_list_empty` | Empty cache list | ⭐⭐ |
| `test_handle_cache_command_list_with_cache` | Cache list display | ⭐⭐⭐ |
| `test_handle_cache_command_list_no_metadata` | List without metadata | ⭐⭐ |
| `test_handle_cache_command_size` | Cache size display | ⭐⭐ |
| `test_handle_cache_command_clean_no_cache` | `cache clean` with no cache present | ⭐⭐ |
| `test_handle_cache_command_clean_no_criteria` | `cache clean` with no criteria flags | ⭐⭐ |
| `test_handle_cache_command_clean_with_orphaned` | `cache clean --orphaned` end-to-end | ⭐⭐⭐ |
| `test_run_gist_cache_not_found` | Cache not found error | ⭐⭐⭐ |
| `test_run_gist_no_results` | No search results error | ⭐⭐⭐ |
| `test_run_gist_with_filename_search` | Filename search mode | ⭐⭐⭐ |
| `test_run_gist_with_description_search` | Description search mode | ⭐⭐⭐ |
| `test_print_run_help` | Help display | ⭐ |

**Coverage**: 78.16% (161/206 lines) — stale, see note above

**Windows Support**: ✅ Fully supported

---

### 1.5 search/query.rs (23)

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

**Coverage**: 70.59% (48/68 lines) — stale, see note above

**Windows Support**: ✅ Fully supported

---

### 1.6 search/interactive.rs (36)

**Location**: `src/search/interactive.rs` within `#[cfg(test)]` module

**Purpose**: Interactive Gist picker — list/preview rendering, scrolling, `/` search and filter (Closes #74)

**Coverage**: Not measured (module added after the last coverage run — see note above)

**Windows Support**: ✅ Fully supported (pure logic, no terminal I/O in these tests)

#### Item text building

| Test Name | Purpose | Importance |
|---|---|---|
| `build_item_text_uses_default_when_no_description` | Fallback label when a Gist has no description | ⭐⭐ |
| `build_item_text_omits_filename_already_in_description` | Avoids duplicating a filename already in the description | ⭐⭐ |
| `build_item_text_appends_filenames_not_in_description` | Appends filenames missing from the description | ⭐⭐ |
| `build_item_text_appends_only_missing_files_from_multiple` | Only appends the subset of filenames not already present | ⭐⭐ |

#### Scroll and viewport management

| Test Name | Purpose | Importance |
|---|---|---|
| `clamp_scroll_keeps_position_within_bounds` | Scroll offset stays within valid range | ⭐⭐ |
| `clamp_scroll_never_goes_below_range_start` | Scroll offset floor | ⭐⭐ |
| `follow_cursor_keeps_scroll_when_cursor_stays_in_view` | No scroll when cursor is already visible | ⭐⭐ |
| `follow_cursor_scrolls_down_past_bottom_edge` | Scrolls down when cursor passes the bottom edge | ⭐⭐ |
| `follow_cursor_scrolls_up_past_top_edge` | Scrolls up when cursor passes the top edge | ⭐⭐ |
| `follow_cursor_never_scrolls_past_the_end` | Scroll clamps at the end of the list | ⭐⭐ |
| `follow_cursor_respects_a_sub_range_start` | Scroll respects a non-zero range start (used by preview) | ⭐⭐ |
| `active_divider_picks_the_last_divider_at_or_before_cursor` | Header/divider tracking as the cursor moves through preview | ⭐⭐ |
| `page_capacity_reserves_header_footer_and_margin` | Row budget accounts for header/footer/margin | ⭐⭐ |
| `visible_window_shows_everything_when_list_fits_budget` | No windowing needed when the list is short | ⭐⭐ |
| `visible_window_includes_selection_and_respects_row_budget` | Visible window always contains the current selection | ⭐⭐⭐ |
| `visible_window_does_not_run_past_start_of_list` | Window clamps at the start of the list | ⭐⭐ |
| `visible_window_does_not_run_past_end_of_list` | Window clamps at the end of the list | ⭐⭐ |
| `visible_window_handles_empty_list` | Empty list edge case | ⭐ |
| `visible_window_accounts_for_wrapped_rows_in_full_mode` | Window sizing accounts for line-wrapped rows | ⭐⭐ |
| `visual_row_count_wraps_at_width` | Row count reflects terminal-width wrapping | ⭐⭐ |
| `visual_row_count_handles_zero_width` | Zero-width terminal edge case | ⭐ |

#### Search and matching (`/` filter and preview search)

| Test Name | Purpose | Importance |
|---|---|---|
| `matcher_supports_regex_patterns` | Regex matching for `/` filter | ⭐⭐⭐ |
| `matcher_falls_back_to_literal_on_invalid_regex` | Invalid regex falls back to literal substring match | ⭐⭐⭐ |
| `matcher_does_plain_case_insensitive_substring_matching` | Case-insensitive plain-text matching | ⭐⭐⭐ |
| `matcher_empty_pattern_matches_everything` | Empty pattern matches all items | ⭐⭐ |
| `find_next_match_wraps_forward` | Preview search wraps to the top when searching forward | ⭐⭐ |
| `find_next_match_wraps_backward` | Preview search wraps to the bottom when searching backward (`N`) | ⭐⭐ |
| `find_next_match_returns_none_when_nothing_matches` | No match found | ⭐⭐ |
| `find_next_match_ignores_ansi_codes_in_the_line` | Search ignores syntax-highlighting ANSI escape codes | ⭐⭐⭐ |

#### Rendering (list and preview lines)

| Test Name | Purpose | Importance |
|---|---|---|
| `render_line_shows_everything_when_full` | Full-width display mode (`Tab` toggle) | ⭐ |
| `render_line_truncates_when_not_full` | Truncation in short-display mode | ⭐⭐ |
| `render_line_truncates_long_text_even_on_a_wide_terminal` | Truncation still applies on a wide terminal | ⭐⭐ |
| `render_line_marks_selected_item` | Selected row is visually marked | ⭐⭐ |
| `render_preview_line_truncates_to_fit_the_terminal_width` | Preview line truncation | ⭐⭐ |
| `render_preview_line_marks_only_the_cursor_row` | Only the cursor row is marked in preview | ⭐⭐ |
| `render_preview_line_keeps_short_content_untruncated` | Short content left untouched | ⭐ |

---

### 1.7 cache/update.rs (17)

**Location**: `src/cache/update.rs` within `#[cfg(test)]` module

**Purpose**: Cache update logic, differential update, MockGitHubClient

| Test Name | Purpose | Importance |
|---|---|---|
| `test_updater_new` | CacheUpdater initialization | ⭐⭐ |
| `test_save_and_load_cache` | Cache save/load | ⭐⭐⭐ |
| `test_load_cache_missing_file` | Handling missing file | ⭐⭐⭐ |
| `test_save_cache_invalid_json` | Detecting invalid JSON | ⭐⭐ |
| `test_cache_metadata` | Metadata round-trip within the update flow | ⭐⭐ |
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

**Coverage**: 62.24% (89/143 lines) — stale, see note above

**Windows Support**: ✅ Fully supported

---

### 1.8 execution/runner.rs (11)

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

**Coverage**: 19.88% → 69.01% (improved by integration tests) — stale, see note above

**Windows Support**: ✅ Fully supported (excluding execution part)

---

### 1.9 execution/highlight.rs (6)

**Location**: `src/execution/highlight.rs` within `#[cfg(test)]` module

**Purpose**: Syntax highlighting for `--preview` output and the interactive picker's preview pane

| Test Name | Purpose | Importance |
|---|---|---|
| `syntax_for_resolves_known_extensions` | Extension → syntect syntax lookup | ⭐⭐⭐ |
| `syntax_for_falls_back_to_shebang_when_extension_unknown` | Shebang-based syntax fallback | ⭐⭐⭐ |
| `syntax_for_does_not_touch_disk_for_nonexistent_files` | No filesystem access for a nonexistent path | ⭐⭐ |
| `highlight_content_wraps_output_in_reset_code` | Output is wrapped with an ANSI reset code | ⭐⭐ |
| `highlight_content_handles_empty_content` | Empty content edge case | ⭐ |
| `highlight_content_falls_back_for_unknown_extension` | Plain-text fallback for unrecognized syntax | ⭐⭐ |

**Coverage**: Not measured (module added after the last coverage run — see note above)

**Windows Support**: ✅ Fully supported

---

### 1.10 github/api.rs (6, 1 active + 5 `#[ignore]`)

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

**Coverage**: 8.33% (5/60 lines) — stale, see note above

**Windows Support**: ⚠️ Partial support (requires gh CLI)

---

### 1.11 github/client.rs (5)

**Location**: `src/github/client.rs` within `#[cfg(test)]` module

**Purpose**: MockGitHubClient operation verification

| Test Name | Purpose | Importance |
|---|---|---|
| `test_mock_check_auth_success` | Mock authentication success | ⭐⭐ |
| `test_mock_get_user` | Mock user retrieval | ⭐⭐ |
| `test_mock_check_rate_limit` | Mock rate limit | ⭐⭐ |
| `test_mock_fetch_gists` | Mock Gist retrieval | ⭐⭐ |
| `test_mock_fetch_gist_content` | Mock content retrieval | ⭐⭐ |

**Coverage**: 100% (MockGitHubClient implementation) — stale, see note above

**Windows Support**: ✅ Fully supported

---

### 1.12 error.rs (4)

**Location**: `src/error.rs` within `#[cfg(test)]` module

**Purpose**: Error type conversion, display

| Test Name | Purpose | Importance |
|---|---|---|
| `test_error_display` | Error display | ⭐⭐ |
| `test_error_from_io` | IO error conversion | ⭐⭐ |
| `test_error_from_json` | JSON error conversion | ⭐⭐ |
| `test_result_type_alias` | Result type alias | ⭐ |

**Coverage**: 0% (Error types tested at runtime) — stale, see note above

**Windows Support**: ✅ Fully supported

---

## 2. Integration Tests (61)

### 2.1 tests/cli_tests.rs (33; 32 compiled on Windows)

**Location**: `tests/cli_tests.rs`

**Purpose**: Verification of command-line argument processing, subcommands, and shell completions via `assert_cmd`

| Test Name | Purpose | Importance |
|---|---|---|
| `test_cli_help` | Top-level `--help` | ⭐⭐ |
| `test_cli_version` | `--version` | ⭐ |
| `test_invalid_command` | Unknown subcommand error | ⭐⭐ |
| `test_update_without_auth` | `update` without `gh` auth | ⭐⭐⭐ |
| `test_update_force` | `update --force` | ⭐⭐⭐ |
| `test_update_verbose` | `update --verbose` | ⭐⭐ |
| `test_update_verbose_without_progress` | Verbose output without a progress bar (non-TTY) | ⭐⭐ |
| `test_update_with_progress_display` | Progress bar display during update | ⭐⭐ |
| `test_run_without_cache` | `run` before any cache exists | ⭐⭐⭐ |
| `test_run_without_query` | `run` with no query argument | ⭐⭐ |
| `test_run_with_preview_flag` | `run --preview` | ⭐⭐⭐ |
| `test_run_with_filename_flag` | `run --filename` | ⭐⭐⭐ |
| `test_run_with_description_flag` | `run --description` | ⭐⭐⭐ |
| `test_run_with_id_flag` | `run --id` | ⭐⭐⭐ |
| `test_cache_list_empty` | `cache list` with no cache | ⭐⭐ |
| `test_cache_list_json_format` | `cache list --json` | ⭐⭐ |
| `test_cache_list_json_format_empty` | `cache list --json` with no cache | ⭐⭐ |
| `test_cache_size` | `cache size` | ⭐⭐ |
| `test_cache_clear_with_no_input` | `cache clear` confirmation prompt, declined | ⭐⭐ |
| `test_config_show` | `config show` | ⭐⭐ |
| `test_config_show_empty` | `config show` with no config file | ⭐⭐ |
| `test_config_show_with_content` | `config show` with existing values | ⭐⭐ |
| `test_config_set_get` | `config set` + `config get` round-trip | ⭐⭐⭐ |
| `test_config_reset` | `config reset` | ⭐⭐ |
| `test_completions_help` | `completions --help` | ⭐ |
| `test_completions_invalid_shell` | Unsupported shell argument | ⭐⭐ |
| `test_completions_bash` | `completions bash` generates a script | ⭐⭐⭐ |
| `test_completions_bash_contains_commands` | Bash script lists subcommands | ⭐⭐ |
| `test_completions_bash_subcommand_completion_not_empty` (`#[cfg(unix)]`) | Regression test for [#77](https://github.com/7rikazhexde/gist-cache-rs/pull/77): sources the generated bash script and asserts `run`/`cache`/`config`/`update` each still return completion candidates. Does not compile on Windows | ⭐⭐⭐ |
| `test_completions_zsh` | `completions zsh` generates a script | ⭐⭐ |
| `test_completions_zsh_contains_commands` | Zsh script lists subcommands | ⭐⭐ |
| `test_completions_fish` | `completions fish` generates a script | ⭐⭐ |
| `test_completions_powershell` | `completions powershell` generates a script | ⭐⭐ |

**Windows Support**: ✅ Fully supported except `test_completions_bash_subcommand_completion_not_empty`, which is Unix-only (see [Testing Guide](./testing.md))

---

### 2.2 tests/integration_test.rs (16)

**Location**: `tests/integration_test.rs`

**Purpose**: Verification of actual interpreter execution

**Dependencies**: bash/python3/node/ruby/perl/php/deno/bun/ts-node on Unix; PowerShell on Windows

| Test Name | Purpose | fixture | Importance |
|---|---|---|---|
| `test_execute_bash_script` (Unix) | Bash execution | hello.sh | ⭐⭐⭐ |
| `test_execute_python_script` (Unix) | Python execution | hello.py | ⭐⭐⭐ |
| `test_execute_node_script` (Unix) | Node.js execution | hello.js | ⭐⭐ |
| `test_execute_ruby_script` (Unix) | Ruby execution | hello.rb | ⭐⭐ |
| `test_execute_perl_script` (Unix) | Perl execution | hello.pl | ⭐⭐ |
| `test_execute_php_script` (Unix) | PHP execution | hello.php | ⭐⭐ |
| `test_execute_ts_node_script` (Unix) | ts-node execution | hello.ts | ⭐⭐ |
| `test_execute_deno_script` (Unix) | Deno execution | hello.ts | ⭐⭐ |
| `test_execute_bun_script` (Unix) | Bun execution | hello.ts | ⭐⭐ |
| `test_execute_with_arguments` (Unix) | Argument passing | args_echo.sh | ⭐⭐⭐ |
| `test_execute_failing_script` (Unix) | Error detection | error_exit.sh | ⭐⭐⭐ |
| `test_preview_mode_does_not_execute` (Unix) | Preview mode | hello.sh | ⭐⭐⭐ |
| `test_execute_powershell_script` (Windows) | PowerShell execution | hello.ps1 | ⭐⭐⭐ |
| `test_execute_powershell_with_arguments` (Windows) | Argument passing | hello.ps1 | ⭐⭐⭐ |
| `test_execute_powershell_preview_mode` (Windows) | Preview mode | hello.ps1 | ⭐⭐⭐ |
| `test_execute_powershell_failing_script` (Windows) | Error detection | hello.ps1 | ⭐⭐⭐ |

**Contribution to Coverage**: execution/runner.rs +30-40% — stale, see note above

**Windows Support**: ⚠️ Conditional — the 12 Unix tests are `#[ignore]`d on Windows (4 PowerShell tests run instead); the 4 PowerShell tests are `#[ignore]`d on Unix

---

### 2.3 tests/runner_test.rs (12)

**Location**: `tests/runner_test.rs`

**Purpose**: Detailed verification of ScriptRunner operations

**Dependencies**: bash on Unix; PowerShell on Windows

| Test Name | Purpose | fixture | Importance |
|---|---|---|---|
| `test_download_mode_creates_file` (Unix) | Download feature | hello.sh | ⭐⭐⭐ |
| `test_preview_with_download_mode` (Unix) | Preview + Download | hello.sh | ⭐⭐ |
| `test_cache_creation_after_execution` (Unix) | Cache creation | hello.sh | ⭐⭐⭐ |
| `test_multiple_files_gist` (Unix) | Multiple file selection | hello.sh | ⭐⭐⭐ |
| `test_force_file_based_execution` (Unix) | File-based execution | hello.sh | ⭐⭐ |
| `test_script_with_empty_arguments` (Unix) | Empty argument handling | hello.sh | ⭐⭐ |
| `test_powershell_download_mode` (Windows) | Download feature | hello.ps1 | ⭐⭐⭐ |
| `test_powershell_preview_with_download` (Windows) | Preview + Download | hello.ps1 | ⭐⭐ |
| `test_powershell_cache_creation` (Windows) | Cache creation | hello.ps1 | ⭐⭐⭐ |
| `test_powershell_multiple_files_gist` (Windows) | Multiple file selection | hello.ps1 | ⭐⭐⭐ |
| `test_powershell_force_file_based` (Windows) | File-based execution | hello.ps1 | ⭐⭐ |
| `test_powershell_with_empty_arguments` (Windows) | Empty argument handling | hello.ps1 | ⭐⭐ |

**Contribution to Coverage**: execution/runner.rs +10-15% — stale, see note above

**Windows Support**: ⚠️ Conditional — the 6 Unix tests are `#[ignore]`d on Windows (6 PowerShell tests run instead); the 6 PowerShell tests are `#[ignore]`d on Unix

---

## 3. E2E Tests (26 Cases, Manual)

> **Note**: The `docs/tests/` directory referenced below was not found in the repository as of this update (2026-08-20). The case list and counts below are carried over unverified from the 2025-11-06 version of this document; treat them as aspirational/planned rather than confirmed to exist.

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
| **Unit Tests** | 190 | 57-60% (stale) | ✅ Full | ✅ Completed |
| **Integration Tests** | 61 | +10-11% (stale) | ⚠️ Conditional | ✅ Completed |
| **E2E Tests** | 26 cases | +2-3% (stale) | ⚠️ Conditional | 📝 Unverified (see note in §3) |
| **Total** | 277 | 68.95% (stale) | - | - |

---

## Test Importance Criteria

| Mark | Meaning | Description |
|---|---|---|
| ⭐⭐⭐ | Essential | Directly leads to data loss or critical bugs |
| ⭐⭐ | Recommended | Affects user experience or major features |
| ⭐ | Optional | Edge cases or rare situations |

---

## Coverage Contribution

> Figures in this section are carried over from 2025-11-06 and have not been re-measured (see note at the top of this document).

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

### Phase 1-5 (Unit Tests): 120 (as of 2025-11-06)

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

### Phase 6 (Integration Tests): 12 (as of 2025-11-06)

- tests/integration_test.rs: 6
- tests/runner_test.rs: 6
- tests/fixtures/: 5 files

**Achieved Coverage**: 68.18% (+10.87%)

### Phase 7 (E2E): 26 Cases (Manual Execution, unverified — see note in §3)

**Expected Coverage**: 70-71% (+2-3%)

### Phase 8 (Unit + Integration growth to 251, as of 2026-08-20)

Between 2025-11-06 and 2026-08-20 the suite grew from 153 to 251 automated tests, driven primarily by:

- **Interactive Gist picker** (Closes #74, PR #75): added `src/search/interactive.rs` (36 tests) and `src/execution/highlight.rs` (6 tests)
- **`cache clean` command**: added 7 tests to `cache/content.rs` and 3 to `cli.rs`
- **Extension-based interpreter configuration and detection**: added tests across `config.rs`, `cli.rs`
- **Shell completions, `config` subcommands, PowerShell execution coverage**: grew `tests/cli_tests.rs` from 15 to 33 and added PowerShell-specific variants across `tests/integration_test.rs` and `tests/runner_test.rs`
- **Bash completion subcommand-completion fix** (PR #77): added 1 Unix-only regression test to `tests/cli_tests.rs`

Coverage percentages were not re-measured as part of this growth; see the note at the top of this document.

---

**Last Updated**: 2026-08-20 (test counts and test lists only)
**Coverage**: 68.95% (533/773 lines) — stale, not re-measured
