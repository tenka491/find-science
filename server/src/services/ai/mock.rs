use super::AiService;
use async_trait::async_trait;
use anyhow::Result;

pub struct MockAiService;

#[async_trait]
impl AiService for MockAiService {
    async fn generate_tags(&self, _image: &str) -> Result<Vec<String>> {
        Ok(vec![
            "science".into(),
            "experiment".into(),
            "biology".into(),
        ])
    }
}