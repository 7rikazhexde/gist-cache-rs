# Configuration Guide

`gist-cache-rs` supports user configuration through both CLI commands and a configuration file. This allows you to customize default behavior and set preferences for script execution and cache management.

## Configuration File Location

The configuration file is stored at:

- **Linux/macOS**: `~/.config/gist-cache/config.toml`
- **Windows**: `%APPDATA%\gist-cache\config.toml`

The configuration file uses TOML format and is created automatically when you first use `gist-cache-rs config` commands.

## Available Configuration Options

### defaults.interpreter

Set the default interpreter for script execution. You can configure interpreters in two ways:

#### Simple Configuration (Legacy)

Set a single default interpreter for all scripts:

```bash
# Set default interpreter
gist-cache-rs config set defaults.interpreter python3

# Now you can omit the interpreter argument
gist-cache-rs run my-script  # Uses python3 by default
```

#### Advanced Configuration (Per-Extension)

**New in v0.8.6**: Configure different interpreters for different file types:

```bash
# Set interpreter for Python files
gist-cache-rs config set defaults.interpreter.py python3

# Set interpreter for Ruby files
gist-cache-rs config set defaults.interpreter.rb ruby

# Set interpreter for TypeScript files
gist-cache-rs config set defaults.interpreter.ts deno

# Set interpreter for JavaScript files
gist-cache-rs config set defaults.interpreter.js node

# Set wildcard fallback for all other file types
gist-cache-rs config set defaults.interpreter."*" bash
```

**Supported interpreters**: `bash`, `sh`, `zsh`, `python3`, `ruby`, `node`, `perl`, `php`, `pwsh`, `ts-node`, `deno`, `bun`, `uv`, etc.

#### Interpreter Resolution Priority

When executing a script, the interpreter is determined using the following priority order (highest to lowest):

1. **Command-line argument** - Explicitly specified interpreter
2. **Shebang detection** - From `#!/usr/bin/env python3` or `#!/usr/bin/python3`
3. **User configuration** - Extension-based settings (`defaults.interpreter.<ext>`)
4. **Heuristics** - Filename patterns (e.g., `Makefile` → `make`)
5. **Content analysis** - Language detection from file content (using tokei)
6. **Global defaults** - Wildcard (`defaults.interpreter."*"`) or `bash` fallback

**Example Configuration File:**

```toml
[defaults.interpreter]
py = "python3"
rb = "ruby"
ts = "deno"
js = "node"
sh = "bash"
Makefile = "make"
"*" = "bash"  # Wildcard fallback
```

**Example Usage:**

```bash
# Python script with .py extension - automatically uses python3
gist-cache-rs run data-analysis  # Detects .py, uses python3

# Override with command-line argument
gist-cache-rs run data-analysis bash  # Forces bash

# Script with shebang - uses detected interpreter
gist-cache-rs run setup-script  # Reads #!/usr/bin/env python3

# Unknown file type - uses wildcard fallback
gist-cache-rs run unknown-script  # Uses bash (from "*")
```

### execution.confirm_before_run

Enable or disable confirmation prompt before executing scripts (for safety).

**Values**: `true` or `false`

```bash
# Enable confirmation prompt
gist-cache-rs config set execution.confirm_before_run true

# Disable confirmation prompt
gist-cache-rs config set execution.confirm_before_run false
```

### cache.retention_days

Set the number of days to retain cached content before automatic cleanup.

**Values**: Any positive integer (days)

```bash
# Set retention period to 30 days
gist-cache-rs config set cache.retention_days 30
```

## Configuration Commands

### Interactive Configuration Setting

**New in v0.8.7**: Configure all settings interactively using a cursor-based menu:

```bash
gist-cache-rs config setting
```

This command provides an interactive interface to configure all settings with visual selection:

- **Interpreter selection**: Choose interpreters for each file extension using cursor keys
- **Execution settings**: Toggle `confirm_before_run` with Yes/No prompt
- **Cache settings**: Set retention days with validation (1-365)
- **Current values**: Shows current configuration values as defaults
- **Validation**: Prevents invalid values before saving

**Example flow:**

```
Interactive Configuration Setting

Configure interpreters for each file extension and other settings.

Select interpreter for .py
  Interpreter for .py
  > uv
    python3

✓ Set .py: uv

Select interpreter for .rb
  Interpreter for .rb
  > ruby

✓ Set .rb: ruby

...

Execution Settings
  Confirm before running scripts? (Y/n): n
✓ Set confirm_before_run: false

Cache Settings
  Cache retention days (1-365) [30]: 60
✓ Set retention_days: 60

✓ Configuration saved successfully!

Config file: /home/user/.config/gist-cache/config.toml
```

This is the recommended way to configure `gist-cache-rs`, especially for first-time setup or when you want to review all available options.

### Set a Configuration Value

```bash
gist-cache-rs config set <key> <value>
```

**Examples:**

```bash
# Set default interpreter to python3
gist-cache-rs config set defaults.interpreter python3

# Enable execution confirmation
gist-cache-rs config set execution.confirm_before_run true

# Set cache retention to 60 days
gist-cache-rs config set cache.retention_days 60
```

### Get a Configuration Value

```bash
gist-cache-rs config get <key>
```

**Examples:**

