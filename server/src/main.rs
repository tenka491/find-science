mod routes;
mod render;
use std::sync::OnceLock;
use std::time::Duration;

use poem::web::sse::{Event, SSE};
use poem::{
    EndpointExt, IntoResponse, Response, Result, Route, Server,
    endpoint::EmbeddedFilesEndpoint,
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    middleware::{Csrf, Tracing},
    post,
    session::{ServerSession, Session, SessionStorage},
    web::{
        CsrfToken, CsrfVerifier, Form, Html, Multipart,
        cookie::{Cookie, CookieJar},
    },
};
use sailfish::TemplateOnce;
use sqlx::{Pool, Sqlite, SqlitePool};
use tracing::info;

use crate::render::render_with_layout;
use routes::signup;
use routes::signup_success;
use routes::login;
use routes::search;
use routes::llm;

#[derive(rust_embed::RustEmbed)]
#[folder = "static"]
struct StaticFiles;

// Database

static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub fn db() -> &'static Pool<Sqlite> {
    DB_POOL.get().expect("database pool not initialized")
}

// SQLite session storage

#[derive(Clone, Copy)]
struct SqliteSessionStorage;

impl SessionStorage for SqliteSessionStorage {
    async fn load_session(
        &self,
        session_id: &str,
    ) -> poem::Result<Option<std::collections::BTreeMap<String, serde_json::Value>>> {
        let row = sqlx::query!(
            "SELECT data FROM sessions WHERE id = ? AND (expires_at IS NULL OR expires_at > unixepoch())",
            session_id
        )
        .fetch_optional(db())
        .await
        .map_err(poem::error::InternalServerError)?;

        match row {
            Some(row) => {
                let map = serde_json::from_str(&row.data)
                    .map_err(poem::error::InternalServerError)?;
                Ok(Some(map))
            }
            None => Ok(None),
        }
    }

    async fn update_session(
        &self,
        session_id: &str,
        entries: &std::collections::BTreeMap<String, serde_json::Value>,
        expires: Option<std::time::Duration>,
    ) -> poem::Result<()> {
        let data = serde_json::to_string(entries).map_err(poem::error::InternalServerError)?;
        let expires_at = expires.map(|d| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                + d.as_secs() as i64
        });

        sqlx::query!(
            "INSERT INTO sessions (id, data, expires_at) VALUES (?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET data = excluded.data, expires_at = excluded.expires_at",
            session_id,
            data,
            expires_at
        )
        .execute(db())
        .await
        .map_err(poem::error::InternalServerError)?;

        Ok(())
    }

    async fn remove_session(&self, session_id: &str) -> poem::Result<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = ?", session_id)
            .execute(db())
            .await
            .map_err(poem::error::InternalServerError)?;

        Ok(())
    }
}

// Template structures

#[derive(TemplateOnce)]
#[template(path = "examples/index.stpl.html")]
struct IndexTemplate {
    visit_count: u32,
    session: SessionPartialTemplate,
}

#[derive(TemplateOnce)]
#[template(path = "examples/form.stpl.html")]
struct FormTemplate {
    csrf_token: String,
    message: Option<String>,
}

#[derive(TemplateOnce)]
#[template(path = "examples/upload.stpl.html")]
struct UploadTemplate {
    file_info: Option<FileInfo>,
}

struct FileInfo {
    name: String,
    size: usize,
    content_type: String,
}

#[derive(TemplateOnce)]
#[template(path = "examples/events.stpl.html")]
struct EventsTemplate;

#[derive(TemplateOnce)]
#[template(path = "examples/session_partial.stpl.html")]
struct SessionPartialTemplate {
    username: String,
}

// Handlers

#[handler]
async fn index(cookies: &CookieJar, session: &Session) -> Result<Html<String>> {
    let visit_count: u32 = cookies
        .get("visits")
        .and_then(|c| c.value_str().parse().ok())
        .unwrap_or(0)
        + 1;

    cookies.add(Cookie::new_with_str("visits", visit_count.to_string()));

    let username = session.get::<String>("username").unwrap_or_default();

    info!(visit_count, %username, "Index page visited");

    render_with_layout(
        "Home - Find Science",
        IndexTemplate {
            visit_count,
            session: SessionPartialTemplate { username },
        },
    )
}

// #[derive(serde::Deserialize)]
// struct LoginForm {
//     username: String,
// }

// #[handler]
// async fn login(session: &Session, Form(form): Form<LoginForm>) -> Result<Html<String>> {
//     session.set("username", form.username.clone());
//     let content = SessionPartialTemplate {
//         username: form.username,
//     }
//     .render_once()
//     .map_err(poem::error::InternalServerError)?;
//     Ok(Html(content))
// }

