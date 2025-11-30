use crate::error::Result;
use colored::Colorize;
use std::env;

/// Self-update options
#[derive(Debug)]
pub struct UpdateOptions {
    pub from_source: bool,
    pub check: bool,
    pub force: bool,
    pub version: Option<String>,
    pub verbose: bool,
}

/// Self-updater for the application
pub struct Updater {
    options: UpdateOptions,
}

impl Updater {
    /// Create a new updater with the given options
    pub fn new(options: UpdateOptions) -> Self {
        Self { options }
    }

    /// Run the self-update process
    pub fn update(&self) -> Result<()> {
        if self.options.verbose {
            println!("{}", "=== Self-Update ===".cyan().bold());
        }

        // Get current version
        let current_version = env!("CARGO_PKG_VERSION");
        println!("現在のバージョン: {}", current_version.green());

        if self.options.from_source {
            self.update_from_source()
        } else {
            self.update_from_releases()
        }
    }

    /// Update from GitHub Releases
    fn update_from_releases(&self) -> Result<()> {
        if self.options.verbose {
            println!("{}", "GitHub Releasesから更新を確認しています...".cyan());
        }

        // Build updater
        let mut builder = self_update::backends::github::Update::configure();
        builder
            .repo_owner("7rikazhexde")
            .repo_name("gist-cache-rs")
            .bin_name("gist-cache-rs")
            .show_download_progress(true)
            .current_version(env!("CARGO_PKG_VERSION"));

        // Apply options
        if let Some(ref version) = self.options.version {
            builder.target_version_tag(version);
        } else {
            builder.no_confirm(true);
        }

        // Check mode: just check for updates
        if self.options.check {
            let release = builder.build()?.get_latest_release()?;
            let latest_version = &release.version;

            println!("最新バージョン: {}", latest_version.green());

            if latest_version.as_str() > env!("CARGO_PKG_VERSION") {
                println!(
                    "{}",
                    format!("新しいバージョン {} が利用可能です", latest_version)
                        .yellow()
                        .bold()
                );
                println!("更新するには: {}", "gist-cache-rs self update".cyan());
            } else {
                println!("{}", "最新版を使用しています".green().bold());
            }
            return Ok(());
        }

        // Perform update
        println!("{}", "更新を確認しています...".cyan());

        let status = builder.build()?.update()?;

        match status {
            self_update::Status::UpToDate(version) => {
                println!(
                    "{}",
                    format!("すでに最新版です ({})", version).green().bold()
                );
            }
            self_update::Status::Updated(version) => {
                println!(
                    "{}",
                    format!("更新が完了しました: {}", version).green().bold()
                );
                println!("新しいバージョンで再起動してください。");
            }
        }

        Ok(())
    }

    /// Update from source (git + cargo install)
    fn update_from_source(&self) -> Result<()> {
        println!("{}", "ソースからの更新は未実装です".yellow());
        println!("GitHub Releasesからの更新を使用してください:");
        println!("  {}", "gist-cache-rs self update".cyan());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updater_creation() {
        let options = UpdateOptions {
            from_source: false,
            check: false,
            force: false,
            version: None,
            verbose: false,
        };

        let updater = Updater::new(options);
        assert!(!updater.options.from_source);
        assert!(!updater.options.check);
        assert!(!updater.options.force);
        assert!(updater.options.version.is_none());
        assert!(!updater.options.verbose);
    }

    #[test]
    fn test_updater_with_options() {
        let options = UpdateOptions {
            from_source: true,
            check: true,
            force: true,
            version: Some("0.5.0".to_string()),
            verbose: true,
        };

        let updater = Updater::new(options);
        assert!(updater.options.from_source);
        assert!(updater.options.check);
        assert!(updater.options.force);
        assert_eq!(updater.options.version, Some("0.5.0".to_string()));
        assert!(updater.options.verbose);
    }
}
