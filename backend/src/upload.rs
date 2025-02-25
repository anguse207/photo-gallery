use axum::extract::{multipart::Field, Multipart, State};
use tracing::{error, info};

use crate::state::AppState;

pub const PATH_PREFIX: &str = "images/";

pub async fn handle_files(State(state): State<AppState>, mut multipart: Multipart) {
    while let Some(file) = multipart.next_field().await.unwrap() {
        handle_file(file, state.clone()).await;
    }
}

async fn handle_file(file: Field<'_>, state: AppState) {
    let name = file.file_name().unwrap().to_string();
    let content_type = file.content_type().unwrap().to_string();
    let bytes = file.bytes().await.unwrap();

    if !is_image(&name, &content_type) {
        return;
    }

    info!(
        "(`{name}`: `{content_type}`) is {size} bytes",
        size = bytes.len()
    );

    std::fs::write(format!("{PATH_PREFIX}{}", &name), bytes).unwrap();
    state.add_image(&name);

    if !state.is_started() {
        state.start().await;
    }
}

const SUPPORTED_IMAGE_FORMATS: [&str; 8] = [
    ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".svg", ".avif",
];

fn is_image(name: &str, content_type: &str) -> bool {
    if !content_type.starts_with("image/") {
        error!("File `{}` is not an image", name);
        return false;
    }

    if !SUPPORTED_IMAGE_FORMATS
        .iter()
        .any(|format| name.to_lowercase().ends_with(format))
    {
        error!("File `{}` has an unsupported format", name);
        return false;
    }

    true
}
