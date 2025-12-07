# Self-Update Feature Design

## Overview

The `gist-cache-rs self update` command will be added to allow users to easily update the application itself.

## Objective

- Update only the binary without re-running the setup script.
- Do not change environment settings such as alias settings or PATH settings.
- Cross-platform compatibility (Linux, macOS, Windows).

## Design Policy

### Differences from the setup script

| Feature                | Setup Script | `self update` command |
|------------------------|--------------|-----------------------|
| Prerequisite check     | ✓            | -                     |
| Build & Install        | ✓            | ✓                     |
| PATH setting           | ✓ (optional) | -                     |
| Alias setting          | ✓ (optional) | -                     |
| Initial cache creation | ✓ (optional) | -                     |

The `self update` command **focuses solely on "binary updates"**. This ensures:

- No re-execution of alias settings.
- No changes to PATH settings.
- No cache clearing.
- Preservation of existing environment settings.

## Implementation Policy

### Approach 1: Download binary from GitHub Releases (Recommended)

**Advantages**:

- Fast (no build required).
- Obtains the latest version via network.
- Not dependent on the user's build environment.

**Disadvantages**:

- Requires uploading platform-specific binaries to GitHub Releases.
- Additional work for the release process.

**Implementation Method**:

- Use the [`self_update`](https://crates.io/crates/self_update) crate.
- Download the corresponding platform binary from GitHub Releases.
- Replace the currently running binary.

**Security Considerations**:

- HTTPS communication (TLS verification).
- Release tag verification.
- (Future) Binary signature verification.

### Approach 2: Build from source (Fallback)

**Advantages**:

- Can get the latest `main`/`master` branch.
- Can update even if no release exists.
- Utilizes existing tools (git, cargo).

**Disadvantages**:

- Building takes time.
- Requires Rust toolchain.
- Consumes disk space.

**Implementation Method**:

1. Clone or pull the repository.
2. Build & install with `cargo install --path .`.

### Recommended Implementation: Hybrid Approach

1. **Default**: Download from GitHub Releases.
2. **Fallback**: Build from source with the `--from-source` flag.
3. **Option**: Check only for updates with `--check`.

## Command Design

### Basic Commands

```bash
# Update to the latest version (from GitHub Releases)
gist-cache-rs self update

# Build from source and update
gist-cache-rs self update --from-source

# Check only for updates (do not actually update)
gist-cache-rs self update --check

# Force update (even if the version is the same)
gist-cache-rs self update --force

# Update to a specific version
gist-cache-rs self update --version 0.5.0
```

### Options

| Option          | Description                                         |
|-----------------|-----------------------------------------------------|
| `--from-source` | Build and update from source instead of GitHub Releases |
| `--check`       | Check only for updates (do not actually update)     |
| `--force`       | Force update even if the version is the same        |
| `--version <VERSION>` | Update to a specific version                        |
| `--verbose`     | Display detailed progress information               |

## Processing Flow

### Update from GitHub Releases

```text
1. Get the current version.
2. Check for the latest release via GitHub API.
3. Check if a new version exists.
   - If not, exit.
4. Search for assets corresponding to the platform.
   - Linux: gist-cache-rs-linux-x86_64.tar.gz
   - macOS: gist-cache-rs-macos-x86_64.tar.gz / gist-cache-rs-macos-aarch64.tar.gz
   - Windows: gist-cache-rs-windows-x86_64.zip
5. Download the asset.
6. Extract the archive.
7. Replace the current binary with the new binary.
8. Set permissions (Unix-like systems only).
9. Display completion message.
```

### Update from Source

```text
1. Verify the existence of git and cargo commands.
2. Determine repository location.
   a. Overridable by environment variable GIST_CACHE_REPO.
   b. Default: Obtain from cargo metadata.
   c. Fallback: Display an error prompting git clone.
3. Get the latest version with `git pull --ff-only`.
   - Note: Depending on the state of the local repository, a fast-forward merge may not be possible, resulting in an error. In such cases, clean the repository (e.g., delete the directory specified by `GIST_CACHE_REPO` and re-clone) or use the update from GitHub Releases (without `--from-source`).
4. Build with `cargo build --release`.
5. Replace the current executable with the built binary (self-replace).
6. Display completion message.
```

## Platform Support

### Linux

- Binary name: `gist-cache-rs`
- Installation location: `~/.cargo/bin/gist-cache-rs`
- Asset name: `gist-cache-rs-linux-x86_64.tar.gz`

### macOS

- Binary name: `gist-cache-rs`
- Installation location: `~/.cargo/bin/gist-cache-rs`
- Asset name:
  - Intel: `gist-cache-rs-macos-x86_64.tar.gz`
  - Apple Silicon: `gist-cache-rs-macos-aarch64.tar.gz`

### Windows

- Binary name: `gist-cache-rs.exe`
- Installation location: `%USERPROFILE%\.cargo\bin\gist-cache-rs.exe`
- Asset name: `gist-cache-rs-windows-x86_64.zip`

## Dependencies

### Newly Added

```toml
[dependencies]
self_update = "0.41"  # Automatic updates from GitHub Releases
```

### Existing Dependencies

- `anyhow` / `thiserror`: Error handling
- `tokio`: Asynchronous processing
- `clap`: CLI argument parsing

## Error Handling

### Expected Errors

1. **Network Errors**
   - Cannot access GitHub API.
   - Cannot download assets.
   - Action: Retry or display error message.

2. **Permission Errors**
   - Cannot replace binary.
   - Action: Display message indicating sudo privileges are required.

3. **Unsupported Platform**
   - No corresponding asset.
   - Action: Suggest using `--from-source`.

4. **Version Acquisition Error**
   - Cannot get current version.
   - Action: Suggest using `--force`.

5. **Build Errors** (when using `--from-source`)
   - Rust toolchain not present.
   - Build failed.
   - Action: Suggest running the setup script.

## Security Considerations

### Current

1. **HTTPS Communication**: Performs TLS/SSL verification.
2. **GitHub API Authentication**: Considers GitHub API rate limits.
3. **Replacing Running Binary**: Uses platform-specific safe methods.

### Future Considerations

1. **Binary Signature Verification**: GPG signature or code signing.
2. **Checksum Verification**: SHA256 hash comparison.
3. **Rollback Functionality**: Revert to the previous version if update fails.

## Testing Policy

### Unit Tests

- Version comparison logic.
- Platform detection.
- Error handling.

### Integration Tests

- Download tests using mock GitHub API.
- File replacement tests (performed in temporary directories).

### E2E Tests (Manual)

- Actual updates from GitHub Releases.
- Platform-specific behavior verification.

## Changes to Release Process

### Automated Build & Release with GitHub Actions

Automate the following upon a new release:

1. Build platform-specific binaries.
   - Linux (x86_64)
   - macOS (x86_64, aarch64)
   - Windows (x86_64)

2. Create archives.
   - Linux/macOS: `.tar.gz`
   - Windows: `.zip`

3. Upload to GitHub Releases.

**Reference**: Examples of GitHub Actions used in existing Rust projects

- `rust-lang/cargo`
- `BurntSushi/ripgrep`

## Milestones

### Phase 1: Basic Implementation

- [x] Add `self update` subcommand.
- [x] Integrate `self_update` crate.
- [x] Download functionality from GitHub Releases.
- [x] Binary replacement functionality.
- [x] Basic error handling.

### Phase 2: Source Build Support

- [x] Implement `--from-source` option.
- [x] Integrate `git pull` + `cargo install`.
- [x] Repository path detection logic.
- [x] Fallback for no tracking information (origin/main).

### Phase 3: Additional Features

- [x] `--check` option (update check only).
- [x] `--version` option (update to a specific version).
- [x] `--force` option (force update).
- [ ] Progress bar display (supported by `self_update` crate).

### Phase 4: CI/CD Integration

- [x] Automate release builds with GitHub Actions.
- [x] Create platform-specific binaries.
- [x] Automate release note generation.
- [x] Parallel builds with Matrix build.
- [x] Automatic asset upload.

### Phase 5: Security Enhancement (Future)

- [ ] Implement binary signing.
- [ ] Checksum verification.
- [ ] Rollback functionality.

## References

### Implementations in similar projects

- **rustup**: Self-update for Rust toolchain.
- **cargo-update**: Update cargo packages.
- **ripgrep**: Self-update from GitHub Releases.

### Crates to be used

- [**self_update**](https://crates.io/crates/self_update)
  - Supports automatic updates from GitHub Releases.
  - Platform detection.
  - Binary replacement.

### Documentation

- [GitHub Releases API](https://docs.github.com/en/rest/releases)
- [cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html)

## Release Process

### Automated Release Builds

Release builds are automatically executed via GitHub Actions when a tag is pushed.

**Steps**:

1. Update version number (Cargo.toml).
2. Update CHANGELOG.
3. Commit and push.
4. Create and push tag.

```bash
# 1. Update version and edit CHANGELOG
vim Cargo.toml
vim CHANGELOG.md
git add Cargo.toml CHANGELOG.md
git commit -m "🔖 Bump version to 0.5.0"

# 2. Create and push tag
git tag v0.5.0
git push origin main
git push origin v0.5.0
```

### Platforms Built

GitHub Actions automatically generates binaries for the following platforms:

| Platform               | Architecture | Asset Name                                 |
|------------------------|--------------|--------------------------------------------|
| Linux                  | x86_64       | `gist-cache-rs-linux-x86_64.tar.gz`        |
| macOS (Intel)          | x86_64       | `gist-cache-rs-macos-x86_64.tar.gz`        |
| macOS (Apple Silicon)  | aarch64      | `gist-cache-rs-macos-aarch64.tar.gz`       |
| Windows                | x86_64       | `gist-cache-rs-windows-x86_64.zip`         |

### Workflow Details

Workflow defined in `.github/workflows/release.yml`:

1. **create-release**: Creates a release page.
   - Extracts version from tag.
   - Generates release notes (referring to CHANGELOG).
   - Includes installation instructions.

2. **build-release**: Platform-specific builds (parallel execution).
   - Installs Rust toolchain.
   - Executes release build.
   - Strips binary (Linux/macOS).
   - Creates archive.
   - Uploads to GitHub Releases.

### Troubleshooting

**If build fails**:

- Verify Cargo.toml version is correct.
- Verify dependencies are up-to-date (`cargo update`).
- Check GitHub Actions logs.

**If assets are not uploaded**:

- Check GitHub token permissions.
- Check workflow file for syntax errors.

## FAQ

### Q1: How to differentiate between setup script and self-update?

**A**:

- **Setup Script**: Used for initial installation (including PATH and alias settings).
- **Self Update**: Used for updating the installed version (updates only the binary).

### Q2: Why not execute the setup script internally?

**A**: The setup script includes alias and PATH settings, so re-executing it with every update would lead to unnecessary operations. `self update` focuses solely on binary updates, achieving simple and predictable behavior.

### Q3: What if GitHub Releases does not exist?

**A**: You can use the `--from-source` option to build from source. Alternatively, you can still use `cargo install --path .` or `cargo install --git` as before.

### Q4: What if an error occurs during an update?

**A**: The current binary will be preserved, allowing you to run it again. Rollback functionality will be considered in the future.

### Q5: Are there any platform-specific considerations?

**A**:

- **Windows**: When replacing a running binary, it is temporarily saved under a different name before replacement.
- **Linux/macOS**: Permissions are set appropriately (`chmod +x`).

## Summary

The `gist-cache-rs self update` command, unlike the setup script, **focuses solely on binary updates**. This ensures:

1. ✓ No re-execution of alias settings.
2. ✓ No changes to PATH settings.
3. ✓ Preservation of existing environment settings.
4. ✓ Simple and predictable behavior.
5. ✓ Cross-platform compatibility.

Phase 1 implemented basic updates from GitHub Releases, and features will be added incrementally thereafter.
