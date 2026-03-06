use async_trait::async_trait;

pub mod mock;
pub mod ollama;

#[derive(Debug)]
pub enum AiError {
    RequestFailed,
    InvalidResponse,
}

#[async_trait]
pub trait AiService: Send + Sync {
    async fn generate_tags(&self, image_base64: &str) -> anyhow::Result<Vec<String>>;
}