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

pub struct OllamaProvider;

impl AiProvider for OllamaProvider {
    fn id(&self) -> &str {
        "ollama"
    }

    fn name(&self) -> &str {
        "Ollama (local)"
    }

    fn key_var(&self) -> &str {
        "OLLAMA_ENDPOINT"
    }

    fn analyze(&self, url: &str, title: &str, api_key: &str) -> Result<Analysis, String> {
        // For Ollama, api_key is the endpoint URL (e.g. http://localhost:11434)
        let endpoint = if api_key.is_empty() {
            "http://localhost:11434"
        } else {
            api_key
        };

        let client = reqwest::blocking::Client::new();

        #[derive(serde::Serialize)]
        struct RequestBody {
            model: String,
            stream: bool,
            messages: Vec<Message>,
        }

        #[derive(serde::Serialize)]
        struct Message {
            role: String,
            content: String,
        }

        let body = RequestBody {
            model: "llama3.2".into(),
            stream: false,
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
            .post(format!("{endpoint}/api/chat", endpoint = endpoint))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| format!("Ошибка HTTP Ollama: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!(
                "Ollama вернул {} — запущен ли он?",
                resp.status()
            ));
        }

        #[derive(Deserialize)]
        struct MessageContent {
            content: String,
        }

        #[derive(Deserialize)]
        struct ResponseBody {
            message: MessageContent,
        }

        let json: ResponseBody = resp.json().map_err(|e| format!("Ошибка парсинга JSON: {e}"))?;
        let text = &json.message.content;

        serde_json::from_str(text).map_err(|e| format!("Ошибка парсинга анализа: {e}"))
    }

    fn is_available(&self, api_key: &str) -> bool {
        let endpoint = if api_key.is_empty() {
            "http://localhost:11434"
        } else {
            api_key
        };
        reqwest::blocking::Client::new()
            .get(format!("{endpoint}/api/tags"))
            .send()
            .is_ok()
    }
}
