// TODO: stub these
// [X] form exists
// [ ] CSRF Works
// [x] POST handler exists
// [x] pretend success -> redirect to /search

use poem::{
  Error, http::StatusCode,
  handler,
  Result,
  web::{
    Html, CsrfToken, Data, Redirect, Form,
    cookie:: {Cookie, CookieJar},
  },

};
use sailfish::TemplateOnce;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::auth::password::verify_password;
use crate::render::render_with_layout;
use crate::auth::session::create_session;

#[derive(Deserialize)]
pub struct LoginFormData {
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
pub async fn login_post(
    Data(pool): Data<&SqlitePool>,
    jar: &CookieJar,
    form: Form<LoginFormData>,
) -> Result<Redirect> {
    let data = form.0;

    // 1. fetch user
    let user = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM users
        WHERE email = ?
        "#,
        data.email
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch user: {:?}", e);
        Error::from_string(
            "failed to fetch user",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    // 2. if user not found
    let user = match user {
        Some(u) => u,
        None => {
            return Ok(Redirect::see_other("/login?error=invalid"));
        }
    };

    // 3. verify password
    let valid = verify_password(&user.password_hash, &data.password)
        .map_err(|e| {
            tracing::error!("pass: {:?}", e);
            Error::from_string(
                "last_login_at failed to update",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    if !valid {
        return Ok(Redirect::see_other("/login?error=invalid"));
    }
    // 4.1. Update session table and added Cookie, yum.
    let session_id = create_session(pool, user.id)
    .await
    .map_err(|e| {
        tracing::error!("Session failed to update: {:?}", e);
        Error::from_string(
            "Session failed to update",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;
    jar.add(
        Cookie::new("session_id", session_id)
    );

    // 4.2. update last login
    sqlx::query!(
        r#"
        UPDATE users
        SET last_login_at = strftime('%s','now')
        WHERE id = ?
        "#,
        user.id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("last_login_at failed to update: {:?}", e);
        Error::from_string(
            "last_login_at failed to update",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    // 5. redirect (session comes later)
    Ok(Redirect::see_other("/search"))
}