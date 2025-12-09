# gist-cache-rs Functional Verification Test Design Document

## Test Objective

To confirm that the gist-cache-rs 2-layer caching mechanism operates correctly as designed.

## Target Gist

- **Gist ID**: 7bcb324e9291fa350334df8efb7f0deb
- **Filename**: hello_args.sh
- **Description**: Bash argument test script
- **URL**: https://gist.github.com/7rikazhexde/7bcb324e9291fa350334df8efb7f0deb

## Prerequisites

- gist-cache-rs is installed
- GitHub CLI is authenticated
- Metadata cache is up-to-date (`gist-cache-rs update` has been executed)

## Test Case List

### TC1: First Execution (no content cache)

**Objective**: Confirm that on first execution, content is fetched from GitHub API and content cache is created.

**Prerequisites**:

- Metadata cache exists
- Content cache for hello_args.sh does not exist

**Steps**:

1. Delete content cache: `rm -rf ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`
2. Execute: `gist-cache-rs run hello_args.sh bash arg1 arg2 arg3`
3. Verify existence of cache file: `ls ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`

**Expected Result**:

- Message "Info: Cache not found, fetching from GitHub API..." is displayed.
- Script executes successfully.
- Arguments are displayed correctly (arg1, arg2, arg3).
- Content cache file is created (`~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/hello_args.sh`).

---

### TC2: Second Execution (with content cache)

**Objective**: Confirm that on second and subsequent executions, content is loaded quickly from cache.

**Prerequisites**:

- TC1 completed (content cache exists)

**Steps**:

1. Execute: `gist-cache-rs run hello_args.sh bash test1 test2`
2. Subjectively check execution time.

**Expected Result**:

- Message "Info: Cache not found..." is **not** displayed.
- Script executes instantly (no network wait).
- Arguments are displayed correctly (test1, test2).
- Execution is faster than TC1 due to loading from cache.

---

### TC3: Metadata Update via `update` Command (no change)

**Objective**: Confirm that content cache is retained if there are no changes to the Gist.

**Prerequisites**:

- TC2 completed (content cache exists)

**Steps**:

1. Execute update command: `gist-cache-rs update --verbose`
2. Verify existence of cache file: `ls -la ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`
3. Execute: `gist-cache-rs run hello_args.sh bash check`

**Expected Result**:

- "No updates" or "Updated: 0 items" is displayed by the update command.
- Content cache file is not deleted.
- Execution loads from cache (no API message).

---

### TC4: Behavior after Gist Update

**Objective**: Confirm that if a Gist is updated, content cache is deleted after update, and the latest version is fetched on next execution.

**Prerequisites**:

- TC3 completed (content cache exists)

**Steps**:

1. Edit hello_args.sh on GitHub (e.g., add a comment line).
2. Record timestamp of cache file: `stat ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/hello_args.sh`
3. Execute update command: `gist-cache-rs update --verbose`
4. Verify existence of cache file: `ls ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`
5. Execute: `gist-cache-rs run hello_args.sh bash updated`

**Expected Result**:

- "Updated: 1 item" is displayed by the update command.
- Content cache directory is deleted (`contents/7bcb324e9291fa350334df8efb7f0deb/` does not exist).
- "Info: Cache not found, fetching from GitHub API..." is displayed on execution.
- Latest version of the script is executed (edits are reflected).
- New content cache is created.

---

### TC5: `--force` Option Behavior

**Objective**: Confirm that `run --force` automatically executes `update` before running.

**Prerequisites**:

- TC4 completed (content cache exists)

**Steps**:

1. Edit hello_args.sh on GitHub again (e.g., add another comment).
2. **Without executing update command**, execute with `--force` option: `gist-cache-rs run --force hello_args.sh bash force_test`

**Expected Result**:

- Metadata cache is automatically updated before execution (internal process).
- Content cache is deleted because the Gist was updated.
- Latest version of the script is executed (second edit is reflected).
- New content cache is created.

---

### TC6: `cache list` Command

**Objective**: Confirm that the cache list is displayed correctly.

**Prerequisites**:

- TC5 completed (content cache exists)

**Steps**:

1. Execute: `gist-cache-rs cache list`

**Expected Result**:

- hello_args.sh (ID: 7bcb324e9291fa350334df8efb7f0deb) is displayed in the list.
- Description, filename, and updated_at are displayed.
- Total number of cached Gists is displayed.

---

### TC7: `cache size` Command

**Objective**: Confirm that the cache size is displayed correctly.

**Prerequisites**:

- TC6 completed

**Steps**:

1. Execute: `gist-cache-rs cache size`

**Expected Result**:

- Number of cached Gists is displayed.
- Total size is displayed (e.g., in KB).
- Path to the cache directory is displayed.

---

### TC8: `cache clear` Command

**Objective**: Confirm that all caches are deleted.

**Prerequisites**:

- TC7 completed (content cache exists)

**Steps**:

1. Execute: `gist-cache-rs cache clear`
2. Enter `y` at the confirmation prompt.
3. Verify cache directory: `ls ~/.cache/gist-cache/contents/`

**Expected Result**:

- Confirmation prompt is displayed.
- Message "All caches deleted" is displayed.
- `contents` directory becomes empty.
- Next execution behaves as a first execution.

---

## Test Execution Order

1. TC1: First execution (no cache)
2. TC2: Second execution (with cache)
3. TC3: update (no change)
4. TC4: Behavior after Gist update ← **Requires editing on GitHub**
5. TC5: --force option ← **Requires re-editing on GitHub**
6. TC6: cache list
7. TC7: cache size
8. TC8: cache clear

## Notes

- TC4 and TC5 require Gist editing on GitHub.
- Minor edits (e.g., adding a comment) are sufficient.
- Metadata cache should be up-to-date before running tests (`gist-cache-rs update`).
- State is carried over between test cases, so they must be executed in order.
