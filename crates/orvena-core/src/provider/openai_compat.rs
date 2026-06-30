//! OpenAI-compatible provider (OpenAI and OpenRouter) over /chat/completions.

use super::{ChatRequest, ChatResponse, Provider};
use crate::config::agent::ProviderSelection;
use crate::error::{Error, Result};
use async_trait::async_trait;

pub struct OpenAiCompat {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
    id: &'static str,
}

impl OpenAiCompat {
    pub fn from_env(sel: &ProviderSelection) -> Result<Self> {
        let (env_key, default_base, id) = match sel.kind.as_str() {
            "openrouter" => ("OPENROUTER_API_KEY", "https://openrouter.ai/api/v1", "openrouter"),
            _ => ("OPENAI_API_KEY", "https://api.openai.com/v1", "openai"),
        };
        let api_key = std::env::var(env_key)
            .map_err(|_| Error::Provider(format!("{env_key} is not set — put it in .env")))?;
        Ok(Self {
            client: reqwest::Client::new(),
            api_key,
            model: sel.model.clone(),
            base_url: sel.base_url.clone().unwrap_or_else(|| default_base.to_string()),
            id,
        })
    }
}

#[async_trait]
impl Provider for OpenAiCompat {
    fn id(&self) -> &str {
        self.id
    }

    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse> {
        let messages: Vec<serde_json::Value> = req
            .messages
            .iter()
            .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
            .collect();

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": req.max_tokens,
            "messages": messages,
        });

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::Provider(format!("{} returned {status}: {text}", self.id)));
        }

        let v: serde_json::Value = resp.json().await?;
        let content = v["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let input_tokens = v["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32;
        let output_tokens = v["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;

        Ok(ChatResponse { content, input_tokens, output_tokens })
    }
}
