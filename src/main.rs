use std::{fs, net::SocketAddr, path::Path, sync::Arc};

use tokio::sync::Mutex;
use uuid::Uuid;

use axum::{
    debug_handler,
    error_handling::HandleErrorLayer,
    extract::{
        ws::{Message, WebSocket},
        Path as ePath, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    BoxError, Json, Router,
};

// use serde::{de, Deserialize};

use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use tempfile::NamedTempFile;
use tower_http::services::ServeDir;

use tracing::info;

use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

const IMAGE_COUNT: usize = 5;

struct AppState {
    images: Mutex<Vec<String>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Rate liming service.
    // Limit to a burst of 5 requests, based on IP,
    // Regen 1 request per 500 ms.
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_millisecond(500)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    let state = AppState {
        images: Mutex::new(vec![]),
    };

    let router = Router::new()
        // Statically serve frontend.
        .nest_service("/", ServeDir::new("public"))
        // Get route, returns the image vec.
        .route("/ws/connect", get(ws_upgrade_handler))
        // Get route, returns the image vec.
        .route("/api/get", get(get_vec))
        // Upload route, used for uploading a single image.
        .route("/api/upload", post(upload))
        // delete route, used for deleting a single image from the
        .route("/api/delete/:image", delete(delete_file))
        // Shared state, containing vec of images.
        .with_state(Arc::new(state))
        // Add the rate limiting service
        .layer(
            ServiceBuilder::new()
                // this middleware goes above `GovernorLayer` because it will receive
                // errors returned by `GovernorLayer`
                .layer(HandleErrorLayer::new(|e: BoxError| async move {
                    display_error(e)
                }))
                .layer(GovernorLayer {
                    // We can leak this because it is created once and then
                    config: Box::leak(governor_conf),
                }),
        );

    let server = axum::Server::bind(&"0.0.0.0:7979".parse().unwrap())
        .serve(router.into_make_service_with_connect_info::<SocketAddr>());

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

#[axum::debug_handler]
async fn ws_upgrade_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async { ws_connection(state, ws).await })
}

async fn ws_connection(app_state: Arc<AppState>, mut ws: WebSocket) {
    // bring the client up to speed...
    let current_images = app_state.images.lock().await.clone();
    let current_images = serde_json::to_string(&current_images).unwrap();
    ws.send(Message::Text(current_images)).await.unwrap();

    loop {
        let len = app_state.images.lock().await.len();
    }
}

async fn watch_for_changes(app_state: Arc<AppState>, len: usize) {
    todo!()
}

#[derive(TryFromMultipart)]
struct ImageUpload {
    #[form_data(limit = "10MiB")]
    file: FieldData<NamedTempFile>,
}

#[debug_handler]
async fn upload(
    State(state): State<Arc<AppState>>,
    TypedMultipart(ImageUpload { file }): TypedMultipart<ImageUpload>,
) -> impl IntoResponse {
    let name = file.metadata.file_name.expect("Error getting name");

    let ext = get_file_ext(&name).await;
    if !(is_image(&ext).await) {
        info!("-> /api/upload = Not an image: {}", &name);
        return StatusCode::UNPROCESSABLE_ENTITY;
    };

    let file_name = format!("{}.{}", Uuid::new_v4(), ext);

    let path = Path::new("./public/images").join(&file_name);

    match file.contents.persist(path) {
        Ok(_) => {
            update_vec(state, &file_name).await;
            info!("-> /api/upload = Saved: {}", &name);
            StatusCode::CREATED
        }
        Err(e) => {
            info!("{}", e);
            info!("-> /api/upload = Error saving: {}", &name);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn get_file_ext(file_name: &String) -> String {
    let ext = Path::new(file_name).extension().unwrap().to_str().unwrap();

    ext.to_lowercase()
}

async fn is_image(ext: &str) -> bool {
    matches!(ext, "jpg" | "jpeg" | "png" | "bmp" | "svg" | "gif" | "raw")
}

// Update the shared Vec<String>.
async fn update_vec(state: Arc<AppState>, file: &str) {
    let mut images = state.images.lock().await;
    images.push(file.to_string());

    let len = images.len();
    if len > IMAGE_COUNT {
        let v = images.clone();
        let (remove_list, new_list) = v.split_at(len - IMAGE_COUNT);
        *images = new_list.to_vec().clone();
        drop(images);

        for file in remove_list {
            del(file).await;
        }
    }
}

// get the shared Vec<String>.
#[debug_handler]
async fn get_vec(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    info!("-> /api/get");

    let images = state.images.lock().await.clone();
    Json(images)
}

// delete a file, require a token to delete files
#[debug_handler]
async fn delete_file(
    State(state): State<Arc<AppState>>,
    ePath(image): ePath<String>,
) -> impl IntoResponse {
    info!("-> /api/delete");

    let mut images = state.images.lock().await;

    if let Some(image_pos) = images.iter().position(|x| *x == image) {
        del(&image).await;
        images.remove(image_pos);
        info!("removed {}", image);
        StatusCode::FOUND
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn del(file: &String) {
    fs::remove_file(format!("./public/images/{}", file)).unwrap();
}
