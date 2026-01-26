// TODO: stub these
// [X] form exists
// [ ] CSRF Works
// [ ] POST handler exists
// [ ] pretend success -> redirect to /search

use poem::{
  handler,
  Result,
  web::{
    Html, CsrfToken, Redirect, Form
  }
};
use sailfish::TemplateOnce;
use serde::Deserialize;

use crate::render::render_with_layout;

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

#[derive(TemplateOnce)]
#[template(path = "login_form.stpl.html")]
struct LogInFormTemplate {
    csrf_token: String,
    message: Option<String>,
}

#[handler]
pub async fn login_page(token: &CsrfToken) -> Result<Html<String>> {
  render_with_layout(
    "Log In",
    LogInFormTemplate {
      csrf_token: token.0.clone(),
      message: None,
    },
  )
}

#[handler]
pub async fn login_post(_form: Form<LoginForm>) -> Result<Redirect> {
    // TODO:
    // - validate
    // - check DB
    // - set session cookie

    Ok(Redirect::see_other("/search"))
}