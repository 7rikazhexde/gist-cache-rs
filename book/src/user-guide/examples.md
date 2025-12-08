# Examples Collection

This document presents practical examples of `gist-cache-rs` usage.

## Basic Usage

### Cache Update

```bash
# First time or full update
$ gist-cache-rs update --verbose
Updating Gist cache...
Mode: Differential update
Rate limit remaining: 4966
Existing cache detected
GitHub user (cache reuse): your-username
Last updated: 2025-10-26T02:22:04Z
Fetching Gist information from GitHub API...
Gists fetched: 1
Differential merge complete: Existing 124 + Diff 1 → Total 124
Updated: 1 item
Cache update completed
Total Gists: 124

# If no updates
$ gist-cache-rs update --verbose
Updating Gist cache...
Mode: Differential update
Rate limit remaining: 4964
Existing cache detected
GitHub user (cache reuse): your-username
Last updated: 2025-10-26T02:35:44Z
Fetching Gist information from GitHub API...
Gists fetched: 0
No updates
Cache update completed
Total Gists: 124
```

---

## Bash Script Examples

### Example 1: Sequential Folder Creation Script

**Gist Description:** A script to create 100 folders with sequential numbers (start number to end number) in a specified path.

#### Preview content in preview mode

```bash
$ gist-cache-rs run -p create_folder
Description: A script to create 100 folders with sequential numbers (start number to end number) in a specified path. #bash
Files: create_folders.sh

=== Gist Content ===
--- create_folders.sh ---
#!/bin/bash
# A script to create 100 folders with sequential numbers (start number to end number) in a specified path.

show_usage() {
  echo "Usage: $0 [prefix] [destination] [start_number] [end_number]"
  echo ""
  echo "If arguments are omitted, you can enter them interactively"
  # ... (omitted)
}
# ... (script body)
```

#### Select from multiple candidates with partial matching search

```bash
$ gist-cache-rs run -p create
Multiple Gists found:

 1. A script to create 100 folders with sequential numbers (start number to end number) in a specified path. #bash | create_folders.sh
 2. Create GitHub Gist with CLI | create_gist.sh
 3. Create multiple directories | create_dirs.sh
 4. Create backup archive | create_backup.sh
 5. Create project template | create_template.sh
 6. Create Docker container | create_container.sh
 7. Create test data | create_testdata.py

Select a number (1-7): 1

Description: A script to create 100 folders with sequential numbers (start number to end number) in a specified path. #bash
Files: create_folders.sh
# ... (content displayed)
```

#### Execute in interactive mode

```bash
$ gist-cache-rs run -i create_folder
Description: A script to create 100 folders with sequential numbers (start number to end number) in a specified path. #bash
Files: create_folders.sh
Executing: create_folders.sh (bash)

Usage: /tmp/create_folders.sh [prefix] [destination] [start_number] [end_number]

If arguments are omitted, you can enter them interactively

Example: /tmp/create_folders.sh aaa /path/to/directory 1000 1500

------------------------------------------------------
 ~$ /tmp/create_folders.sh aaa bbb 0 200
 Creating folder: ./bbb/aaa_No.0-99 (range: 0-99)
 Creating folder: ./bbb/aaa_No.100-200 (range: 100-200)
------------------------------------------------------

Run in interactive mode? (y/N): y

=== Interactive Mode ===
Enter prefix: test1
Enter destination directory: ./test
Enter start number: 0
Enter end number: 1000

 Creating folder: ./test/test1_No.0-99 (range: 0-99)
 Creating folder: ./test/test1_No.100-199 (range: 100-199)
 Creating folder: ./test/test1_No.200-299 (range: 200-299)
 Creating folder: ./test/test1_No.300-399 (range: 300-399)
 Creating folder: ./test/test1_No.400-499 (range: 400-499)
 Creating folder: ./test/test1_No.500-599 (range: 500-599)
 Creating folder: ./test/test1_No.600-699 (range: 600-699)
 Creating folder: ./test/test1_No.700-799 (range: 700-799)
 Creating folder: ./test/test1_No.800-899 (range: 800-899)
 Creating folder: ./test/test1_No.900-999 (range: 900-999)
 Creating folder: ./test/test1_No.1000-1000 (range: 1000-1000)
Processing completed.
```

**Key Points:**

- 📝 Enable interactive mode with the `-i` option.
- 💬 The `read` command within the script works correctly.
- ✅ The script executes while accepting user input.

---

## Python Script Examples

### Example 2: Pandas/NumPy Data Analysis (PEP 723 Compatible)

**Gist Description:** data_analysis.py - Pandas/NumPy usage example #python #pandas #numpy #uv #pep723 #csv

#### Search by tag (Preview)

