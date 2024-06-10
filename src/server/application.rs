use std::sync::Arc;

use crate::{
    routes::{general, packages},
    AppState,
};
use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

pub struct App {
    state: Arc<AppState>,
}

impl App {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn router(self) -> anyhow::Result<Router> {
        let router = Router::new()
            .route("/", get(general::index))
            .route("/packages", get(packages::get))
            .route("/packages", post(packages::post))
            .with_state(self.state)
            .nest_service(
                "/public/styles",
                ServeDir::new(format!(
                    "{}/public/styles",
                    std::env::current_dir()?
                        .to_str()
                        .context("couldn't find cwd")?
                )),
            );
        Ok(router)
    }
}
