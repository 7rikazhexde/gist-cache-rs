# Feature Update Plan: Configuration and Run Commands

## 1. Objective

To enhance the `config` and `run` commands to provide more flexible and intuitive interpreter management. This involves allowing per-extension default interpreters and improving automatic interpreter detection based on file content.

## 2. Background

* The current `config` command only supports a single, global default interpreter via `defaults.interpreter`.
* The `run` command's argument parsing can be ambiguous when the interpreter is omitted, and its fallback logic is not sophisticated enough to handle files without extensions.

## 3. Proposed Changes

### 3.1. Configuration File Format Enhancement

The configuration structure in `config.toml` will be enhanced to support per-extension interpreter settings. We will modify the `[defaults]` table to make `interpreter` a sub-table.

**Current Format:**

```toml
[defaults]
interpreter = "uv"
```

**Proposed Format:**

```toml
[defaults]
  # This field will be deprecated in the future but kept for backward compatibility for now.
  # interpreter = "uv"

  [defaults.interpreter]
  py = "python3"
  rb = "ruby"
  sh = "bash"
  ts = "deno"
  # A special key for a global fallback when no other rule matches.
  "*" = "bash"
```

This structure allows users to set defaults like `gist-cache-rs config set defaults.interpreter.py python3`.

### 3.2. Interpreter Resolution Logic

The logic for determining the interpreter will be updated to follow a clear priority order:

1. **Command-Line Argument (Highest Priority):** If an interpreter is specified directly in the `run` command (e.g., `... run my_gist ruby`), it will always be used. This maintains existing behavior for maximum user control.

2. **Shebang Detection (File-specific):** Check the first line of the script for a shebang (e.g., `#!/usr/bin/env python3`). If found, this is honored next, as it's an explicit instruction from the script's author.

3. **User Configuration (Environment-specific):** If no shebang is found, the user's personal configuration is checked.

    * The file's extension (e.g., `py`) and full filename (e.g., `Makefile`) are checked against the `[defaults.interpreter]` table. If a match is found, that interpreter is used.

4. **Heuristics and Analysis (Inference-based):** If the configuration yields no match, the system will infer the interpreter.

    a. **Filename Heuristics:** Before analyzing the full file content, check for common, un-configured filenames (`Makefile`) or extensions (`.py`). This provides a quick, conventional lookup.

    b. **Language Detection via Content (`tokei`):** If the filename is not informative, analyze the file's content to guess the language using the `tokei` crate. Map the detected language to a default interpreter.

5. **Global Defaults (Lowest Priority):**

    a. **Global Config Default:** Look for a wildcard setting `defaults.interpreter."*"` in the configuration file.

    b. **Final Fallback:** If all other methods fail, default to a standard shell (`bash`).

### 3.3. `config set` Command Enhancement

The `config set` command logic will be updated to correctly handle dot-separated keys for nested tables, allowing commands like `gist-cache-rs config set defaults.interpreter.py python3` to work as expected.

## 4. Implementation Steps

1. **Switch to `main` branch, pull latest changes, and create a new feature branch.**
2. **Modify `config.rs`:** Update the `UserConfig` and `Defaults` structs to reflect the new nested `interpreter` table structure.
3. **Enhance `config set` logic:** Implement the logic to parse dot-notated keys and update the nested TOML structure.
4. **Add `tokei` dependency:** Add the `tokei` crate to `Cargo.toml`.
5. **Refactor Interpreter Logic:** Overhaul the interpreter selection logic (currently in `cli.rs`) to implement the new priority-based resolution system, integrating the calls to check the config and `tokei` for language detection.
6. **Update Tests:** Modify existing tests for `config` and `run` to align with the new specifications, and add new tests to cover per-extension defaults and content-based detection.
