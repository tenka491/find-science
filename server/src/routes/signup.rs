// TODO: stub these
// [x] form exists
// [x] POST Handler exists -> redirect to signup success
// [ ] DB insert
// [ ] DB migration

use poem::{
  handler,
  Result,
  web::{
    Html, CsrfToken,
    Form, Redirect,
  }
};
use sailfish::TemplateOnce;
use serde::Deserialize;

use crate::render::render_with_layout;

#[derive(Deserialize)]
struct SignUpform {
  email: String,
  password: String,
}

#[derive(TemplateOnce)]
#[template(path = "signup_form.stpl.html")]
struct SignUpFormTemplate {
    csrf_token: String,
    message: Option<String>,
}

#[handler]
pub async fn signup_page(token: &CsrfToken) -> Result<Html<String>> {
  render_with_layout(
    "Sign up form",
    SignUpFormTemplate {
      csrf_token: token.0.clone(),
      message: None,
    },
  )
}

#[handler]
pub async fn signup_post(_form: Form<SignUpform>) -> Result<Redirect>{
  // TODO: 
  // - validate
  // - Insert into DB
  // - Hash Password

  Ok(Redirect::see_other("signup/success"))
}