```bash
$ gist-cache-rs run -p '#pep723'
Multiple Gists found:

 1. data_analysis.py - Pandas/NumPy usage example #python #pandas #numpy #uv #pep723 #csv | data_analysis.py
 2. uv_test.py - UV temporary installation test #python #pandas #numpy #uv #pep723 | uv_test.py

Select a number (1-2): 1

Description: data_analysis.py - Pandas/NumPy usage example #python #pandas #numpy #uv #pep723 #csv
Files: data_analysis.py

=== Gist Content ===
--- data_analysis.py ---
#!/usr/bin/env python3
# /// script
# dependencies = ["pandas", "numpy"]
# ///

import pandas as pd
import numpy as np
import sys
import os

def main() -> None:
    print(f"Pandas version: {pd.__version__}")
    print(f"NumPy version: {np.__version__}")

    if len(sys.argv) < 2:
        print("Error: Please specify the path to the CSV file (e.g., input.csv)")
        sys.exit(1)

    csv_file = sys.argv[1]

    if not os.path.exists(csv_file):
        print(f"Error: File '{csv_file}' not found.")
        sys.exit(1)

    # Read CSV file
    try:
        df = pd.read_csv(csv_file)
        print(f"\nCSV file '{csv_file}' read (rows: {len(df)})")
        print("\nDataFrame (first 5 rows):")
        print(df.head())

        # Simple data analysis
        print(f"\nNumber of columns: {len(df.columns)}")
        print(f"\nMean:\n{df.mean(numeric_only=True)}")

        # Generate random data as an example (optional)
        if len(df) > 0:
            print(f"\nAdded random column 'Random':")
            df["Random"] = np.random.randint(1, 100, len(df))
            print(df[["Random"]].head())

    except Exception as e:
        print(f"Error: An exception occurred during CSV processing: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
```

#### Execute with uv (automatic dependency management)

```bash
$ gist-cache-rs run 723 uv sample/input.csv
Multiple Gists found:

 1. data_analysis.py - Pandas/NumPy usage example #python #pandas #numpy #uv #pep723 #csv | data_analysis.py
 2. uv_test.py - UV temporary installation test #python #pandas #numpy #uv #pep723 | uv_test.py

Select a number (1-2): 1

Description: data_analysis.py - Pandas/NumPy usage example #python #pandas #numpy #uv #pep723 #csv
Files: data_analysis.py
Executing: data_analysis.py (python3)

Pandas version: 2.3.3
NumPy version: 2.3.4

CSV file 'sample/input.csv' read (rows: 5)

DataFrame (first 5 rows):
    A   B
0  77  28
1   5  65
2  47  34
3  84  82
4  65  46

Number of columns: 2

Mean:
A    55.6
B    51.0
dtype: float64

Added random column 'Random':
   Random
0      67
1      70
2       7
3      74
4      60
```

**Key Points:**

- 📦 Dependencies defined by PEP 723 metadata (`# /// script`).
- ⚡ `uv` automatically installs pandas and numpy.
- 🔧 The argument `sample/input.csv` is passed to the script.
- 🎯 Executes temporarily without polluting the global environment.

---

## Search Techniques

### Tips for Keyword Search

#### 1. Partial Match Search

```bash
# Search for all Gists containing "create"
$ gist-cache-rs run create

# Search for all Gists containing "data"
$ gist-cache-rs run data
```

#### 2. Tag Search

```bash
# Filter by hashtag
$ gist-cache-rs run '#bash'
$ gist-cache-rs run '#python'
$ gist-cache-rs run '#pep723'
```

#### 3. Filename Search

```bash
# Search directly by filename
$ gist-cache-rs run --filename data_analysis.py
$ gist-cache-rs run --filename create_folders.sh
```

#### 4. Description Search

```bash
# Search only by description
$ gist-cache-rs run --description "Data Analysis"
$ gist-cache-rs run --description "Numpy"
```

#### 5. Direct ID Specification

```bash
# Execute directly using Gist ID
$ gist-cache-rs run --id [your_gist_id] uv input.csv
```

---

## Cache Management Examples

### Check Cache List

```bash
$ gist-cache-rs cache list
List of Cached Gists:

ID: 7bcb324e9291fa350334df8efb7f0deb
  Description: hello_args.sh - Argument display script #bash #test
  File: hello_args.sh
  Updated: 2025-10-26 12:30:45

ID: e3a6336c9f3476342626551372f14d6e
  Description: data_analysis.py - Pandas/NumPy usage example #python #pep723
  File: data_analysis.py
  Updated: 2025-10-25 18:22:10

Total: 2 Gists cached
```

### Check Cache Size

```bash
$ gist-cache-rs cache size
Cache Size Information:

Number of cached Gists: 15
Total size: 89.45 KB
Cache directory: /home/user/.cache/gist-cache/contents
```

