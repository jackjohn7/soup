use axum::response::Response;
use maud::html;
use tracing::debug;

use crate::components::layout::Layout;

pub async fn index() -> Response {
    debug!("hit the index route");
    // default layout is what we need for index
    Layout::builder().tailwindcss().build().render(html! {
        h1 ."text-4xl font-bold" { "Soup.rs" }
        p { "Bowl template registry" }
        a href="/packages" hx-headers="{\"ACCEPT\": \"text/html\"}" { "Packages" }
        form method="GET" action="/packages" hx-headers="{\"ACCEPT\": \"text/html\"}" {
            input name="q" placeholder="search for a package" {}
            button { "Search" }
        }
    })
}
