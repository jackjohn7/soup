use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{header::ACCEPT, StatusCode},
    response::{IntoResponse, Response},
};
use maud::html;
use tracing::debug;

use crate::{components::layout::Layout, AppState};

struct _GetQuery {
    q: String,
    sort: String,
}
/// GET /packages
pub async fn get(State(_): State<Arc<AppState>>, req: Request) -> Result<Response, StatusCode> {
    debug!("hit the packages route");

    // TODO: Use query parameters to search (q, sort)
    // Deserialize them into `GetQuery` struct
    match req.headers().get(ACCEPT).map(|x| x.to_str().unwrap()) {
        Some(x) if x.contains("text/html") => {
            // TODO: Need to match on whether or not the htmx header is set
            // If it isn't we need to return the entire page. Otherwise, a partial could be fine
            // Realistically, this should be my approach for all routes
            debug!("wants html");
            Ok(Layout::builder().tailwindcss().build().render(html! {
            h1 { "Soup.rs Packages" }
            }))
        }
        // TODO: display same thing as html but with plain-text for curl search
        Some(x) if x.contains("text/plain") => Ok(String::from("Soup.rs Packages").into_response()),
        Some(x) if x.contains("application/json") => {
            Ok(String::from("{\"packages\": []}").into_response())
        }
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

/// POST /packages
///
/// This route allows a user to publish a package
pub async fn post(State(_): State<Arc<AppState>>) -> Result<Response, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
