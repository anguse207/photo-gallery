use std::{fs, path::Path, sync::Arc};

use tokio::sync::Mutex;
use uuid::Uuid;

use axum::{
    debug_handler,
    extract::{Path as ePath, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router, Server,
};

use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use tempfile::NamedTempFile;
use tower_http::services::ServeDir;

use tracing::info;

// TODO: Add rate limiting based on ip
// https://github.com/benwis/tower-governor

const IMAGE_COUNT: usize = 10;

struct AppState {
    images: Mutex<Vec<String>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        images: Mutex::new(vec![]),
    });

    let router = Router::new()
        // Statically serve frontend
        .nest_service("/", ServeDir::new("public"))
        // get route
        .route("/api/get", get(get_vec))
        // Upload route
        .route("/api/upload", post(upload))
        // delete route
        .route("/api/delete/:image", delete(delete_file))
        .with_state(state);

    let server = Server::bind(&"0.0.0.0:7001".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

#[derive(TryFromMultipart)]
struct Upload {
    #[form_data(limit = "10MiB")]
    file: FieldData<NamedTempFile>,
}

#[debug_handler]
async fn upload(
    State(state): State<Arc<AppState>>,
    TypedMultipart(Upload { file }): TypedMultipart<Upload>,
) -> impl IntoResponse {
    info!("-> /api/upload");

    let name = file.metadata.file_name.expect("Error getting name");
    info!("New upload: {}", &name);

    let ext = get_file_ext(&name).await;
    if !(is_image(&ext).await) {
        return StatusCode::UNPROCESSABLE_ENTITY;
    };

    let file_name = format!("{}.{}", Uuid::new_v4().to_string(), ext);

    let path = Path::new("./images").join(&file_name);

    match file.contents.persist(path) {
        Ok(_) => {
            update_vec(state, &file_name).await;
            StatusCode::CREATED
        }
        Err(e) => {
            info!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn get_file_ext(file_name: &String) -> String {
    let ext = Path::new(file_name).extension().unwrap().to_str().unwrap();

    return ext.to_lowercase();
}

async fn is_image(ext: &str) -> bool {
    match ext {
        "jpg" | "jpeg" | "png" | "bmp" | "svg" | "gif" | "raw" => {
            info!("ext: {}", { ext });
            true
        }
        _ => {
            info!("ext: {}", { ext });
            false
        }
    }
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
            del(&file).await;
        }
    }
}

// get the shared Vec<String>.
#[debug_handler]
async fn get_vec(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    info!("-> /api/get");

    let images = state.images.lock().await.clone();
    return Json(images);
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
    fs::remove_file(format!("./images/{}", file)).unwrap();
}
