use serde::{Deserialize, Serialize};

use crate::db::Database;

const MAX_TABS: usize = 500;
const MAX_URL_LEN: usize = 2048;
const MAX_TITLE_LEN: usize = 512;
const VALID_BROWSERS: &[&str] = &["Edge", "Chrome", "Firefox", "Unknown"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTab {
    pub url: String,
    pub title: String,
    pub favicon: Option<String>,
    pub browser: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub imported: usize,
    pub duplicates: usize,
}

fn validate_tab(tab: &NewTab) -> Result<(), String> {
    if tab.url.is_empty() || tab.url.len() > MAX_URL_LEN {
        return Err(format!(
            "Недопустимая длина URL: должно быть 1-{} символов",
            MAX_URL_LEN
        ));
    }
    if tab.title.is_empty() || tab.title.len() > MAX_TITLE_LEN {
        return Err(format!(
            "Недопустимая длина заголовка: должно быть 1-{} символов",
            MAX_TITLE_LEN
        ));
    }
    if !VALID_BROWSERS.contains(&tab.browser.as_str()) {
        return Err(format!(
            "Недопустимый браузер '{}': должен быть одним из {:?}",
            tab.browser, VALID_BROWSERS
        ));
    }
    Ok(())
}

pub fn import_tabs(db: &Database, tabs: Vec<NewTab>) -> Result<ImportResult, String> {
    if tabs.len() > MAX_TABS {
        return Err(format!(
            "Слишком много вкладок: {} (максимум {})",
            tabs.len(),
            MAX_TABS
        ));
    }

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut imported = 0usize;
    let mut duplicates = 0usize;

    for tab in &tabs {
        validate_tab(tab)?;

        let exists: bool = conn
            .query_row(
                "SELECT COUNT(*) > 0 FROM tabs WHERE url = ?1",
                rusqlite::params![tab.url],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        if exists {
            duplicates += 1;
            continue;
        }

        conn.execute(
            "INSERT INTO tabs (url, title, favicon, browser) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tab.url, tab.title, tab.favicon, tab.browser],
        )
        .map_err(|e| e.to_string())?;

        let tab_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO sync_log (action, entity_id) VALUES ('import', ?1)",
            rusqlite::params![tab_id],
        )
        .map_err(|e| e.to_string())?;

        imported += 1;
    }

    Ok(ImportResult {
        imported,
        duplicates,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_url() {
        let tab = NewTab {
            url: String::new(),
            title: "Test".into(),
            favicon: None,
            browser: "Chrome".into(),
        };
        assert!(validate_tab(&tab).is_err());
    }

    #[test]
    fn rejects_empty_title() {
        let tab = NewTab {
            url: "https://example.com".into(),
            title: String::new(),
            favicon: None,
            browser: "Chrome".into(),
        };
        assert!(validate_tab(&tab).is_err());
    }

    #[test]
    fn rejects_invalid_browser() {
        let tab = NewTab {
            url: "https://example.com".into(),
            title: "Test".into(),
            favicon: None,
            browser: "Opera".into(),
        };
        assert!(validate_tab(&tab).is_err());
    }

    #[test]
    fn accepts_valid_tab() {
        let tab = NewTab {
            url: "https://example.com".into(),
            title: "Example".into(),
            favicon: None,
            browser: "Edge".into(),
        };
        assert!(validate_tab(&tab).is_ok());
    }
}
