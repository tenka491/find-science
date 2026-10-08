# Find Science

Find Science helps people discover citizen science projects. A user searches by keyword and location, or takes a photo. An AI model turns the photo into search keywords.

The company name "Acme Science" and the `acme-science.example` domain are placeholders.

## Architecture

The repo holds two apps and some helper scripts.

| Folder | What it is |
| --- | --- |
| `server/` | Rust web server. It renders every page as HTML on the server. |
| `mobile/` | Flutter app. It shows the server's pages in a WebView and adds the camera and location. |
| `scripts/` | Scripts that start and stop a local Ollama container for AI image tagging. |

### Server

- Built with [Poem](https://github.com/poem-web/poem). Pages are [Sailfish](https://github.com/rust-sailfish/sailfish) templates in `server/templates/`.
- [HTMX](https://htmx.org) handles partial page updates, such as loading search results and the "Load More" button. The app has no client-side framework.
- Styles use Tailwind v4. `server/styles/input.css` compiles to `server/static/style.css`.
- Data is stored in SQLite and accessed through SQLx. SQLx checks SQL queries at compile time, so the server needs a migrated database before it compiles.
- Project search goes through a `ProjectsService` trait. If `PROJECTS_API_URL` is unset, the server uses built-in mock data, so search works with no external API.
- Photo search goes through an `AiService` trait. `AI_PROVIDER` picks the implementation. With `ollama`, the server sends the image to a local Ollama model (`llava`), which returns keywords. With `mock` (the default), it returns fixed keywords so the flow runs without Docker.

### Mobile app

- The app loads the server in a WebView. It uses `http://localhost:3000` in debug builds and `https://acme-science.example` in release builds (see `mobile/lib/config.dart`).
- Native screens handle the camera and the location form. The app uploads the photo to `POST /search/image`. It then passes the result into the page by calling `window.populateSearchResults(...)`.

### Request flow for a photo search

1. The user takes a photo in the mobile app.
2. The app posts the photo and location to `/search/image`.
3. The server asks Ollama for keywords and geocodes the location if needed.
4. The app fills the search form in the WebView with the keywords and coordinates.
5. HTMX requests `/search/results` and shows the matching projects.

## Getting started

### Requirements

- Rust (stable) and `sqlx-cli`: `cargo install sqlx-cli`
- Flutter 3.38.7 or newer, for the mobile app
- Docker, only for AI photo search
- The standalone `tailwindcss` CLI, only if you change styles

### 1. Start the server

```bash
cd server
export DATABASE_URL=sqlite://data.db
touch data.db
sqlx migrate run
cargo run
```

The server runs at http://localhost:3000. Sign up for an account, then log in to reach the search page.

To reload the server on every code change, run `cargo watch -x run`.

### 2. Turn on AI photo tagging (optional)

By default the server uses mock AI tags, so photo search returns the same keywords for every photo. To tag photos with a real model:

1. Start Ollama. The first run downloads the `llava` model, which takes a while.

   ```bash
   ./scripts/dev-ai        # start Ollama and pull the llava model
   ./scripts/dev-ai-stop   # stop it later
   ```

2. In `server/.env`, set `AI_PROVIDER=ollama`.
3. Restart the server. The startup log shows `AI: using Ollama at http://localhost:11434 with model llava`.

### 3. Start the mobile app

Start the server first. Otherwise the WebView loads a blank page.

```bash
cd mobile
flutter run
```

To skip the camera and location permission prompts during development, run:

```bash
flutter run --dart-define=DEV_BYPASS_PERMISSIONS=true
```

## Configuration

The server reads `server/.env` at startup.

| Variable | Purpose |
| --- | --- |
| `DATABASE_URL` | SQLite database used by SQLx at compile time and by `sqlx-cli`. |
| `AI_PROVIDER` | `mock` (default) or `ollama`. Any other value stops the server at startup. |
| `OLLAMA_BASE_URL` | Ollama address. Defaults to `http://localhost:11434`. |
| `OLLAMA_MODEL` | Ollama model used for photo tagging. Defaults to `llava`. |
| `PROJECTS_API_URL` | Real projects API. Leave it unset to use mock data. |
| `PROJECTS_BASE_URL` | Prefix for relative project links returned by the API. |

## Tests

```bash
cd server && cargo test
cd mobile && flutter test
```

## More detail

- `server/README.md` covers SQLx setup and common database errors.
- `mobile/README.md` covers iOS Simulator setup.
- `scripts/README.md` covers the AI service.
