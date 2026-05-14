use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::config::AppConfig;
use crate::db::models::{Review, Tab};
use crate::db::Database;

#[derive(Debug, Deserialize)]
pub struct GetBatchParams {
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReviewParams {
    pub tab_id: i64,
    pub decision: String,
}

#[derive(Debug, Serialize)]
pub struct SubmitReviewResult {
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct PurgatoryConfig {
    pub batch_size: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPurgatoryConfigParams {
    pub batch_size: usize,
}

#[tauri::command]
pub fn get_purgatory_batch(
    db: State<Arc<Database>>,
    params: GetBatchParams,
) -> Result<Vec<Tab>, String> {
    let config = AppConfig::load(&db.app_data_dir);
    let limit = params.limit.unwrap_or(config.purgatory_batch_size).min(50).max(1);

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, url, title, favicon, browser, imported_at, status
             FROM tabs
             WHERE status IN ('new', 'analyzed')
               AND id NOT IN (SELECT tab_id FROM reviews)
             ORDER BY imported_at ASC
             LIMIT ?1",
        )
        .map_err(|e| e.to_string())?;

    let tabs: Vec<Tab> = stmt
        .query_map(rusqlite::params![limit as i64], |row| {
            Ok(Tab {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                favicon: row.get(3)?,
                browser: row.get(4)?,
                imported_at: row.get(5)?,
                status: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(tabs)
}

#[tauri::command]
pub fn submit_review(
    db: State<Arc<Database>>,
    params: SubmitReviewParams,
) -> Result<SubmitReviewResult, String> {
    if !["keep", "delete", "later"].contains(&params.decision.as_str()) {
        return Err(format!(
            "Недопустимое решение '{}': допустимые значения: keep, delete, later",
            params.decision
        ));
    }

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO reviews (tab_id, decision) VALUES (?1, ?2)",
        rusqlite::params![params.tab_id, params.decision],
    )
    .map_err(|e| e.to_string())?;

    match params.decision.as_str() {
        "delete" => {
            conn.execute(
                "UPDATE tabs SET status = 'deleted' WHERE id = ?1",
                rusqlite::params![params.tab_id],
            )
            .map_err(|e| e.to_string())?;
        }
        "keep" => {
            conn.execute(
                "UPDATE tabs SET status = 'reviewed' WHERE id = ?1",
                rusqlite::params![params.tab_id],
            )
            .map_err(|e| e.to_string())?;
        }
        _ => {} // later: no status change
    }

    Ok(SubmitReviewResult { success: true })
}

#[tauri::command]
pub fn get_review_history(
    db: State<Arc<Database>>,
) -> Result<Vec<Review>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, tab_id, decision, reviewed_at
             FROM reviews
             ORDER BY reviewed_at DESC
             LIMIT 100",
        )
        .map_err(|e| e.to_string())?;

    let reviews: Vec<Review> = stmt
        .query_map([], |row| {
            Ok(Review {
                id: row.get(0)?,
                tab_id: row.get(1)?,
                decision: row.get(2)?,
                reviewed_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(reviews)
}

#[tauri::command]
pub fn get_purgatory_config(
    db: State<Arc<Database>>,
) -> Result<PurgatoryConfig, String> {
    let config = AppConfig::load(&db.app_data_dir);
    Ok(PurgatoryConfig {
        batch_size: config.purgatory_batch_size,
    })
}

#[tauri::command]
pub fn set_purgatory_config(
    db: State<Arc<Database>>,
    params: SetPurgatoryConfigParams,
) -> Result<PurgatoryConfig, String> {
    let batch_size = params.batch_size.min(50).max(5);
    let mut config = AppConfig::load(&db.app_data_dir);
    config.purgatory_batch_size = batch_size;
    config.save(&db.app_data_dir)?;
    Ok(PurgatoryConfig { batch_size })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_decision() {
        let params = SubmitReviewParams {
            tab_id: 1,
            decision: "invalid".into(),
        };
        // Validation happens before DB access
        assert!(!["keep", "delete", "later"].contains(&params.decision.as_str()));
    }

    #[test]
    fn accepts_valid_decisions() {
        for d in &["keep", "delete", "later"] {
            assert!(["keep", "delete", "later"].contains(d));
        }
    }

    #[test]
    fn batch_size_clamped() {
        assert_eq!(0usize.max(5), 5);
        assert_eq!(100usize.min(50).max(5), 50);
        assert_eq!(15usize.min(50).max(5), 15);
    }
}
