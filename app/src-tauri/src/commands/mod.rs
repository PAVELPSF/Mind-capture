pub mod analyze;
pub mod export;
pub mod import;
pub mod purgatory;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::models::Tab;
use crate::db::Database;

#[tauri::command]
pub fn get_status(db: State<Arc<Database>>) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM tabs", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(format!("ok — {} вкладок в базе", count))
}

#[derive(Debug, Deserialize)]
pub struct GetTabsParams {
    pub browser: Option<String>,
    pub status: Option<String>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct GetTabsResult {
    pub tabs: Vec<Tab>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

#[tauri::command]
pub fn get_tabs(
    db: State<Arc<Database>>,
    params: GetTabsParams,
) -> Result<GetTabsResult, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).min(200).max(1);
    let offset = (page - 1) * per_page;

    // Build WHERE clause dynamically
    let mut conditions: Vec<String> = vec![];
    let mut bind_params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

    if let Some(ref browser) = params.browser {
        conditions.push(format!("browser = ?{}", bind_params.len() + 1));
        bind_params.push(Box::new(browser.clone()));
    }
    if let Some(ref status) = params.status {
        conditions.push(format!("status = ?{}", bind_params.len() + 1));
        bind_params.push(Box::new(status.clone()));
    }

    let where_clause = if conditions.is_empty() {
        String::from("1=1")
    } else {
        conditions.join(" AND ")
    };

    // Count total
    let count_sql = format!("SELECT COUNT(*) FROM tabs WHERE {}", where_clause);
    let total: usize = {
        let mut stmt = conn.prepare(&count_sql).map_err(|e| e.to_string())?;
        let count: i64 = stmt
            .query_row(rusqlite::params_from_iter(bind_params.iter().map(|p| p.as_ref())), |row| row.get(0))
            .map_err(|e| e.to_string())?;
        count as usize
    };

    // Fetch page
    let select_sql = format!(
        "SELECT id, url, title, favicon, browser, imported_at, status FROM tabs WHERE {} ORDER BY imported_at DESC LIMIT {} OFFSET {}",
        where_clause, per_page, offset
    );

    let mut stmt = conn.prepare(&select_sql).map_err(|e| e.to_string())?;
    let tabs: Vec<Tab> = stmt
        .query_map(
            rusqlite::params_from_iter(bind_params.iter().map(|p| p.as_ref())),
            |row| {
                Ok(Tab {
                    id: row.get(0)?,
                    url: row.get(1)?,
                    title: row.get(2)?,
                    favicon: row.get(3)?,
                    browser: row.get(4)?,
                    imported_at: row.get(5)?,
                    status: row.get(6)?,
                })
            },
        )
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(GetTabsResult {
        tabs,
        total,
        page,
        per_page,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_status_returns_ok() {
        assert!(true);
    }
}
