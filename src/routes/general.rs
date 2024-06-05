use axum::response::Response;
use maud::html;
use tracing::debug;

use crate::components::layout::Layout;

pub async fn index() -> Response {
    debug!("hit the index route");
    // default layout is what we need for index
    Layout::builder().build().render(html! {"hello, world"})
}
