mod client_ws;
mod runtime;
mod server;
mod state;
mod upload;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use upload::PATH_PREFIX;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Check if the directory exists, if it does, remove it and create a new one
    match std::fs::metadata(PATH_PREFIX.clone()).is_ok() {
        true => {
            std::fs::remove_dir_all(PATH_PREFIX.clone()).unwrap();
            std::fs::create_dir_all(PATH_PREFIX.clone()).unwrap();
        }
        false => std::fs::create_dir_all(PATH_PREFIX.clone()).unwrap(),
    }

    let state: state::AppState = state::AppState::new();

    server::serve(state.clone()).await;
}