```bash
# Get current default interpreter
gist-cache-rs config get defaults.interpreter
# Output: python3

# Get confirmation setting
gist-cache-rs config get execution.confirm_before_run
# Output: true

# Get a value that's not set
gist-cache-rs config get cache.retention_days
# Output: Config key 'cache.retention_days' not set
```

### Show All Configuration

Display all current configuration settings:

```bash
gist-cache-rs config show
```

**Example output:**

```
Configuration:
  defaults.interpreter: python3
  execution.confirm_before_run: true
  cache.retention_days: 30
```

### Edit Configuration File

Open the configuration file in your default text editor:

```bash
gist-cache-rs config edit
```

This command uses the `$EDITOR` environment variable (Linux/macOS) or `notepad` (Windows) to open the config file. If `$EDITOR` is not set, it falls back to `vi` on Unix-like systems.

### Reset Configuration

Reset all configuration to default values:

```bash
gist-cache-rs config reset
```

This removes the configuration file and resets all settings to defaults.

## Configuration File Format

The configuration file uses TOML format. Here are examples:

### Simple Configuration

```toml
[defaults]
interpreter = "python3"

[execution]
confirm_before_run = true

[cache]
retention_days = 30
```

### Advanced Configuration (v0.8.6+)

```toml
[defaults.interpreter]
py = "python3"
rb = "ruby"
ts = "deno"
js = "node"
sh = "bash"
Makefile = "make"
"*" = "bash"  # Wildcard fallback for unknown file types

[execution]
confirm_before_run = false

[cache]
retention_days = 30
```

You can edit this file directly using `gist-cache-rs config edit` or any text editor.

## Practical Examples

### Example 1: Set Default Python Interpreter

If you primarily run Python scripts, set the default interpreter:

```bash
# Set default to python3
gist-cache-rs config set defaults.interpreter python3

# Now you can run Python scripts without specifying the interpreter
gist-cache-rs run data-analysis
# Instead of: gist-cache-rs run data-analysis python3
```

### Example 1b: Configure Per-Extension Interpreters (v0.8.6+)

For mixed-language projects, configure different interpreters for different file types:

#### Method 1: Interactive (Recommended)

```bash
# Use interactive configuration (v0.8.7+)
gist-cache-rs config setting

# Follow the prompts to select interpreters for each extension
# - Use arrow keys to navigate
# - Press Enter to confirm selection
# - All settings are validated before saving
```

#### Method 2: Manual CLI commands

```bash
# Configure interpreters for different languages
gist-cache-rs config set defaults.interpreter.py python3
gist-cache-rs config set defaults.interpreter.rb ruby
gist-cache-rs config set defaults.interpreter.ts deno
gist-cache-rs config set defaults.interpreter.js node
gist-cache-rs config set defaults.interpreter."*" bash

# View your configuration
gist-cache-rs config show
```

**Result:**

```bash
# Now scripts automatically use the right interpreter
gist-cache-rs run data-script      # .py file → uses python3
gist-cache-rs run deploy-script    # .rb file → uses ruby
gist-cache-rs run build-script     # .ts file → uses deno
gist-cache-rs run test-runner      # .js file → uses node
gist-cache-rs run backup-script    # .sh file → uses bash (wildcard)
```

### Example 2: Enable Safety Confirmation

For added security when running scripts:

```bash
# Enable confirmation before execution
gist-cache-rs config set execution.confirm_before_run true

# Now you'll be prompted before executing any script
gist-cache-rs run backup
# Prompt: Execute backup.sh with bash? [y/N]:
```

### Example 3: Manage Cache Retention

Set how long to keep cached content:

```bash
# Keep cache for 60 days
gist-cache-rs config set cache.retention_days 60

# Clean old cache entries older than retention period
gist-cache-rs cache clean --older-than 60
```

### Example 4: View Current Configuration

Check your current settings:

```bash
# Show all configuration
gist-cache-rs config show

# Or check specific settings
gist-cache-rs config get defaults.interpreter
gist-cache-rs config get execution.confirm_before_run
```

## Environment Variable Override

For testing or temporary configurations, you can override the config directory using the `GIST_CACHE_DIR` environment variable:

```bash
# Use a custom directory for both config and cache
GIST_CACHE_DIR=/tmp/test-cache gist-cache-rs config show
```

This is particularly useful for:

- Testing different configurations
- Isolating test environments
- Running multiple configurations simultaneously

## Tips

1. **Use interactive setup first**: Run `config setting` for first-time configuration (v0.8.7+)
2. **Start with safe defaults**: Enable `confirm_before_run` when you're new to the tool
3. **Use per-extension configuration**: Set up interpreters for each file type you commonly use
4. **Set a wildcard fallback**: Configure `defaults.interpreter."*"` to handle unknown file types
5. **Leverage shebang detection**: Scripts with shebangs automatically use the correct interpreter
6. **Regular cleanup**: Set an appropriate `retention_days` value to keep your cache clean
7. **Check before reset**: Use `config show` before `config reset` to review your settings
8. **Interactive vs. Manual**: Use `config setting` for guided setup, `config set` for quick changes
9. **Edit directly for complex changes**: Use `config edit` to modify multiple settings at once
10. **Override when needed**: Command-line arguments always take precedence over configuration

## Related Commands

- `gist-cache-rs config setting` - Interactive configuration (v0.8.7+)
- `gist-cache-rs config --help` - Show config command help
- `gist-cache-rs cache clean --help` - Cache cleanup options
- `gist-cache-rs run --help` - Script execution options
