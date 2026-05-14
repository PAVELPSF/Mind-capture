pub mod claude;
pub mod ollama;
pub mod openai;

use serde::{Deserialize, Serialize};

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub topic: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabInfo {
    pub id: i64,
    pub url: String,
    pub title: String,
}

/// Core trait for AI providers. Each provider implements analyze() and
/// metadata methods so the UI can discover and select them.
pub trait AiProvider: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn analyze(&self, url: &str, title: &str, api_key: &str) -> Result<Analysis, String>;
    fn is_available(&self, api_key: &str) -> bool;

    #[allow(dead_code)]
    /// Return the recommended environment variable name for the API key.
    fn key_var(&self) -> &str;
}

/// Build a Vec of all available providers.  Each provider checks whether
/// its API key / endpoint is configured; unavailable providers are still
/// returned so the UI can list them, but is_available() will be false.
pub fn all_providers() -> Vec<Box<dyn AiProvider>> {
    vec![
        Box::new(claude::ClaudeProvider),
        Box::new(openai::OpenAiProvider),
        Box::new(ollama::OllamaProvider),
    ]
}

/// Analyse a batch of tabs with a given provider.  Returns (analysed, failed)
/// counts.  A failed tab does not block the rest of the batch.
pub fn analyze_batch(
    db: &Database,
    provider: &dyn AiProvider,
    api_key: &str,
    tabs: &[TabInfo],
) -> (usize, usize) {
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(_) => return (0, tabs.len()),
    };

    let mut analysed = 0usize;
    let mut failed = 0usize;

    for tab in tabs {
        match provider.analyze(&tab.url, &tab.title, api_key) {
            Ok(analysis) => {
                let tags_json = serde_json::to_string(&analysis.tags).unwrap_or_default();
                let result = conn.execute(
                    "INSERT INTO notes (tab_id, content, tags, priority) VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![
                        tab.id,
                        format!("{}\n\n{}", analysis.topic, analysis.summary),
                        tags_json,
                        analysis.priority,
                    ],
                );
                if result.is_ok() {
                    let _ = conn.execute(
                        "UPDATE tabs SET status = 'analyzed' WHERE id = ?1",
                        rusqlite::params![tab.id],
                    );
                    analysed += 1;
                } else {
                    failed += 1;
                }
            }
            Err(_) => {
                failed += 1;
            }
        }
    }

    (analysed, failed)
}
