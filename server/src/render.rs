use poem::web::Html;
use sailfish::TemplateOnce;
// use if you want to create an RS file to reference the template file
// use crate::templates::layout::LayoutTemplate;

#[derive(TemplateOnce)]
#[template(path = "layout.stpl.html")]
struct LayoutTemplate<C: TemplateOnce> {
    title: String,
    content: C,
}

/// Renders a full HTML page using the shared layout
pub fn render_with_layout(
    title: impl Into<String>,
    content: impl TemplateOnce,
) -> poem::Result<Html<String>> {
    let layout = LayoutTemplate {
        title: title.into(),
        content,
    };

    Ok(Html(
        layout
            .render_once()
            .map_err(poem::error::InternalServerError)?,
    ))
}