### Clear All Caches

```bash
$ gist-cache-rs cache clear
Clear All Caches

Are you sure you want to delete 15 Gist caches?
  This operation is irreversible.

Proceed? (y/N): y

All caches deleted
```

---

## `--force` Option Usage Examples

### Always execute development Gists with the latest version

```bash
# Use in the cycle of editing and executing development scripts
$ gist-cache-rs run --force test-script bash arg1 arg2

# Internally performs the following actions:
# 1. Incrementally updates the metadata cache.
# 2. If the Gist has been updated, deletes the content cache.
# 3. Fetches and executes the latest version.
# 4. Creates a new cache.
```

### Combine with search options

```bash
# Search by description and always execute the latest version
$ gist-cache-rs run --force --description "backup script" bash /src /dst

# Search by filename and execute the latest version
$ gist-cache-rs run --force --filename deploy.sh bash
```

**Key Points:**

- 📡 Automatically runs `update` (incremental update) before execution.
- ⚡ If the Gist is not updated, it executes quickly using the existing cache.
- 🔄 Only fetches a new version if it has been updated.

---

## File Download

### Basic Download

```bash
# Save to download folder after execution
$ gist-cache-rs run --download data_analysis uv input.csv

Description: data_analysis.py - Pandas/NumPy usage example #python #pandas #numpy #uv #pep723 #csv
Files: data_analysis.py
Executing: data_analysis.py (python3)

Pandas version: 2.3.3
NumPy version: 2.3.4
# ... (execution results)

=== Downloading File ===
  ✓ Download complete: /home/user/Downloads/data_analysis.py

1 file saved to /home/user/Downloads
```

### Download After Preview

```bash
# Download after confirming content
$ gist-cache-rs run -p --download backup

Description: Backup script #bash #backup
Files: backup.sh

=== Gist Content ===
--- backup.sh ---
#!/bin/bash
# Backup script
# ... (content displayed)

=== Downloading File ===
  ✓ Download complete: /home/user/Downloads/backup.sh

1 file saved to /home/user/Downloads
```

### Force Update and Download

```bash
# Get the latest version, then execute and download
$ gist-cache-rs run -f --download setup bash

Updating Gist cache...
No updates
Cache update completed

Description: Setup script #bash #setup
Files: setup.sh
Executing: setup.sh (bash)
# ... (execution results)

=== Downloading File ===
  ✓ Download complete: /home/user/Downloads/setup.sh

1 file saved to /home/user/Downloads
```

### Download by ID

```bash
# Directly specify Gist ID to download
$ gist-cache-rs run --download --id abc123def456

ID specified mode: abc123def456

Description: Useful script #bash
Files: utility.sh
Executing: utility.sh (bash)
# ... (execution results)

=== Downloading File ===
  ✓ Download complete: /home/user/Downloads/utility.sh

1 file saved to /home/user/Downloads
```

**Key Points:**

- 📥 Saves to the download folder (`~/Downloads`).
- 🔄 Cache is automatically created during download, speeding up subsequent executions.
- 🎯 Can be used with other options (`--preview`, `--force`, `--interactive`, etc.).
- 💾 Useful for saving files separately from executable script caches.

**Operation Order:**

1. `--preview --download`: Preview display → Download
2. `--force --download`: Cache update → Execute → Download
3. `--download` only: Execute → Download

---

## Tips & Tricks

### 1. Quickly Execute Recently Updated Gists

```bash
# Since the cache is sorted by update time in descending order,
# the first one found by partial match is the latest.
$ gist-cache-rs run keyword
```

### 2. Gists with Multiple Files

```bash
# If there are multiple files, the first file will be executed.
$ gist-cache-rs run multi-file-gist
```

### 3. Debug Mode

```bash
# Display debug information in verbose mode
$ gist-cache-rs update --verbose

# Preview content before execution
$ gist-cache-rs run -p script-name
```

### 4. Combine with Aliases

```bash
# Alias frequently used scripts
alias analyze='gcrsr data_analysis uv'

# Usage example
analyze mydata.csv
```

---

## Troubleshooting

### Q: Script not found

```bash
# Update the cache
$ gist-cache-rs update

# Check details in verbose mode
$ gist-cache-rs update --verbose
```

### Q: Interactive mode does not work

```bash
# Use the -i option
$ gist-cache-rs run -i script-name

# For bash, it may work without -i
$ gist-cache-rs run script-name bash
```

### Q: Error with uv

```bash
# Check if uv is installed
$ which uv

# Try executing with python3
$ gist-cache-rs run script-name python3
```

---

## Related Documentation

- [README.md](../README.md) - Project overview and basic features
- [INSTALL.md](INSTALL.md) - Installation guide
- [QUICKSTART.md](QUICKSTART.md) - 5-minute guide to get started
