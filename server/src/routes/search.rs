// TODO: stub these
// [ ] GET /Search - view
// [ ] No Query
// [ ] QUery - fake results
use poem::{
    handler,
    web::{ CsrfToken, Form, Html},
    Result,
};
use sailfish::TemplateOnce;
use serde::Deserialize;

use crate::render::render_with_layout;

#[derive(Deserialize)]
struct SearchForm {
    query: String,
}

#[derive(TemplateOnce)]
#[template(path = "search.stpl.html")]
struct SearchPageTemplate {
    csrf_token: String,
}

#[derive(TemplateOnce)]
#[template(path = "search_results.stpl.html")]
struct SearchResultsTemplate {
    query: String,
    results: Vec<String>,
}

#[handler]
pub async fn search_page(token: &CsrfToken) -> Result<Html<String>> {
    render_with_layout("Search", SearchPageTemplate {
        csrf_token: token.0.clone(),
    })
}

#[handler]
pub async fn search_results(form: Form<SearchForm>) -> Result<Html<String>> {
    // TODO: replace with real search later
    let query = form.query.clone();

    // stubbed results
    let results = vec![
        format!("Result 1 for '{}'", query),
        format!("Result 2 for '{}'", query),
        format!("Result 3 for '{}'", query),
    ];

    Ok(Html(
        SearchResultsTemplate { query, results }
            .render_once()
            .map_err(poem::error::InternalServerError)?,
    ))
}