#[handler]
async fn logout(session: &Session) -> Result<Html<String>> {
    session.remove("username");
    let content = SessionPartialTemplate {
        username: String::new(),
    }
    .render_once()
    .map_err(poem::error::InternalServerError)?;
    Ok(Html(content))
}

#[handler]
async fn form_get(token: &CsrfToken) -> Result<Html<String>> {
    render_with_layout(
        "Form Demo - Find Science",
        FormTemplate {
            csrf_token: token.0.clone(),
            message: None,
        },
    )
}

#[derive(serde::Deserialize)]
struct MessageForm {
    csrf_token: String,
    message: String,
}

#[handler]
async fn form_post(
    token: &CsrfToken,
    verifier: &CsrfVerifier,
    Form(form): Form<MessageForm>,
) -> Result<impl IntoResponse> {
    if !verifier.is_valid(&form.csrf_token) {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body("Invalid CSRF token"));
    }

    info!(message = %form.message, "Form submitted");

    render_with_layout(
        "Form Demo - Find Science",
        FormTemplate {
            csrf_token: token.0.clone(),
            message: Some(form.message),
        },
    )
    .map(IntoResponse::into_response)
}

#[handler]
async fn upload_get() -> Result<Html<String>> {
    render_with_layout(
        "Upload Demo - Find Science",
        UploadTemplate { file_info: None },
    )
}

#[handler]
async fn upload_post(mut multipart: Multipart) -> Result<Html<String>> {
    let mut file_info = None;

    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some("file") {
            let filename = field.file_name().map(String::from).unwrap_or_default();
            let content_type = field.content_type().map(String::from).unwrap_or_default();
            let data = field.bytes().await?;

            info!(
                filename = %filename,
                size = data.len(),
                content_type = %content_type,
                "File uploaded"
            );

            file_info = Some(FileInfo {
                name: filename,
                size: data.len(),
                content_type,
            });
        }
    }

    render_with_layout("Upload Demo - Find Science", UploadTemplate { file_info })
}

#[handler]
async fn events_page() -> Result<Html<String>> {
    render_with_layout("SSE Demo - Find Science", EventsTemplate)
}

const TOTAL_TICKS: u32 = 10;

#[handler]
fn sse_stream() -> SSE {
    let stream = async_stream::stream! {
        // Reset all elements on stream start
        yield Event::message(
            r#"<div class="h-full bg-blue-600 transition-all duration-300" style="width: 0%"></div>"#
        ).event_type("progress");
        yield Event::message(format!(
            r#"<span class="text-2xl font-bold text-blue-600">0</span> / {}"#,
            TOTAL_TICKS
        )).event_type("counter");
        yield Event::message("").event_type("log");
        yield Event::message(
            r#"<span class="text-blue-500">Streaming...</span>"#
        ).event_type("status");

        for i in 1..=TOTAL_TICKS {
            tokio::time::sleep(Duration::from_secs(1)).await;

            // Update the progress bar
            let percent = i * 100 / TOTAL_TICKS;
            yield Event::message(format!(
                r#"<div class="h-full bg-blue-600 transition-all duration-300" style="width: {}%"></div>"#,
                percent
            )).event_type("progress");

            // Update the counter display
            yield Event::message(format!(
                r#"<span class="text-2xl font-bold text-blue-600">{}</span> / {}"#,
                i, TOTAL_TICKS
            )).event_type("counter");

            // Append to the event log
            yield Event::message(format!(
                r#"<div class="py-1 border-b border-gray-700">Tick {} received at {}% complete</div>"#,
                i, percent
            )).event_type("log");
        }

        // Send completion event
        yield Event::message(
            r#"<span class="text-green-500 font-semibold">Stream complete!</span>"#
        ).event_type("status");
    };

    SSE::new(stream)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite:data.db").await?;
    sqlx::migrate!().run(&pool).await?;
    DB_POOL.set(pool).expect("database pool already initialized");

    let app = Route::new()
        .at("/", get(index))
        .at("/logout", post(logout))
        .at("/form", get(form_get).post(form_post))
        .at("/upload", get(upload_get).post(upload_post))
        .at("/events", get(events_page))
        .at("/sse", get(sse_stream))
        .at("/login", get(login::login_page).post(login::login_post))
        .at("/signup", get(signup::signup_page).post(signup::signup_post))
        .at("/signup/success", get(signup_success::signup_success))
        .at("/search", get(search::search_page))
        .at("/search/results", post(search::search_results))
        .at("/api/analyze-image", post(llm::analyze_image))
        .nest("/static", EmbeddedFilesEndpoint::<StaticFiles>::new())
        .with(Csrf::new())
        .with(ServerSession::new(
            poem::session::CookieConfig::default(),
            SqliteSessionStorage,
        ))
        .with(Tracing);

    info!("Starting server at http://127.0.0.1:3000");

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}
