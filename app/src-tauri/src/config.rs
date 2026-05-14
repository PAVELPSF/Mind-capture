// Simple config storage for API keys.
// Keys are persisted as JSON in the app data directory alongside the database.
// TODO: upgrade to OS keychain via Tauri secure store plugin for production.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub active_provider: String,
    pub purgatory_batch_size: usize,
    pub last_exported_at: Option<String>,
    pub claude: ProviderConfig,
    pub openai: ProviderConfig,
    pub ollama: ProviderConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            active_provider: "claude".into(),
            purgatory_batch_size: 15,
            last_exported_at: None,
            claude: ProviderConfig {
                api_key: String::new(),
                model: "claude-sonnet-4-20250514".into(),
                enabled: false,
            },
            openai: ProviderConfig {
                api_key: String::new(),
                model: "gpt-4.1-nano".into(),
                enabled: false,
            },
            ollama: ProviderConfig {
                api_key: "http://localhost:11434".into(),
                model: "llama3.2".into(),
                enabled: false,
            },
        }
    }
}

impl AppConfig {
    fn path(app_data_dir: &std::path::Path) -> PathBuf {
        app_data_dir.join("config.json")
    }

    pub fn load(app_data_dir: &std::path::Path) -> Self {
        let path = Self::path(app_data_dir);
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, app_data_dir: &std::path::Path) -> Result<(), String> {
        let path = Self::path(app_data_dir);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("Serialize: {e}"))?;
        fs::write(&path, json).map_err(|e| format!("Write: {e}"))?;
        Ok(())
    }

    pub fn provider_config(&self, provider_id: &str) -> Option<&ProviderConfig> {
        match provider_id {
            "claude" => Some(&self.claude),
            "openai" => Some(&self.openai),
            "ollama" => Some(&self.ollama),
            _ => None,
        }
    }

    pub fn provider_config_mut(&mut self, provider_id: &str) -> Option<&mut ProviderConfig> {
        match provider_id {
            "claude" => Some(&mut self.claude),
            "openai" => Some(&mut self.openai),
            "ollama" => Some(&mut self.ollama),
            _ => None,
        }
    }
}
