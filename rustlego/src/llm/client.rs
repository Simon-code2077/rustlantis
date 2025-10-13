use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::time::Duration;

pub struct LLMClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl LLMClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .or_else(|_| env::var("LLM_API_KEY"))
            .map_err(|_| anyhow!("API key not found in environment variables"))?;
        
        let base_url = env::var("LLM_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        
        let model = env::var("LLM_MODEL")
            .unwrap_or_else(|_| "gpt-4".to_string());

        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;

        Ok(Self {
            client,
            api_key,
            base_url,
            model,
        })
    }

    pub async fn generate_completion(&self, prompt: &str) -> Result<String> {
        let request_body = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are an expert Rust programmer specialized in generating correct, idiomatic Rust functions. Always follow the exact specifications in the prompt and ensure your code compiles."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.7,
            "max_tokens": 2048
        });

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("LLM API error: {}", error_text));
        }

        let response_json: Value = response.json().await?;
        
        let content = response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .ok_or_else(|| anyhow!("Invalid response format from LLM API"))?;

        Ok(content.to_string())
    }

    pub async fn generate_with_retry(&self, prompt: &str, max_retries: u32) -> Result<String> {
        let mut last_error = None;
        
        for attempt in 0..max_retries {
            match self.generate_completion(prompt).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries - 1 {
                        tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt))).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
}