use crate::error::{GistCacheError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Supported file extensions for interpreter configuration
pub const SUPPORTED_EXTENSIONS: &[&str] = &["py", "rb", "js", "ts", "sh", "php", "pl", "ps1", "*"];

/// Get valid interpreters for a specific extension
pub fn get_valid_interpreters_for_extension(extension: &str) -> &'static [&'static str] {
    match extension {
        "py" => &["uv", "python3"],
        "rb" => &["ruby"],
        "js" => &["node"],
        "ts" => &["ts-node", "deno", "bun"],
        "sh" => &["bash", "sh", "zsh", "fish"],
        "php" => &["php"],
        "pl" => &["perl"],
        "ps1" => &["pwsh"],
        "*" => &[
            "bash", "sh", "zsh", "fish", "python3", "uv", "ruby", "node", "ts-node", "deno", "bun",
            "php", "perl", "pwsh", "make",
        ],
        _ => &[],
    }
}

/// Interpreter setting that can be either a single string (legacy) or a map of extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InterpreterSetting {
    /// Legacy format: single interpreter string (e.g., "bash", "python3")
    Single(String),
    /// New format: map of file extensions to interpreters (e.g., {"py": "python3", "*": "bash"})
    Multiple(HashMap<String, String>),
}

impl Default for InterpreterSetting {
    fn default() -> Self {
        InterpreterSetting::Multiple(HashMap::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DefaultsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpreter: Option<InterpreterSetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_before_run: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<DefaultsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<ExecutionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<CacheConfig>,
}

#[derive(Clone)]
pub struct Config {
    pub cache_dir: PathBuf,
    pub cache_file: PathBuf,
    pub contents_dir: PathBuf,
    pub download_dir: PathBuf,
    pub config_file: PathBuf,
    pub user_config: UserConfig,
}

impl Config {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| GistCacheError::Config("Could not find home directory".to_string()))?;

        // Check for environment variable override first
        let (config_dir, cache_dir) = if let Ok(override_dir) = std::env::var("GIST_CACHE_DIR") {
            // Use GIST_CACHE_DIR for both config and cache
            let base_dir = PathBuf::from(override_dir);
            let cache_dir = base_dir.join("gist-cache");
            (base_dir.clone(), cache_dir)
        } else {
            // Config directory: follow platform standards
            #[cfg(unix)]
            let config_dir = home.join(".config").join("gist-cache");

            #[cfg(windows)]
            let config_dir = dirs::config_dir()
                .unwrap_or_else(|| home.join("AppData").join("Roaming"))
                .join("gist-cache");

            // Cache directory: follow platform standards
            #[cfg(unix)]
            let cache_dir = home.join(".cache").join("gist-cache");

            #[cfg(windows)]
            let cache_dir = dirs::cache_dir()
                .unwrap_or_else(|| home.join("AppData").join("Local"))
                .join("gist-cache");

            (config_dir, cache_dir)
        };

        let config_file = config_dir.join("config.toml");
        let cache_file = cache_dir.join("cache.json");
        let contents_dir = cache_dir.join("contents");

        // Load user config if exists
        let user_config = Self::load_user_config(&config_file)?;

        // Download directory: use platform standard
        let download_dir = dirs::download_dir().unwrap_or_else(|| home.join("Downloads"));

        Ok(Self {
            cache_dir,
            cache_file,
            contents_dir,
            download_dir,
            config_file,
            user_config,
        })
    }

    fn load_user_config(config_file: &PathBuf) -> Result<UserConfig> {
        if config_file.exists() {
            let content = std::fs::read_to_string(config_file)?;
            let config: UserConfig = toml::from_str(&content).map_err(|e| {
                GistCacheError::Config(format!("Failed to parse config file: {}", e))
            })?;
            Ok(config)
        } else {
            Ok(UserConfig::default())
        }
    }

    pub fn save_user_config(&self) -> Result<()> {
        // Ensure config directory exists
        if let Some(parent) = self.config_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(&self.user_config)
            .map_err(|e| GistCacheError::Config(format!("Failed to serialize config: {}", e)))?;
        std::fs::write(&self.config_file, content)?;
        Ok(())
    }

