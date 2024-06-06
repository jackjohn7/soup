use axum::{
    http::{header::ACCEPT, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use maud::html;
use tracing::debug;

use crate::components::layout::Layout;

/// GET /packages
pub async fn get(headers: HeaderMap) -> Result<Response, StatusCode> {
    debug!("hit the packages route");

    // TODO: Use query parameters to search (q, sort)
    match headers.get(ACCEPT).map(|x| x.to_str().unwrap()) {
        Some(x) if x.contains("text/html") => {
            // TODO: Need to match on whether or not the htmx header is set
            // If it isn't we need to return the entire page. Otherwise, a partial could be fine
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
