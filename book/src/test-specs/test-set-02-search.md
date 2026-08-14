# gist-cache-rs Functional Verification Test Design Document (Test Set 2: Search Functionality)

## Test Objective

To confirm that the gist-cache-rs search functionality operates correctly as designed.

## Target Functionality

- Auto Search (default)
- Direct ID Specification Search (--id)
- Filename Search (--filename)
- Description Search (--description)
- Selection UI from multiple candidates
- Error handling for 0 search results

## Prerequisites

- gist-cache-rs is installed
- GitHub CLI is authenticated
- Metadata cache is up-to-date (`gist-cache-rs update` has been executed)
- Multiple test Gists exist

## Test Case List

### TC1: Auto Search (default)

**Objective**: Confirm that automatic search by keyword works correctly.

**Prerequisites**:

- Multiple Gists including hello_args.sh exist

**Steps**:

1. Search by keyword "hello": `gist-cache-rs run hello bash`
2. Check search results.
3. If multiple candidates are displayed, highlight one with `↑`/`↓` and press `Enter` to execute.

**Expected Result**:

- Gists containing "hello" in filename or description are displayed.
- If 1 item, executed directly; if multiple, selection UI is displayed.
- Selected Gist is executed correctly.

**Verification Items**:

- Partial match search by filename works.
- Partial match search by description works.
- Case-insensitive search works.

---

### TC2: Direct ID Specification (--id)

**Objective**: Confirm that search by directly specifying Gist ID works correctly.

**Prerequisites**:

- Gist ID for hello_args.sh (7bcb324e9291fa350334df8efb7f0deb) is known.

**Steps**:

1. Directly specify Gist ID: `gist-cache-rs run --id 7bcb324e9291fa350334df8efb7f0deb bash test`
2. Check execution result.

**Expected Result**:

- Specified Gist is executed directly, skipping the search process.
- "ID specification mode" message is displayed.
- Multiple candidate selection UI is not displayed.

**Verification Items**:

- Direct access with accurate Gist ID is possible.
- "ID specification mode" message is displayed.
- Cannot be used in conjunction with other search options (error or ignored).

---

### TC3: Filename Search (--filename)

**Objective**: Confirm that search limited to filename works correctly.

**Prerequisites**:

- Multiple Gists including hello_args.sh exist.

**Steps**:

1. Search by filename: `gist-cache-rs run --filename hello_args.sh bash`
2. Check search results.
3. Check execution result.

**Expected Result**:

- Only Gists with filename matching "hello_args.sh" are searched.
- Gists with "hello_args.sh" in description but different filename are excluded.
- Partial match search (e.g., "hello" finds "hello_args.sh") works.

**Verification Items**:

- Filename search works correctly.
- Description is not searched.
- If multiple matches, selection UI is displayed.

---

### TC4: Description Search (--description)

**Objective**: Confirm that search limited to description works correctly.

**Prerequisites**:

- Gists containing specific keywords in description (e.g., "#bash", "#test") exist.

**Steps**:

1. Search by description: `gist-cache-rs run --description "#bash" bash`
2. Check search results.
3. Select one from multiple candidates and execute.

**Expected Result**:

- Only Gists containing "#bash" in description are searched.
- If "#bash" is in filename but not description, Gist is excluded.
- Partial match search works.

**Verification Items**:

- Description search works correctly.
- Filename is not searched.
- Tag search (e.g., "#bash") works effectively.

---

### TC5: Selection from Multiple Candidates

**Objective**: Confirm that the interactive picker for selecting from multiple search results works correctly.

**Prerequisites**:

- Searching for "hello" hits multiple Gists.

**Steps**:

1. Search with a keyword that hits multiple results: `gist-cache-rs run hello bash`
2. Check the displayed candidate list (description/filename, deduplicated and truncated to fit the terminal).
3. Move the highlight with `↑`/`↓` (e.g., down to the 4th item) and press `Enter`.
4. Confirm that the selected Gist is executed.

**Expected Result**:

- Multiple Gists are displayed in a scrollable, arrow-key-navigable list running in the terminal's alternate screen buffer.
- Description and filename of each Gist are displayed (long entries truncated by default; `Tab` toggles full display).
- A footer shows the current position (`[n/total]`) and full/short state.
- Gist corresponding to the highlighted item is executed after `Enter`.

**Verification Items**:

- List is displayed correctly and scrolls to keep the selection visible when there are more results than fit on screen.
- `↑`/`↓` moves the highlight; `Enter` executes the highlighted Gist.
- `Tab` toggles full/short item display.
- `Space` opens a syntax-highlighted content preview of the highlighted Gist without leaving the list.
- `/` filters the list (regex supported, falls back to literal substring matching); `Esc` clears the filter or cancels the picker.

---

### TC6: Error Handling for 0 Search Results

**Objective**: Confirm that error handling when no search results are found works correctly.

**Prerequisites**:

- Use a non-existent keyword (e.g., "nonexistent_gist_xyz").

**Steps**:

1. Search with a non-existent keyword: `gist-cache-rs run nonexistent_gist_xyz bash`
2. Check error message.
3. Check exit code.

**Expected Result**:

- Error message like "No search results for query: nonexistent_gist_xyz" is displayed.
- Program exits with an appropriate error code (non-zero).
- Program does not crash.

**Verification Items**:

- Clear error message is displayed.
- Exit code is non-zero.
- No memory leaks or crashes occur.

---

## Additional Test Cases

### TC7: Search with Special Characters

**Objective**: Confirm that search with keywords containing special characters works correctly.

**Steps**:

1. Search with a keyword containing special characters: `gist-cache-rs run "data_analysis.py" bash`

**Expected Result**:

- Filenames containing underscores or periods are searched correctly.
- Special characters are searched without escaping.

---

### TC8: Search with Spaces

**Objective**: Confirm that search for descriptions containing spaces works correctly.

**Steps**:

1. Search with a keyword containing spaces: `gist-cache-rs run --description "Test Script" bash`

**Expected Result**:

- Descriptions containing spaces are searched correctly.
- Keywords enclosed in quotes are processed correctly.

---

## Test Execution Order

1. TC1: Auto Search (default)
2. TC2: Direct ID Specification (--id)
3. TC3: Filename Search (--filename)
4. TC4: Description Search (--description)
5. TC5: Selection from Multiple Candidates
6. TC6: Error Handling for 0 Search Results
7. TC7: Search with Special Characters (optional)
8. TC8: Search with Spaces (optional)

## Notes

- TC5 requires interactive input, making automation difficult.
- TC6 is an error case, so it should be executed after normal tests.
- Search results vary depending on the environment (owned Gists).
- Metadata cache should be up-to-date before running tests.
