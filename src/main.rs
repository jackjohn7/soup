use anyhow::Context;
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod components;
pub mod routes;
use routes::general;

struct AppState {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set up tracing for logging with defaults
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env() // env: RUST_LOG
                .unwrap_or_else(|_| "soup=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing Soup Registry");

    // TODO: connect to database

    let _app_state = Arc::new(AppState {});

    let app = Router::new()
        .route("/", get(general::index))
        //.with_state(app_state)
        .nest_service(
            "/public/styles",
            ServeDir::new(format!(
                "{}/public/styles",
                std::env::current_dir()?
                    .to_str()
                    .context("couldn't find cwd")?
            )),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5173").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
