use dotenv::dotenv;
use std::sync::Arc;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod components;
pub mod config;
pub mod errors;
pub mod routes;
pub mod server;
use config::Env;
use server::application::App;

// TODO: Move state into separate module
pub struct AppState {
    pub env: Env,
}

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

    info!("Load .env file into environment");
    dotenv()?;

    info!("Reading environment variables");
    let env = envy::from_env::<Env>()?;

    // TODO: connect to database

    let app_state = Arc::new(AppState { env });

    let app = App::new(app_state).router().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5173").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
