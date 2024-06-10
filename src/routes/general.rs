use axum::response::Response;
use maud::html;
use tracing::debug;

use crate::components::layout::Layout;

pub async fn index() -> Response {
    debug!("hit the index route");
    // default layout is what we need for index
    Layout::builder().tailwindcss().build().render(html! {
        a href="/packages" hx-headers="{\"ACCEPT\": \"text/html\"}" { "Packages" }
        form method="GET" action="/packages" hx-headers="{\"ACCEPT\": \"text/html\"}" {
            input name="q" placeholder="search for a package" {}
            button { "Search" }
        }
    })
}
