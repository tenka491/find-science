use async_trait::async_trait;

pub mod mock;
pub mod ollama;


#[async_trait]
pub trait AiService: Send + Sync {
    async fn generate_tags(&self, image_base64: &str) -> anyhow::Result<Vec<String>>;
}