use axum::{
    extract::DefaultBodyLimit,
    routing::{any, get},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

use crate::client_ws::client_ws_handler;
use crate::state::AppState;
use crate::{frontend, upload};

pub async fn serve(state: AppState) {
    // build our application with some routes
    let app = Router::new()
        .route(
            "/api/upload",
            get(frontend::upload_form).post(upload::handle_files),
        )
        .route("/api/ws", any(client_ws_handler))
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

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", &listener.local_addr().unwrap());

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    })
    .await
    .unwrap();
}
