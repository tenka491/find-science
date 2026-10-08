# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository layout

The repo holds two apps that work together:

- `server/` is a Rust web server (Poem, Sailfish templates, HTMX, Tailwind v4, SQLite via SQLx). It renders all UI as server-side HTML.
- `mobile/` is a Flutter app that wraps the server's pages in a WebView. It adds native camera, image picking, and location on top.
- `scripts/` has helpers to run a local Ollama container for AI image tagging.

## Commands

### Server (run from `server/`)

```bash
cargo install sqlx-cli            # one time
export DATABASE_URL=sqlite://data.db
touch data.db
sqlx migrate run                  # or: cargo sqlx database setup
cargo run                         # serves http://127.0.0.1:3000
cargo watch -x run                # auto-reload
cargo check
cargo test                        # all tests
cargo test test_encode_image      # single test by name
./build.sh                        # regenerate static/style.css with the standalone tailwindcss CLI
```

SQLx checks `query!` macros against a live database at compile time. Builds fail if `DATABASE_URL` is unset or `data.db` lacks the current migrations. There is no `.sqlx` offline cache in the repo. Add new migrations to `server/migrations/`. `main.rs` also runs `sqlx::migrate!()` on startup.

After editing any `.stpl.html` template, run `./build.sh` so Tailwind picks up new classes. `static/style.css` is generated and committed. Do not edit it by hand. Edit `styles/input.css` instead.

### Mobile (run from `mobile/`)

```bash
flutter run                                          # server must already be running
flutter run -d "iPhone 16e"
flutter run --dart-define=DEV_BYPASS_PERMISSIONS=true  # skip camera/location prompts
flutter analyze
flutter test
```

### AI (optional)

`./scripts/dev-ai` starts an `ollama/ollama` Docker container on port 11434 and pulls `llava`. `./scripts/dev-ai-stop` stops it.

## Architecture

### Server request flow

- `main.rs` builds the route table, holds a global SQLite pool (`db()`), and wraps everything in `Csrf` and `Tracing` middleware. Static files in `static/` are embedded into the binary with `rust-embed` and served at `/static`.
- Each page handler builds a `#[derive(TemplateOnce)]` struct pointing at a file in `templates/`. Full pages go through `render::render_with_layout`, which wraps content in `layout.stpl.html`. HTMX partials (for example `search_results.stpl.html`) are rendered directly with `render_once()` and returned without the layout.
- Forms embed the CSRF token from the `&CsrfToken` extractor.
- Auth: `routes/signup.rs` and `routes/login.rs` hash passwords with Argon2 (`auth/password.rs`). Login inserts a row into `sessions` (`auth/session.rs`) and sets a `session_id` cookie. No middleware checks the session yet, so `/search` is not protected.
- `src/examples.rs` and `templates/examples/` are reference code for HTMX, SSE, uploads, and sessions. `examples.rs` is not declared as a module, so it does not compile into the binary.

### Search

- `GET /search/results` (`routes/search.rs`) calls the `ProjectsService` trait (`services/projects/`). It maps results into `ProjectCard` and returns an HTMX partial. The "load more" button uses the returned `next` page number. Page size is `PAGE_SIZE` (10).
- `main.rs` picks the `ProjectsService` at startup. If `PROJECTS_API_URL` is set, it uses `HttpProjectsService`, which sends `q`, `lat`, `lng`, and `page` to that URL. Otherwise it uses `MockProjectsService`, which serves 12 sample projects and matches keywords against name, goal, and task. Relative project links are prefixed with `PROJECTS_BASE_URL`. Absolute links pass through unchanged.
- `POST /search/image` takes multipart fields `image`, `latitude`, `longitude`, and `location`. It geocodes a free-text location through OpenStreetMap Nominatim when no coordinates are given. It sends the base64 image to the `AiService` trait (`services/ai/`) and returns JSON `{keywords, latitude, longitude}`.
- `AiService` has two implementations: `OllamaClient` (calls `/api/generate`, splits the reply on commas) and `MockAiService` (fixed tags, used in tests). `main.rs` always builds `OllamaClient`. It reads `OLLAMA_URL` and `OLLAMA_MODEL`. It does not read `AI_PROVIDER` or `OLLAMA_BASE_URL`, even though `.env` and `scripts/README.md` mention them.

### Mobile and server bridge

- `config.dart` picks `baseUrl`: `http://localhost:3000` in debug, `https://acme-science.example` in release. Use `localhost`, not `127.0.0.1`, because the server binds to `127.0.0.1` and the iOS simulator needs `localhost`.
- `webview_page.dart` loads the server in a WebView. When the page is `/search` and `document.cookie` contains `session_id`, it asks for camera and location permissions (`permissions_gate.dart`).
- `validation_image_screen.dart` posts the photo and location to `/search/image` with the Dart `http` package, outside the WebView. It then calls `window.populateSearchResults(<json>)` inside the WebView. That JS function lives in `server/templates/search.stpl.html`, so changes to the JSON shape or function name must be made on both sides.
- `theme.dart` mirrors the Tailwind colors so native screens match the web UI.

## Branding

This repo is white-labelled. The placeholder company is "Acme Science" (Dart package `acme_science`, domain `acme-science.example`), and the product is "Find Science". The brand palette is defined twice and must stay in sync: `server/styles/input.css` (`@theme` variables) and `mobile/lib/theme.dart` (`AppColors`). Logos live in `mobile/assets/images/` and `mobile/assets/icons/molecule.svg`.

## Environment

`server/.env` is loaded by `dotenvy` and is tracked in git. Keys: `DATABASE_URL`, `AI_PROVIDER`, `OLLAMA_BASE_URL`, `OLLAMA_MODEL`, and the commented-out `PROJECTS_API_KEY`, `PROJECTS_BASE_URL`, `PROJECTS_API_URL`. Note that `main.rs` connects to the hardcoded `sqlite:data.db` path, not `DATABASE_URL`.
