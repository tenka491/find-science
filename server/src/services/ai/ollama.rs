use super::AiService;
use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaClient {
    pub base_url: String,
    pub client: Client,
    pub model: String,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
            model,
        }
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    images: Vec<String>,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[async_trait]
impl AiService for OllamaClient {
    async fn generate_tags(&self, image_base64: &str) -> Result<Vec<String>> {
        let prompt = r#"
            You are an image tagging system for scientific observations.

            Return 3 concise search keywords describing the image.

            Rules:
            - lowercase
            - comma separated
            - no numbering
            - no explanations
            - prefer the most specific organism name possible
            - if an animal species is visible, use the species name when possible
            - avoid vague words like animal, creature, object

            Example:
            dog, puppy, beagle
        "#;

        let req = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.into(),
            images: vec![image_base64.to_string()],
            stream: false,
        };

        let url = format!("{}/api/generate", self.base_url);

        let resp: OllamaResponse = self.client
            .post(url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        let tags = resp.response
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(tags)
    }
}