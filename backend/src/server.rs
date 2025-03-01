use axum::{
    extract::DefaultBodyLimit,
    routing::{any, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::client_ws::client_ws_handler;
use crate::state::AppState;
use crate::upload;

pub async fn serve(state: AppState) {
    // build our application with some routes
    let app = Router::new()
        .route("/api/upload", post(upload::handle_files))
        .route("/api/ws", any(client_ws_handler))
        .fallback_service(ServeDir::new(std::env::var("FRONTEND_DIR").unwrap()))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024 /* 10mb */))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let address = format!("0.0.0.0:{}", std::env::var("BACKEND_PORT").unwrap());
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::debug!("listening on {}", &listener.local_addr().unwrap());

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    })
    .await
    .unwrap();
}
