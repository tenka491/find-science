use poem::{
    Error, http::StatusCode,
    handler,
    Result,
    web::{Html, CsrfToken, CsrfVerifier, Form, Redirect, Data},
};
use sailfish::TemplateOnce;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::auth::password::hash_password;
use crate::render::render_with_layout;

#[derive(TemplateOnce)]
#[template(path = "signup_form.stpl.html")]
struct SignUpFormTemplate {
    csrf_token: String,
    message: Option<String>,
}

#[derive(Deserialize)]
pub struct SignUpData {
    username: String,
    email: String,
    zip: String,
    password: String,
    confirm_password: String,
    csrf_token: String,
}

#[handler]
pub async fn signup_page(token: &CsrfToken) -> Result<Html<String>> {
    render_with_layout(
        "Sign up",
        SignUpFormTemplate {
            csrf_token: token.0.clone(),
            message: None,
        },
    )
}

#[handler]
pub async fn signup_post(
    Data(pool): Data<&SqlitePool>,
    verifier: &CsrfVerifier,
    Form(data): Form<SignUpData>,
) -> Result<Redirect> {
    if !verifier.is_valid(&data.csrf_token) {
        return Err(Error::from_string(
            "Invalid CSRF token",
            StatusCode::FORBIDDEN,
        ));
    }

    // Validate input (business logic errors → BadRequest)
    if data.password != data.confirm_password {
        return Ok(Redirect::see_other("/signup?error=password_mismatch"));
    }

    // Hash password (internal error → 500)
    let hashed = hash_password(&data.password)
        .map_err(|e| {
            tracing::error!("password hash failed: {:?}", e);
            Error::from_string(
                "password hash failed",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    // Insert user (internal error → 500)
    sqlx::query!(
        r#"
        INSERT INTO users (username, email, zip, password_hash)
        VALUES (?, ?, ?, ?)
        "#,
        data.username,
        data.email,
        data.zip,
        hashed,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("db insert failed: {:?}", e);
        Error::from_string(
            "database error",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    // Success redirect
    Ok(Redirect::see_other("/signup/success"))
}