    pub fn set_config_value(&mut self, key: &str, value: &str) -> Result<()> {
        // Handle nested interpreter config (e.g., "defaults.interpreter.py")
        if let Some(extension) = key.strip_prefix("defaults.interpreter.") {
            if self.user_config.defaults.is_none() {
                self.user_config.defaults = Some(DefaultsConfig::default());
            }
            let defaults = self.user_config.defaults.as_mut().unwrap();

            // Get or create the interpreter map
            let interpreter_map = match &mut defaults.interpreter {
                Some(InterpreterSetting::Multiple(map)) => map,
                Some(InterpreterSetting::Single(_)) => {
                    // Convert single to multiple
                    defaults.interpreter = Some(InterpreterSetting::Multiple(HashMap::new()));
                    if let Some(InterpreterSetting::Multiple(map)) = &mut defaults.interpreter {
                        map
                    } else {
                        unreachable!()
                    }
                }
                None => {
                    defaults.interpreter = Some(InterpreterSetting::Multiple(HashMap::new()));
                    if let Some(InterpreterSetting::Multiple(map)) = &mut defaults.interpreter {
                        map
                    } else {
                        unreachable!()
                    }
                }
            };

            interpreter_map.insert(extension.to_string(), value.to_string());
        } else {
            match key {
                "defaults.interpreter" => {
                    if self.user_config.defaults.is_none() {
                        self.user_config.defaults = Some(DefaultsConfig::default());
                    }
                    // Legacy format: set as single interpreter or wildcard
                    self.user_config.defaults.as_mut().unwrap().interpreter =
                        Some(InterpreterSetting::Single(value.to_string()));
                }
                "execution.confirm_before_run" => {
                    let bool_value = value.parse::<bool>().map_err(|_| {
                        GistCacheError::Config(format!("Invalid boolean value: {}", value))
                    })?;
                    if self.user_config.execution.is_none() {
                        self.user_config.execution = Some(ExecutionConfig {
                            confirm_before_run: None,
                        });
                    }
                    self.user_config
                        .execution
                        .as_mut()
                        .unwrap()
                        .confirm_before_run = Some(bool_value);
                }
                "cache.retention_days" => {
                    let days = value.parse::<u32>().map_err(|_| {
                        GistCacheError::Config(format!("Invalid number value: {}", value))
                    })?;
                    if self.user_config.cache.is_none() {
                        self.user_config.cache = Some(CacheConfig {
                            retention_days: None,
                        });
                    }
                    self.user_config.cache.as_mut().unwrap().retention_days = Some(days);
                }
                _ => {
                    return Err(GistCacheError::Config(format!(
                        "Unknown config key: {}",
                        key
                    )));
                }
            }
        }
        self.save_user_config()
    }

    pub fn get_config_value(&self, key: &str) -> Option<String> {
        // Handle nested interpreter config (e.g., "defaults.interpreter.py")
        if let Some(extension) = key.strip_prefix("defaults.interpreter.") {
            let defaults = self.user_config.defaults.as_ref()?;
            match &defaults.interpreter {
                Some(InterpreterSetting::Multiple(map)) => map.get(extension).cloned(),
                Some(InterpreterSetting::Single(s)) if extension == "*" => Some(s.clone()),
                _ => None,
            }
        } else {
            match key {
                "defaults.interpreter" => {
                    let defaults = self.user_config.defaults.as_ref()?;
                    match &defaults.interpreter {
                        Some(InterpreterSetting::Single(s)) => Some(s.clone()),
                        Some(InterpreterSetting::Multiple(map)) => {
                            // Return wildcard if exists, or JSON representation
                            map.get("*").cloned().or_else(|| {
                                // Return a summary of the map
                                Some(format!("{} extensions configured", map.len()))
                            })
                        }
                        None => None,
                    }
                }
                "execution.confirm_before_run" => self
                    .user_config
                    .execution
                    .as_ref()?
                    .confirm_before_run
                    .map(|v| v.to_string()),
                "cache.retention_days" => self
                    .user_config
                    .cache
                    .as_ref()?
                    .retention_days
                    .map(|v| v.to_string()),
                _ => None,
            }
        }
    }

    pub fn reset_config(&mut self) -> Result<()> {
        self.user_config = UserConfig::default();
        if self.config_file.exists() {
            std::fs::remove_file(&self.config_file)?;
        }
        Ok(())
    }

