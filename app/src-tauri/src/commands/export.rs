use std::sync::Arc;

use serde::Serialize;
use tauri::State;

use crate::config::AppConfig;
use crate::db::Database;

#[derive(Debug, Serialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct BookmarkFolder {
    pub name: String,
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Serialize)]
pub struct ExportPayload {
    pub folders: Vec<BookmarkFolder>,
    pub total: usize,
    pub delta: bool,
}

#[derive(Debug, Serialize)]
pub struct ExportHtmlResult {
    pub path: String,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct ExportStatus {
    pub ready_count: usize,
    pub last_export: Option<String>,
}

fn fetch_export_tabs(db: &Database, _delta: bool) -> Result<Vec<(String, String, String)>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Delta export: only tabs with status 'reviewed' or 'analyzed'
    // (tabs already exported have status 'exported' and are skipped)
    let sql = String::from(
        "SELECT t.title, t.url, COALESCE(c.name, 'Unsorted') as folder
         FROM tabs t
         LEFT JOIN tab_collections tc ON t.id = tc.tab_id
         LEFT JOIN collections c ON tc.collection_id = c.id
         WHERE t.status IN ('reviewed', 'analyzed')
         ORDER BY c.name, t.imported_at DESC",
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows: Vec<(String, String, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(rows)
}

fn build_folders(rows: Vec<(String, String, String)>) -> Vec<BookmarkFolder> {
    let mut folders: Vec<BookmarkFolder> = vec![];
    let mut current_name: Option<String> = None;
    let mut current_bookmarks: Vec<Bookmark> = vec![];

    for (title, url, folder) in rows {
        let folder_name = if folder.is_empty() { "Unsorted".into() } else { folder };

        if current_name.as_ref() != Some(&folder_name) {
            if let Some(name) = current_name.take() {
                folders.push(BookmarkFolder {
                    name,
                    bookmarks: std::mem::take(&mut current_bookmarks),
                });
            }
            current_name = Some(folder_name);
        }

        current_bookmarks.push(Bookmark { title, url });
    }

    if let Some(name) = current_name {
        folders.push(BookmarkFolder {
            name,
            bookmarks: current_bookmarks,
        });
    }

    folders
}

fn mark_exported(db: &Database) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = now_timestamp();

    conn.execute(
        "UPDATE tabs SET status = 'exported' WHERE status IN ('reviewed', 'analyzed')",
        [],
    )
    .map_err(|e| e.to_string())?;

    let mut config = AppConfig::load(&db.app_data_dir);
    config.last_exported_at = Some(now);
    config.save(&db.app_data_dir)?;

    Ok(())
}

fn now_timestamp() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", dur.as_secs())
}

fn now_date_label() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    // Approximate date from UNIX timestamp for display only
    let days_since_epoch = secs / 86400;
    // 1970-01-01 + days → rough year/month/day
    let days = days_since_epoch as i64;
    // Use civil calendar approximation
    let era = if days >= 0 { days } else { days - 146096 };
    let doe = ((era % 146097) + 146097) % 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era / 146097 * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", y, m, d)
}

pub fn get_export_payload_inner(db: &Arc<Database>) -> Result<ExportPayload, String> {
    let config = AppConfig::load(&db.app_data_dir);
    let delta = config.last_exported_at.is_some();
    let rows = fetch_export_tabs(db, delta)?;
    let total = rows.len();
    let folders = build_folders(rows);

    Ok(ExportPayload {
        folders,
        total,
        delta,
    })
}

#[tauri::command]
pub fn get_export_payload(
    db: State<Arc<Database>>,
) -> Result<ExportPayload, String> {
    get_export_payload_inner(&db)
}

#[tauri::command]
pub fn export_html(
    db: State<Arc<Database>>,
) -> Result<ExportHtmlResult, String> {
    let config = AppConfig::load(&db.app_data_dir);
    let delta = config.last_exported_at.is_some();
    let rows = fetch_export_tabs(&db, delta)?;
    let total = rows.len();
    let folders = build_folders(rows);

    let date_label = now_date_label();

    let mut html = String::from(
        "<!DOCTYPE NETSCAPE-Bookmark-file-1>\n\
         <META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">\n\
         <TITLE>MindCapture Export</TITLE>\n\
         <H1>MindCapture Export — ",
    );
    html.push_str(&date_label);
    html.push_str("</H1>\n<DL><p>\n");

    for folder in &folders {
        html.push_str("  <DT><H3>");
        html.push_str(&escape_html(&folder.name));
        html.push_str("</H3>\n  <DL><p>\n");

        for bm in &folder.bookmarks {
            html.push_str("    <DT><A HREF=\"");
            html.push_str(&escape_html(&bm.url));
            html.push_str("\">");
            html.push_str(&escape_html(&bm.title));
            html.push_str("</A>\n");
        }

        html.push_str("  </DL><p>\n");
    }

    html.push_str("</DL><p>\n");

    let exports_dir = db.app_data_dir.join("exports");
    std::fs::create_dir_all(&exports_dir)
        .map_err(|e| format!("Failed to create exports dir: {e}"))?;

    let filename = format!("MindCapture_{}.html", date_label);
    let path = exports_dir.join(&filename);

    std::fs::write(&path, &html)
        .map_err(|e| format!("Failed to write HTML: {e}"))?;

    mark_exported(&db)?;

    Ok(ExportHtmlResult {
        path: path.to_string_lossy().into(),
        total,
    })
}

#[tauri::command]
pub fn get_export_status(
    db: State<Arc<Database>>,
) -> Result<ExportStatus, String> {
    let config = AppConfig::load(&db.app_data_dir);
    let rows = fetch_export_tabs(&db, config.last_exported_at.is_some())?;

    Ok(ExportStatus {
        ready_count: rows.len(),
        last_export: config.last_exported_at,
    })
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
