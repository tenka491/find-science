use poem::{
    handler,
    web::{CsrfToken, Data, Form, Html, Json, Multipart},
    Result,
};
use reqwest::Url;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::render::render_with_layout;
use crate::services::ai::AiService;
use crate::utils::image::encode_image;

//
// ===== Form Input =====
//

#[derive(Debug, Deserialize)]
pub struct SearchForm {
    pub keywords: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

//
// ===== API Response =====
//

#[derive(Debug, Deserialize)]
pub struct ProjectsApiResponse {
    pub entities: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: Option<String>,
    pub link: Option<String>,
    pub goal: Option<String>,
    pub task: Option<String>,
    pub image: Option<String>,
}

//
// ===== UI Model =====
//

pub struct ProjectCard {
    pub title: String,
    pub link: String,
    pub goal: String,
    pub task: String,
    pub image: String,
    // pub full_link: String,
}

//
// ===== Templates =====
//

#[derive(TemplateOnce)]
#[template(path = "search.stpl.html")]
struct SearchPageTemplate {
    csrf_token: String,
    message: Option<String>,
}

#[derive(TemplateOnce)]
#[template(path = "search_results.stpl.html")]
struct SearchResultsTemplate<'a> {
    projects: &'a [ProjectCard],
}

//
// ===== Handlers =====
//

#[handler]
pub async fn search_page(token: &CsrfToken) -> Result<Html<String>> {
    render_with_layout(
        "Search",
        SearchPageTemplate {
            csrf_token: token.0.clone(),
            message: None,
        },
    )
}

#[handler]
pub async fn search_results(Form(form): Form<SearchForm>) -> Result<Html<String>> {
    // Build query

    let api_url = std::env::var("PROJECTS_API_URL")
        .unwrap_or_else(|_| "https://api.acme-science.example/projects?format=json".to_string());

    let mut url = Url::parse(&api_url).unwrap();

    if let Some(lat) = &form.latitude {
        if !lat.is_empty() {
            url.query_pairs_mut().append_pair("lat", lat);
        }
    }

    if let Some(lng) = &form.longitude {
        if !lng.is_empty() {
            url.query_pairs_mut().append_pair("lng", lng);
        }
    }
   
    if let Some(keywords) = &form.keywords {
        if !keywords.is_empty() {
            url.query_pairs_mut().append_pair("q", keywords);
        }
    }

    tracing::info!("Acme Science URL: {}", url);

    // Call API

    let resp = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .map_err(poem::error::InternalServerError)?;

    let data: ProjectsApiResponse = resp
        .json()
        .await
        .map_err(poem::error::InternalServerError)?;

    //
    // Map API → UI model
    //
    let base_url = std::env::var("PROJECTS_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let cards: Vec<ProjectCard> = data
        .entities
        .into_iter()
        .map(|p| ProjectCard {
            title: p.name.unwrap_or_else(|| "Untitled Project".into()),
            // link: p.link.unwrap_or_else(|| "#".into()),
            goal: p.goal.unwrap_or_else(|| "No goal listed.".into()),
            task: p.task.unwrap_or_else(|| "No task listed.".into()),
            image: p.image.unwrap_or_else(|| "/static/placeholder.jpg".into()),
            link: match p.link {
                Some(ref link) => {
                    format!(
                        "{}/{}",
                        base_url.trim_end_matches('/'),
                        link.trim_start_matches('/')
                    )
                }
                None => "#".to_string(),
            }
        })
        .collect();

    //
    // Render template
    //

    let body_html = SearchResultsTemplate { projects: &cards }
        .render_once()
        .map_err(poem::error::InternalServerError)?;

    Ok(Html(body_html))
}

//
// ===== Image Search =====
//

#[derive(Serialize)]
struct ImageSearchResponse {
    keywords: Vec<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

#[handler]
pub async fn search_image(
    mut multipart: Multipart,
    ai_service: Data<&Arc<dyn AiService>>,
) -> Result<Json<ImageSearchResponse>> {
    let mut image_bytes = None;
    let mut latitude = None;
    let mut longitude = None;

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image") => image_bytes = Some(field.bytes().await?.to_vec()),
            Some("latitude") => latitude = field.text().await?.parse().ok(),
            Some("longitude") => longitude = field.text().await?.parse().ok(),
            _ => {}
        }
    }

    let image = image_bytes.ok_or_else(|| {
        poem::Error::from_string("image is required", poem::http::StatusCode::BAD_REQUEST)
    })?;

    let image_base64 = encode_image(&image);
    let keywords = ai_service.generate_tags(&image_base64).await?;

    Ok(Json(ImageSearchResponse {
        keywords,
        latitude,
        longitude,
    }))
}