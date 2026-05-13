//! OpenAI 兼容 LLM 客户端
//!
//! 支持环境变量 OPENAI_BASE_URL 切换后端。

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f64,
    pub response_format: Option<ResponseFormat>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
    pub json_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub message: ChatMessage,
}

pub struct LlmClient {
    client: Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl LlmClient {
    pub fn new() -> Self {
        let base_url = std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let api_key = std::env::var("OPENAI_API_KEY")
            .unwrap_or_default();
        let model = std::env::var("LLM_MODEL")
            .unwrap_or_else(|_| "gpt-4o".to_string());

        Self {
            client: Client::new(),
            base_url,
            api_key,
            model,
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        json_schema: Option<serde_json::Value>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/chat/completions", self.base_url);

        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.1,
            response_format: json_schema.map(|schema| ResponseFormat {
                format_type: "json_schema".to_string(),
                json_schema: Some(schema),
            }),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| "No response from LLM".into())
    }
}