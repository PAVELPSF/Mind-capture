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

pub struct OpenAiProvider;

impl AiProvider for OpenAiProvider {
    fn id(&self) -> &str {
        "openai"
    }

    fn name(&self) -> &str {
        "OpenAI"
    }

    fn key_var(&self) -> &str {
        "OPENAI_API_KEY"
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
            max_completion_tokens: u32,
            messages: Vec<Message>,
            response_format: ResponseFormat,
        }

        #[derive(serde::Serialize)]
        struct ResponseFormat {
            #[serde(rename = "type")]
            format_type: String,
        }

        let body = RequestBody {
            model: "gpt-4.1-nano".into(),
            max_completion_tokens: 256,
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
            response_format: ResponseFormat {
                format_type: "json_object".into(),
            },
        };

        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| format!("Ошибка HTTP: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            return Err(format!("OpenAI API вернул {status}: {text}"));
        }

        #[derive(Deserialize)]
        struct Choice {
            message: MessageContent,
        }

        #[derive(Deserialize)]
        struct MessageContent {
            content: String,
        }

        #[derive(Deserialize)]
        struct ResponseBody {
            choices: Vec<Choice>,
        }

        let json: ResponseBody = resp.json().map_err(|e| format!("Ошибка парсинга JSON: {e}"))?;
        let text = json
            .choices
            .first()
            .map(|c| c.message.content.as_str())
            .unwrap_or("");

        serde_json::from_str(text).map_err(|e| format!("Ошибка парсинга анализа: {e}"))
    }

    fn is_available(&self, api_key: &str) -> bool {
        !api_key.is_empty() && api_key.starts_with("sk-")
    }
}
