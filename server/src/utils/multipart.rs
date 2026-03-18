use poem::web::Multipart;

async fn extract_image(mut multipart: Multipart) -> anyhow::Result<Vec<u8>> {

    while let Some(field) = multipart.next_field().await? {

        if field.name() == Some("file") {
            let bytes = field.bytes().await?;
            return Ok(bytes.to_vec());
        }
    }

    anyhow::bail!("No file found")
}