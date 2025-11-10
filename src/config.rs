// Simple configuration management

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub reminder: ReminderConfig,
    pub blink_detection: BlinkDetectionConfig,
    pub posture_detection: PostureDetectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderConfig {
    pub enabled: bool,
    pub interval_minutes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlinkDetectionConfig {
    pub enabled: bool,
    pub check_interval_seconds: u64,
    pub threshold_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureDetectionConfig {
    pub enabled: bool,
    pub sensitivity: String, // "low", "medium", "high"
}

impl Config {
    pub fn load_or_default() -> anyhow::Result<Self> {
        let config_path = Self::config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            let default = Self::default();
            default.save()?;
            Ok(default)
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("blinko");
        path.push("config.toml");
        path
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            reminder: ReminderConfig {
                enabled: true,
                interval_minutes: 20,
            },
            blink_detection: BlinkDetectionConfig {
                enabled: false,
                check_interval_seconds: 5,
                threshold_seconds: 30,
            },
            posture_detection: PostureDetectionConfig {
                enabled: false,
                sensitivity: "medium".to_string(),
            },
        }
    }
}

