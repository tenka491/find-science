use poem::{handler, web::Html};
use sailfish::TemplateOnce;
use crate::render::render_with_layout;

#[derive(TemplateOnce)]
#[template(path = "signup_success.stpl.html")]
struct SignupSuccessTemplate;

#[handler]
pub async fn signup_success() -> poem::Result<Html<String>> {
    render_with_layout(
        "Signup Complete",
        SignupSuccessTemplate,
    )
}
