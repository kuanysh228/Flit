use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub reading: ReadingConfig,
    pub ui: UiConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ReadingConfig {
    pub default_wpm: u16,
    pub min_wpm: u16,
    pub max_wpm: u16,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    pub theme: String,
    pub show_progress: bool,
    pub show_wpm: bool,
    pub show_clock: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct StorageConfig {
    pub db_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            reading: ReadingConfig::default(),
            ui: UiConfig::default(),
            storage: StorageConfig::default(),
        }
    }
}

impl Default for ReadingConfig {
    fn default() -> Self {
        Self { default_wpm: 300, min_wpm: 100, max_wpm: 1200 }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            show_progress: true,
            show_wpm: true,
            show_clock: true,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self { db_path: None }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = crate::paths::config_path();
        if !path.exists() {
            return Self::default();
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => return Self::default(),
        };
        toml::from_str(&content).unwrap_or_default()
    }
}
