use poem::{handler, web::{Multipart, Data}, Result};
use std::sync::Arc;

use crate::utils::multipart::extract_image;
use crate::utils::image::encode_image;
use crate::services::ai::AiService;

#[handler]
pub async fn test_ai(
    multipart: Multipart,
    ai: Data<&Arc<dyn AiService>>,
) -> Result<String> {

    let bytes = extract_image(multipart).await.unwrap();

    let base64 = encode_image(&bytes);

    let tags = ai.generate_tags(&base64).await.unwrap();

    Ok(tags.join(", "))
}