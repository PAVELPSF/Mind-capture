use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::ai::{self, TabInfo};
use crate::config::{AppConfig, ProviderConfig};
use crate::db::Database;

#[derive(Debug, Serialize)]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub available: bool,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeTabsParams {
    pub tab_ids: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResult {
    pub analyzed: usize,
    pub failed: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetProviderParams {
    pub provider_id: String,
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetActiveProviderParams {
    pub provider_id: String,
}

#[tauri::command]
pub fn get_providers(
    db: State<Arc<Database>>,
) -> Result<Vec<ProviderInfo>, String> {
    let config = AppConfig::load(&*db.app_data_dir);

    Ok(ai::all_providers()
        .iter()
        .map(|p| {
            let cfg = config.provider_config(p.id()).cloned()
                .unwrap_or_else(|| ProviderConfig {
                    api_key: String::new(),
                    model: String::new(),
                    enabled: false,
                });
            ProviderInfo {
                id: p.id().into(),
                name: p.name().into(),
                available: p.is_available(&cfg.api_key),
                enabled: cfg.enabled,
            }
        })
        .collect())
}

#[tauri::command]
pub fn get_config(
    db: State<Arc<Database>>,
) -> Result<AppConfig, String> {
    Ok(AppConfig::load(&db.app_data_dir))
}

#[tauri::command]
pub fn set_provider(
    db: State<Arc<Database>>,
    params: SetProviderParams,
) -> Result<AppConfig, String> {
    let mut config = AppConfig::load(&db.app_data_dir);

    if let Some(cfg) = config.provider_config_mut(&params.provider_id) {
        if let Some(key) = params.api_key {
            cfg.api_key = key;
        }
        if let Some(model) = params.model {
            cfg.model = model;
        }
        if let Some(enabled) = params.enabled {
            cfg.enabled = enabled;
        }
    } else {
        return Err(format!("Неизвестный провайдер: {}", params.provider_id));
    }

    config.save(&db.app_data_dir)?;
    Ok(config)
}

#[tauri::command]
pub fn set_active_provider(
    db: State<Arc<Database>>,
    params: SetActiveProviderParams,
) -> Result<AppConfig, String> {
    let mut config = AppConfig::load(&db.app_data_dir);
    config.active_provider = params.provider_id;
    config.save(&db.app_data_dir)?;
    Ok(config)
}

#[tauri::command]
pub fn analyze_tabs(
    db: State<Arc<Database>>,
    params: AnalyzeTabsParams,
) -> Result<AnalyzeResult, String> {
    let config = AppConfig::load(&db.app_data_dir);
    let provider_id = &config.active_provider;
    let provider_cfg = config
        .provider_config(provider_id)
        .ok_or_else(|| format!("Неизвестный провайдер: {provider_id}"))?;

    if !provider_cfg.enabled {
        return Err(format!("Провайдер {provider_id} не включён"));
    }
    if provider_cfg.api_key.is_empty() {
        return Err("API-ключ не настроен для этого провайдера".into());
    }

    let providers = ai::all_providers();
    let provider = providers
        .iter()
        .find(|p| p.id() == provider_id)
        .ok_or_else(|| format!("Провайдер {provider_id} не найден"))?;

    // Load tabs from DB
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let tabs: Vec<TabInfo> = params
        .tab_ids
        .iter()
        .filter_map(|id| {
            conn.query_row(
                "SELECT id, url, title FROM tabs WHERE id = ?1",
                rusqlite::params![id],
                |row| {
                    Ok(TabInfo {
                        id: row.get(0)?,
                        url: row.get(1)?,
                        title: row.get(2)?,
                    })
                },
            )
            .ok()
        })
        .collect();
    drop(conn);

    let (analyzed, failed) = ai::analyze_batch(&db, provider.as_ref(), &provider_cfg.api_key, &tabs);

    Ok(AnalyzeResult { analyzed, failed })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_providers_are_created() {
        let providers = ai::all_providers();
        assert_eq!(providers.len(), 3);
        assert_eq!(providers[0].id(), "claude");
        assert_eq!(providers[1].id(), "openai");
        assert_eq!(providers[2].id(), "ollama");
    }
}
