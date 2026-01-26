use poem::{handler, web::Json, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct ImageAnalysisRequest {
    image_url: String,
}

#[derive(serde::Serialize)]
struct ImageAnalysisResponse {
    description: String,
    tags: Vec<String>,
}

#[handler]
pub async fn analyze_image(
    _req: Json<ImageAnalysisRequest>,
) -> Result<Json<ImageAnalysisResponse>> {
    // TODO: connect to your LLM/image analysis pipeline here
    Ok(Json(ImageAnalysisResponse {
        description: "Stub description from LLM".into(),
        tags: vec!["tag1".into(), "tag2".into()],
    }))
}
