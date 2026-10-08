use poem::{
    handler,
    web::{CsrfToken, Data, Html, Json, Multipart, Query},
    Result,
};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::render::render_with_layout;
use crate::services::ai::AiService;
use crate::services::projects::{PAGE_SIZE, ProjectQuery, ProjectsService};
use crate::utils::image::encode_image;

// ===== Form Input =====

#[derive(Debug, Deserialize)]
pub struct SearchForm {
    pub keywords: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub page: Option<usize>, // for pagination
}

// ===== UI Model =====

pub struct ProjectCard {
    pub title: String,
    pub link: String,
    pub goal: String,
    pub task: String,
    pub image: String,
}

// ===== Templates =====

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
    next: Option<usize>
}

// ===== Handlers =====

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
pub async fn search_results(
    Query(form): Query<SearchForm>,
    projects_service: Data<&Arc<dyn ProjectsService>>,
) -> Result<Html<String>> {
    let query = ProjectQuery {
        keywords: form.keywords,
        latitude: form.latitude,
        longitude: form.longitude,
        page: form.page.unwrap_or(1),
    };

    let data = projects_service.search(&query).await?;

    // Set Page #
    let total = data.total.unwrap_or(0);
    let page = data.page.unwrap_or(1);
    let per_page = data.entities.len();

    let fetched_so_far = (page - 1) * PAGE_SIZE + per_page;
    let has_more = fetched_so_far < total;

    let next_page = if has_more { Some(page + 1) } else { None };

    // Map API → UI model
    let base_url = std::env::var("PROJECTS_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let cards: Vec<ProjectCard> = data
        .entities
        .into_iter()
        .map(|p| ProjectCard {
            title: p.name.unwrap_or_else(|| "Untitled Project".into()),
            goal: p.goal.unwrap_or_else(|| "No goal listed.".into()),
            task: p.task.unwrap_or_else(|| "No task listed.".into()),
            image: p.image.unwrap_or_else(|| "/static/placeholder.svg".into()),
            link: match p.link {
                Some(ref link) if link.starts_with("http") => link.clone(),
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

    // Render template

    let body_html = SearchResultsTemplate { projects: &cards, next: next_page }
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
    let mut location_str = None;

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image") => image_bytes = Some(field.bytes().await?.to_vec()),
            Some("latitude") => latitude = field.text().await?.parse().ok(),
            Some("longitude") => longitude = field.text().await?.parse().ok(),
            Some("location") => location_str = Some(field.text().await?.to_string()),
            _ => {}
        }
    }
    // println!("DEBUG: latitude={:?}, longitude={:?}, location_str={:?}", latitude, longitude, location_str);

    let image = image_bytes.ok_or_else(|| {
        poem::Error::from_string("image is required", poem::http::StatusCode::BAD_REQUEST)
    })?;

    // If location string provided but no coords, geocode it
    if location_str.is_some() && latitude.is_none() && longitude.is_none() {
        if let Some(coords) = geocode_location(&location_str.unwrap()).await {
            latitude = Some(coords.0);
            longitude = Some(coords.1);
        }
    }

    let image_base64 = encode_image(&image);
    let keywords = ai_service.generate_tags(&image_base64).await?;
    // let keywords = vec!["dogs", "mammals", "wolves"]
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect();
    
    // println!("DEBUG: Returning - keywords={:?}, latitude={:?}, longitude={:?}", keywords, latitude, longitude);

    Ok(Json(ImageSearchResponse {
        keywords,
        latitude,
        longitude,
    }))
}

async fn geocode_location(location: &str) -> Option<(f64, f64)> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
        urlencoding::encode(location)
    );

    // println!("DEBUG: Geocoding '{}' with URL: {}", location, url);

    let response = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", "AcmeScience-App/1.0")
        .send()
        .await
        .ok()?;

    // println!("DEBUG: Got response status: {}", response.status());

    let data: Vec<serde_json::Value> = response.json().await.ok()?;
    
    // println!("DEBUG: Parsed JSON: {:?}", data);
    
    if let Some(result) = data.first() {
        let lat = result.get("lat")?.as_str()?.parse::<f64>().ok()?;
        let lon = result.get("lon")?.as_str()?.parse::<f64>().ok()?;
        // println!("DEBUG: Found lat={}, lon={}", lat, lon);
        return Some((lat, lon));
    }

    // println!("DEBUG: No results in geocoding response");
    None
}