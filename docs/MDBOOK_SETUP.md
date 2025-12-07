# mdbook Documentation Setup

This document describes how to set up and deploy the mdbook documentation for gist-cache-rs.

## Local Development

### Prerequisites

Install mdbook:

```bash
cargo install mdbook
```

### Build and Serve Locally

```bash
# Build the documentation
mdbook build

# Serve with live reload (recommended for development)
mdbook serve

# The documentation will be available at http://localhost:3000
```

### Directory Structure

```
gist-cache-rs/
├── book.toml                     # mdbook configuration
├── book/
│   ├── src/                      # Source markdown files
│   │   ├── README.md
│   │   ├── SUMMARY.md            # Table of contents
│   │   ├── user-guide/
│   │   ├── developer-guide/
│   │   └── test-specs/
│   └── book/                     # Generated HTML (gitignored)
└── .github/workflows/
    └── deploy-mdbook.yml         # GitHub Pages deployment
```

## GitHub Pages Deployment

### Initial Setup

1. **Enable GitHub Pages in Repository Settings**
   - Go to your repository on GitHub
   - Navigate to `Settings` → `Pages`
   - Under "Source", select `GitHub Actions`

2. **Push Changes to Main Branch**
   - The workflow will automatically trigger when changes to documentation files are pushed
   - Monitored paths:
     - `book/**`
     - `book.toml`
     - `docs/**`
     - `README.md`
     - `.github/workflows/deploy-mdbook.yml`

3. **Manual Deployment (Optional)**
   - Go to `Actions` tab in your repository
   - Select `Deploy mdbook to GitHub Pages` workflow
   - Click `Run workflow` → `Run workflow`

### Deployment Workflow

The deployment workflow (`.github/workflows/deploy-mdbook.yml`) performs the following:

1. **Build**
   - Checks out the repository
   - Sets up mdbook (latest version)
   - Builds the documentation with `mdbook build`
   - Uploads the generated HTML as a Pages artifact

2. **Deploy**
   - Deploys the artifact to GitHub Pages
   - The documentation will be available at:
     `https://<username>.github.io/<repository-name>/`

### Verification

After deployment:

1. Check the Actions tab for workflow status
2. Once completed, visit your GitHub Pages URL:
   - For this repository: `https://7rikazhexde.github.io/gist-cache-rs/`

### Troubleshooting

#### Workflow fails with "Pages deployment is not enabled"

- **Solution**: Ensure GitHub Pages is enabled in repository settings with source set to "GitHub Actions"

#### Changes not reflected after deployment

- **Solution**:
  - Check that your changes were committed to the main branch
  - Wait a few minutes for GitHub Pages cache to update
  - Try a hard refresh in your browser (Ctrl+Shift+R)

#### Build fails with "mdbook not found"

- **Solution**: This should not happen in CI as the workflow installs mdbook automatically. For local development, run `cargo install mdbook`

## Updating Documentation

### Editing Content

1. Edit markdown files in `book/src/`
2. Test locally with `mdbook serve`
3. Commit and push changes
4. The workflow will automatically rebuild and deploy

### Adding New Pages

1. Create a new markdown file in the appropriate directory:
   - User documentation: `book/src/user-guide/`
   - Developer documentation: `book/src/developer-guide/`
   - Test specifications: `book/src/test-specs/`

2. Add the new page to `book/src/SUMMARY.md`:
   ```markdown
   - [New Page Title](path/to/new-page.md)
   ```

3. Test locally and push changes

### Modifying Structure

Edit `book/src/SUMMARY.md` to reorganize the table of contents. The structure in SUMMARY.md determines the navigation menu.

## Configuration

### book.toml

Key configurations:

- `title`: Documentation title
- `src`: Source directory (`book/src`)
- `build-dir`: Output directory (`book/book`)
- Theme: Rust theme with Navy dark theme
- Features: Search, code folding, edit links

### Customization

To customize the appearance:

1. Add custom CSS in `book/theme/custom.css`
2. Modify `book.toml` settings
3. See [mdbook documentation](https://rust-lang.github.io/mdBook/) for all options

## CI/CD Integration

The deployment workflow integrates with:

- **Trigger**: Push to main branch or manual workflow dispatch
- **Permissions**: Read contents, write to Pages, id-token for deployment
- **Concurrency**: Only one deployment runs at a time

## Additional Resources

- [mdbook User Guide](https://rust-lang.github.io/mdBook/)
- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [GitHub Actions for Pages](https://github.com/actions/deploy-pages)
