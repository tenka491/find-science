use async_trait::async_trait;
use serde::Deserialize;

pub mod http;
pub mod mock;

/// Number of projects returned per page.
pub const PAGE_SIZE: usize = 10;

#[derive(Debug, Default)]
pub struct ProjectQuery {
    pub keywords: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub page: usize,
}

#[derive(Debug, Deserialize)]
pub struct ProjectPage {
    pub total: Option<usize>,
    pub page: Option<usize>,
    pub entities: Vec<Project>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub name: Option<String>,
    pub link: Option<String>,
    pub goal: Option<String>,
    pub task: Option<String>,
    pub image: Option<String>,
}

#[async_trait]
pub trait ProjectsService: Send + Sync {
    async fn search(&self, query: &ProjectQuery) -> anyhow::Result<ProjectPage>;
}
