use serde::{Deserialize, Serialize};
use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::header::ACCEPT,
    response::{IntoResponse, Response},
    Json,
};
use bowl_core::config::Config;
use maud::html;
use tracing::debug;

use crate::{components::layout::Layout, errors::SoupError, AppState};

struct _GetQuery {
    q: String,
    sort: String,
}
/// GET /packages
pub async fn get(State(_): State<Arc<AppState>>, req: Request) -> Result<Response, SoupError> {
    debug!("hit the packages route");

    // TODO: Use query parameters to search (q, sort)
    // Deserialize them into `GetQuery` struct
    match req.headers().get(ACCEPT).map(|x| x.to_str().unwrap()) {
        Some(x) if x.contains("text/html") => {
            // TODO: Need to match on whether or not the htmx header is set
            // If it isn't we need to return the entire page. Otherwise, a partial could be fine
            // Realistically, this should be my approach for all routes
            Ok(Layout::builder().tailwindcss().build().render(html! {
            h1 { "Soup.rs Packages" }
            }))
        }
        // TODO: display same thing as html but with plain-text for curl search
        Some(x) if x.contains("text/plain") || x.contains("*/*") => {
            Ok(String::from("Soup.rs Packages").into_response())
        }
        Some(x) if x.contains("application/json") => {
            Ok(String::from("{\"packages\": []}").into_response())
        }
        Some(e) => Err(SoupError::InvalidHeader {
            expected: "text/html | text/plain | application/json | */*",
            found: e.into(),
        }),
        None => Err(SoupError::MissingHeader("ACCEPT")),
    }
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    config: Config,
    readme: String,
    data: Vec<u8>,
}

/// POST /packages
///
/// This route allows a user to publish a package
pub async fn post(
    State(_): State<Arc<AppState>>,
    Json(_body): Json<Package>,
) -> Result<Response, SoupError> {
    Err(SoupError::NotImplemented)
}
