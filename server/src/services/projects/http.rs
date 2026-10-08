use super::{ProjectPage, ProjectQuery, ProjectsService};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Client, Url};

/// Calls a remote projects finder API that returns `ProjectPage` JSON.
pub struct HttpProjectsService {
    pub api_url: Url,
    pub client: Client,
}

impl HttpProjectsService {
    pub fn new(api_url: &str) -> Result<Self> {
        Ok(Self {
            api_url: Url::parse(api_url)?,
            client: Client::new(),
        })
    }
}

#[async_trait]
impl ProjectsService for HttpProjectsService {
    async fn search(&self, query: &ProjectQuery) -> Result<ProjectPage> {
        let mut url = self.api_url.clone();

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in [
                ("lat", &query.latitude),
                ("lng", &query.longitude),
                ("q", &query.keywords),
            ] {
                if let Some(value) = value.as_deref().filter(|v| !v.is_empty()) {
                    pairs.append_pair(key, value);
                }
            }
            pairs.append_pair("page", &query.page.to_string());
        }

        tracing::info!("Projects API URL: {}", url);

        Ok(self.client.get(url).send().await?.json().await?)
    }
}
