# gist-cache-rs Functional Verification Test Design Document (Test Set 4: Preview Function Verification)

## Test Objective

To confirm that the gist-cache-rs preview function (`-p`/`--preview`) works correctly.

## Target Functionality

- Preview display with Auto Search
- Preview display with Direct ID Specification
- Preview display with Filename Search
- Preview display with existing cache
- `--preview` option (long option) behavior

## Prerequisites

- gist-cache-rs is installed
- GitHub CLI is authenticated
- Metadata cache is up-to-date (`gist-cache-rs update` has been executed)
- Test Gists for hello_args series exist

## Test Case List

### TC1: Preview with Auto Search (-p option)

**Objective**: Confirm that preview is displayed correctly with keyword search.

**Prerequisites**:

- Searching for "hello" hits multiple Gists.

**Steps**:

1. Search with `-p` option: `gist-cache-rs run -p hello bash`
2. Select one from multiple candidates (e.g., number 7).
3. Check displayed content.

**Expected Result**:

- Multiple Gists are displayed in a numbered list.
- After selecting a number, the following are displayed:
  - Description
  - Files
  - `=== Gist Content ===` section
  - `--- Filename ---` header
  - File content (source code)
- Script is not executed (no version information or argument display).

**Verification Items**:

- Preview mode launches correctly.
- Gist content is displayed correctly.
- Exits without executing.

---

### TC2: Preview with Direct ID Specification

**Objective**: Confirm that preview is displayed correctly when specifying ID.

**Prerequisites**:

- hello_args.sh (ID: 7bcb324e9291fa350334df8efb7f0deb) is known.

**Steps**:

1. Preview with ID: `gist-cache-rs run -p --id 7bcb324e9291fa350334df8efb7f0deb bash`
2. Check displayed content.

**Expected Result**:

- "ID specification mode" message is displayed.
- Search process is skipped.
- Description, Files, Gist content are displayed.
- Script is not executed.

**Verification Items**:

- Combination of direct ID specification and preview mode works.
- Selection UI is skipped.
- Content is displayed correctly.

---

### TC3: Preview with Filename Specification

**Objective**: Confirm that preview is displayed correctly with filename search.

**Prerequisites**:

- hello_args.py exists.

**Steps**:

1. Preview with filename: `gist-cache-rs run -p --filename hello_args.py python3`
2. Check displayed content.

**Expected Result**:

- Gist containing hello_args.py is searched.
- If single result, direct preview display.
- If multiple results, selection UI is displayed.
- Gist content is displayed correctly.
- Script is not executed.

**Verification Items**:

- Combination of filename search and preview mode works.
- Content is displayed correctly.

---

### TC4: Preview with Cache

**Objective**: Confirm preview behavior when content cache exists.

**Prerequisites**:

- hello_args.sh is already cached (executed previously).

**Steps**:

1. Verify cache: `ls ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`
2. Execute preview: `gist-cache-rs run -p --id 7bcb324e9291fa350334df8efb7f0deb bash`
3. Check displayed content.

**Expected Result**:

- "Info: Cache not found..." message is not displayed.
- Content is loaded quickly from cache.
- Gist content is displayed correctly.
- Script is not executed.

**Verification Items**:

- Preview display from cache works.
- No network access occurs (if cache already exists).

---

### TC5: `--preview` Option (Long Option)

**Objective**: Confirm that the long option `--preview` works correctly.

**Prerequisites**:

- hello_args.rb exists.

**Steps**:

1. Search with `--preview` option: `gist-cache-rs run --preview --filename hello_args.rb ruby`
2. Check displayed content.

**Expected Result**:

- Behaves the same as `-p`.
- Gist content is displayed correctly.
- Script is not executed.

**Verification Items**:

- Long option `--preview` works.
- Consistent behavior with `-p`.

---

## Test Execution Order

1. TC1: Preview with Auto Search (Interactive Test)
2. TC2: Preview with Direct ID Specification
3. TC3: Preview with Filename Specification
4. TC4: Preview with Cache
5. TC5: --preview Option (Long Option)

## Notes

- TC1 requires interactive input.
- Scripts are not executed in preview mode.
- Messages vary depending on cache presence.
- TC4 creates cache after TC2 execution.
- Arguments (bash, python3, etc.) are not used in preview display (display only).