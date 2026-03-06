use poem::{
    handler,
    web::{ Multipart, Data, Html, CsrfToken },
    Result,
};
use sailfish::TemplateOnce;
use std::sync::Arc;
use serde::Deserialize;
use reqwest::Url;

use crate::render::render_with_layout;
use crate::services::ai::AiService;
use crate::utils::image::encode_image;

#[derive(Deserialize)]
struct SearchForm {
    query: Option<String>,
}

#[derive(TemplateOnce)]
#[template(path = "search.stpl.html")]
struct SearchPageTemplate {
    csrf_token: String,
}

#[derive(TemplateOnce)]
#[template(path = "search_results.stpl.html")]
struct SearchResultsTemplate {
    query: String,
    results: Vec<String>,
    ai_tags: Vec<String>,
    projects_url: String,
}

#[handler]
pub async fn search_page(token: &CsrfToken) -> Result<Html<String>> {
    render_with_layout("Search", SearchPageTemplate {
        csrf_token: token.0.clone(),
    })
}

#[handler]
pub async fn search_results(
    mut multipart: Multipart,
    ai_service: Data<&Arc<dyn AiService>>,
    ) -> Result<Html<String>> {
    // Initialize all form fields
    let mut keywords = None;
    let mut latitude = None;
    let mut longitude = None;
    let mut events = None;
    let mut projects = None;
    let mut active = None;
    let mut upcoming = None;
    let mut image_bytes = None;

    // Parse multipart fields
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        tracing::info!("Received field: {}", name);

        match name.as_str() {
            "keywords" => keywords = Some(field.text().await?),
            "latitude" => latitude = Some(field.text().await?),
            "longitude" => longitude = Some(field.text().await?),
            "events" => events = Some(field.text().await?),
            "projects" => projects = Some(field.text().await?),
            "active" => active = Some(field.text().await?),
            "upcoming" => upcoming = Some(field.text().await?),
            "image" => image_bytes = Some(field.bytes().await?.to_vec()),
            _ => {}
        }
    }

    let mut final_keywords = keywords.clone().unwrap_or_default();
    let mut ai_tags = vec![];

    // If image is present, get AI tags
    if let Some(bytes) = image_bytes {
        let base64_img = encode_image(&bytes);
        tracing::info!("base64_img: {}", base64_img);
        let ai_tags = ai_service.generate_tags(&base64_img).await.unwrap_or_default();
        if !ai_tags.is_empty() {
            final_keywords = format!("{} {}", final_keywords, ai_tags.join(" "));
        }
    }
    tracing::info!("ai tags: {:?}", ai_tags);

    // Build Acme Science query parameters
    let mut params = vec![];

    if !final_keywords.is_empty() { params.push(("phrase", final_keywords.clone())); }
    if let Some(lat) = latitude.clone() { params.push(("lat", lat)); }
    if let Some(lng) = longitude.clone() { params.push(("lng", lng)); }
    if let Some(e) = events.clone() { params.push(("activity", e)); }
    if let Some(p) = projects.clone() { params.push(("topic", p)); }
    if let Some(a) = active.clone() { params.push(("active", a)); }
    if let Some(u) = upcoming.clone() { params.push(("upcoming", u)); }

    // Get base URL from environment
    let api_url = std::env::var("PROJECTS_API_URL").unwrap_or_else(|_| "https://api.acme-science.example/projects?format=json&key=YOUR_KEY".to_string());
    let mut url = Url::parse(&api_url).unwrap();
    url.query_pairs_mut().extend_pairs(params);

    tracing::info!("Final Acme Science URL: {}", url);

    // Send request to Acme Science
    let client = reqwest::Client::new();
    let resp = client.get(url.clone()).send().await.map_err(poem::error::InternalServerError)?;
    let body = resp.text().await.map_err(poem::error::InternalServerError)?;

    // Stub: parse JSON or just return placeholder results
    let results = vec![
        format!("Result 1 for '{}'", final_keywords),
        format!("Result 2 for '{}'", final_keywords),
        format!("Result 3 for '{}'", final_keywords),
    ];
    tracing::info!("resp: {}", body);
    Ok(Html(
        SearchResultsTemplate {
            query: final_keywords,
            results,
            ai_tags,
            projects_url: url.to_string(),
        }
        .render_once()
        .map_err(poem::error::InternalServerError)?,
    ))
}