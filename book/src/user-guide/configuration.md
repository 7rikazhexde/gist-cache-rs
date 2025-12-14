# Configuration Guide

`gist-cache-rs` supports user configuration through both CLI commands and a configuration file. This allows you to customize default behavior and set preferences for script execution and cache management.

## Configuration File Location

The configuration file is stored at:

- **Linux/macOS**: `~/.config/gist-cache/config.toml`
- **Windows**: `%APPDATA%\gist-cache\config.toml`

The configuration file uses TOML format and is created automatically when you first use `gist-cache-rs config` commands.

## Available Configuration Options

### defaults.interpreter

Set the default interpreter for script execution.

**Example values**: `bash`, `python3`, `ruby`, `node`, `uv`, etc.

```bash
# Set default interpreter
gist-cache-rs config set defaults.interpreter python3

# Now you can omit the interpreter argument
gist-cache-rs run my-script  # Uses python3 by default
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

The configuration file uses TOML format. Here's an example:

```toml
[defaults]
interpreter = "python3"

[execution]
confirm_before_run = true

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

1. **Start with safe defaults**: Enable `confirm_before_run` when you're new to the tool
2. **Set your preferred interpreter**: Save time by configuring your most-used interpreter
3. **Regular cleanup**: Set an appropriate `retention_days` value to keep your cache clean
4. **Check before reset**: Use `config show` before `config reset` to review your settings
5. **Edit directly for complex changes**: Use `config edit` to modify multiple settings at once

## Related Commands

- `gist-cache-rs config --help` - Show config command help
- `gist-cache-rs cache clean --help` - Cache cleanup options
- `gist-cache-rs run --help` - Script execution options
