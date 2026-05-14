use serde::Deserialize;

use super::{AiProvider, Analysis};

const SYSTEM_PROMPT: &str = "\
You are a knowledge organizer. Analyze a browser tab's URL and title.
Return a JSON object with:
- topic: short topic label (2-5 words)
- summary: a 1-3 sentence description of what this page contains
- tags: array of 3-5 lowercase tags
- priority: number 0-10 indicating how useful this page is to keep (0=noise, 10=must-read)

Respond with ONLY the JSON object, no markdown, no explanation.";

pub struct ClaudeProvider;

impl AiProvider for ClaudeProvider {
    fn id(&self) -> &str {
        "claude"
    }

    fn name(&self) -> &str {
        "Claude API"
    }

    fn key_var(&self) -> &str {
        "CLAUDE_API_KEY"
    }

    fn analyze(&self, url: &str, title: &str, api_key: &str) -> Result<Analysis, String> {
        let client = reqwest::blocking::Client::new();

        #[derive(serde::Serialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(serde::Serialize)]
        struct RequestBody {
            model: String,
            max_tokens: u32,
            messages: Vec<Message>,
        }

        let body = RequestBody {
            model: "claude-sonnet-4-20250514".into(),
            max_tokens: 256,
            messages: vec![
                Message {
                    role: "system".into(),
                    content: SYSTEM_PROMPT.into(),
                },
                Message {
                    role: "user".into(),
                    content: format!("URL: {url}\nTitle: {title}"),
                },
            ],
        };

        let resp = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| format!("Ошибка HTTP: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("Claude API вернул {}", resp.status()));
        }

        #[derive(Deserialize)]
        struct ContentBlock {
            text: String,
        }

        #[derive(Deserialize)]
        struct ResponseBody {
            content: Vec<ContentBlock>,
        }

        let json: ResponseBody = resp.json().map_err(|e| format!("Ошибка парсинга JSON: {e}"))?;
        let text = json
            .content
            .first()
            .map(|c| c.text.as_str())
            .unwrap_or("");

        serde_json::from_str(text).map_err(|e| format!("Ошибка парсинга анализа: {e}"))
    }

    fn is_available(&self, api_key: &str) -> bool {
        !api_key.is_empty() && api_key.starts_with("sk-ant-")
    }
}