    pub fn ensure_cache_dir(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            std::fs::create_dir_all(&self.cache_dir)?;
        }
        if !self.contents_dir.exists() {
            std::fs::create_dir_all(&self.contents_dir)?;
        }
        Ok(())
    }

    pub fn ensure_download_dir(&self) -> Result<()> {
        if !self.download_dir.exists() {
            std::fs::create_dir_all(&self.download_dir)?;
        }
        Ok(())
    }

    pub fn cache_exists(&self) -> bool {
        self.cache_file.exists()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("Failed to create config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_config_new() {
        let config = Config::new().unwrap();
        assert!(config.cache_dir.to_string_lossy().contains("gist-cache"));
        assert!(config.cache_file.to_string_lossy().contains("cache.json"));
        assert!(config.contents_dir.to_string_lossy().contains("contents"));
        assert!(!config.download_dir.to_string_lossy().is_empty());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.cache_dir.to_string_lossy().contains("gist-cache"));
    }

    #[test]
    fn test_ensure_cache_dir() {
        let config = Config::new().unwrap();
        let test_cache_dir = std::env::temp_dir().join("test_gist_cache");
        let test_contents_dir = test_cache_dir.join("contents");

        let test_config = Config {
            cache_dir: test_cache_dir.clone(),
            cache_file: test_cache_dir.join("cache.json"),
            contents_dir: test_contents_dir.clone(),
            download_dir: config.download_dir,
            config_file: test_cache_dir.join("config.toml"),
            user_config: UserConfig::default(),
        };

        // Clean up if exists
        let _ = fs::remove_dir_all(&test_cache_dir);

        // Ensure directories are created
        test_config.ensure_cache_dir().unwrap();
        assert!(test_cache_dir.exists());
        assert!(test_contents_dir.exists());

        // Clean up
        let _ = fs::remove_dir_all(&test_cache_dir);
    }

    #[test]
    fn test_ensure_download_dir() {
        let config = Config::new().unwrap();
        let test_download_dir = std::env::temp_dir().join("test_downloads");

        let test_config = Config {
            cache_dir: config.cache_dir,
            cache_file: config.cache_file,
            contents_dir: config.contents_dir,
            download_dir: test_download_dir.clone(),
            config_file: config.config_file,
            user_config: UserConfig::default(),
        };

        // Clean up if exists
        let _ = fs::remove_dir_all(&test_download_dir);

        // Ensure directory is created
        test_config.ensure_download_dir().unwrap();
        assert!(test_download_dir.exists());

        // Clean up
        let _ = fs::remove_dir_all(&test_download_dir);
    }

    #[test]
    fn test_cache_exists() {
        let config = Config::new().unwrap();
        let test_cache_dir = std::env::temp_dir().join("test_cache_exists");
        let test_cache_file = test_cache_dir.join("cache.json");

        let test_config = Config {
            cache_dir: test_cache_dir.clone(),
            cache_file: test_cache_file.clone(),
            contents_dir: test_cache_dir.join("contents"),
            download_dir: config.download_dir,
            config_file: config.config_file,
            user_config: UserConfig::default(),
        };

        // Clean up
        let _ = fs::remove_dir_all(&test_cache_dir);
        fs::create_dir_all(&test_cache_dir).unwrap();

        // File doesn't exist
        assert!(!test_config.cache_exists());

        // Create file
        fs::write(&test_cache_file, "{}").unwrap();
        assert!(test_config.cache_exists());

        // Clean up
        let _ = fs::remove_dir_all(&test_cache_dir);
    }

    #[test]
    fn test_set_nested_interpreter_config() {
        let temp_dir = std::env::temp_dir().join("test_nested_interpreter");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let mut config = Config {
            cache_dir: temp_dir.clone(),
            cache_file: temp_dir.join("cache.json"),
            contents_dir: temp_dir.join("contents"),
            download_dir: temp_dir.join("downloads"),
            config_file: temp_dir.join("config.toml"),
            user_config: UserConfig::default(),
        };

        // Set extension-specific interpreters
        config
            .set_config_value("defaults.interpreter.py", "python3")
            .unwrap();
        config
            .set_config_value("defaults.interpreter.rb", "ruby")
            .unwrap();
        config
            .set_config_value("defaults.interpreter.*", "bash")
            .unwrap();

        // Verify the values
        assert_eq!(
            config.get_config_value("defaults.interpreter.py"),
            Some("python3".to_string())
        );
        assert_eq!(
            config.get_config_value("defaults.interpreter.rb"),
            Some("ruby".to_string())
        );
        assert_eq!(
            config.get_config_value("defaults.interpreter.*"),
            Some("bash".to_string())
        );

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_legacy_single_interpreter_config() {
        let temp_dir = std::env::temp_dir().join("test_legacy_interpreter");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let mut config = Config {
            cache_dir: temp_dir.clone(),
            cache_file: temp_dir.join("cache.json"),
            contents_dir: temp_dir.join("contents"),
            download_dir: temp_dir.join("downloads"),
            config_file: temp_dir.join("config.toml"),
            user_config: UserConfig::default(),
        };

        // Set legacy single interpreter
        config
            .set_config_value("defaults.interpreter", "python3")
            .unwrap();

        // Verify it's stored as Single variant
        if let Some(defaults) = &config.user_config.defaults {
            if let Some(InterpreterSetting::Single(s)) = &defaults.interpreter {
                assert_eq!(s, "python3");
            } else {
                panic!("Expected Single variant");
            }
        } else {
            panic!("Expected defaults to be set");
        }

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_config_persistence() {
        let temp_dir = std::env::temp_dir().join("test_config_persistence");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let config_file = temp_dir.join("config.toml");

        // Create and save config
        {
            let mut config = Config {
                cache_dir: temp_dir.clone(),
                cache_file: temp_dir.join("cache.json"),
                contents_dir: temp_dir.join("contents"),
                download_dir: temp_dir.join("downloads"),
                config_file: config_file.clone(),
                user_config: UserConfig::default(),
            };

            config
                .set_config_value("defaults.interpreter.py", "python3")
                .unwrap();
            config
                .set_config_value("defaults.interpreter.ts", "deno")
                .unwrap();
        }

        // Load and verify
        {
            let loaded_config = Config::load_user_config(&config_file).unwrap();
            if let Some(defaults) = &loaded_config.defaults {
                if let Some(InterpreterSetting::Multiple(map)) = &defaults.interpreter {
                    assert_eq!(map.get("py"), Some(&"python3".to_string()));
                    assert_eq!(map.get("ts"), Some(&"deno".to_string()));
                } else {
                    panic!("Expected Multiple variant");
                }
            } else {
                panic!("Expected defaults to be set");
            }
        }

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
