use std::path::Path;

use uuid::Uuid;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, Server,
};

use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use tempfile::NamedTempFile;
use tower_http::services::ServeDir;

use tracing::info;

// TODO: Add rate limiting based on ip

#[derive(TryFromMultipart)]
struct Upload {
    #[form_data(limit = "10MiB")]
    file: FieldData<NamedTempFile>,
}

async fn upload(TypedMultipart(Upload { file }): TypedMultipart<Upload>) -> impl IntoResponse {
    info!("-> /api/upload");

    let name = file.metadata.file_name.expect("Error getting name");
    let ext = Path::new(&name).extension().unwrap().to_str().unwrap();

    info!("New upload: {}, of type {:?}", &name, &ext);

    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "bmp" | "svg" | "gif" | "raw" => (),
        _ => panic!("Not an image, Killing thread"),
    }

    let file_name = format!("{}.{}", Uuid::new_v4().to_string(), ext) ;

    let path = Path::new("./images").join(file_name);

    match file.contents.persist(path) {
        Ok(_) => {

            StatusCode::CREATED
        },
        Err(e) => {
            info!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

// Update the shared Vec<String>.
async fn update_vec(file: String) -> impl IntoResponse {
    
}

// get the shared Vec<String>.
async fn get_vec(file: String) -> impl IntoResponse {
    
}

// delete a file, require a token to delete files
async fn delete_file() -> impl IntoResponse {

}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let router = Router::new()
        // Statically serve frontend
        .nest_service("/", ServeDir::new("public"))
        // Upload route
        .route("/api/upload", post(upload))
        // Test route
        .route("/api/hello", get(|| async move { "Hello World!" }));

    let server = Server::bind(&"0.0.0.0:7001".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}
