mod routes;
mod render;
mod auth;

use std::sync::OnceLock;


use poem::{
    EndpointExt, Route, Server,
    endpoint::EmbeddedFilesEndpoint,
    get,
    listener::TcpListener,
    middleware::{Csrf, Tracing},
    post,
    
};
use sqlx::{Pool, Sqlite, SqlitePool};
use tracing::info;

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



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite:data.db").await?;
    sqlx::migrate!().run(&pool).await?;
    DB_POOL.set(pool.clone()).expect("database pool already initialized");

    let app = Route::new()
        // .at("/", get(index))
        // .at("/logout", post(logout))
        // .at("/form", get(form_get).post(form_post))
        // .at("/upload", get(upload_get).post(upload_post))
        // .at("/events", get(events_page))
        // .at("/sse", get(sse_stream))
        .at("/", get(login::login_page).post(login::login_post).data(pool.clone()))
        .at("/login", get(login::login_page).post(login::login_post).data(pool.clone()))
        .at("/signup", get(signup::signup_page).post(signup::signup_post).data(pool.clone()))
        .at("/signup/success", get(signup_success::signup_success))
        .at("/search", get(search::search_page))
        .at("/search/results", post(search::search_results))
        .at("/api/analyze-image", post(llm::analyze_image))
        .nest("/static", EmbeddedFilesEndpoint::<StaticFiles>::new())
        .with(Csrf::new())
        // .with(ServerSession::new(
        //     poem::session::CookieConfig::default(),
        //     SqliteSessionStorage,
        // ))
        .with(Tracing);

    info!("Starting server at http://127.0.0.1:3000");

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}
