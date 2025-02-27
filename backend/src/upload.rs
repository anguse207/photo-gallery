use axum::extract::{multipart::Field, Multipart, State};
use tracing::{debug, error};

use crate::state::AppState;

pub const PATH_PREFIX: &str = "";

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

    debug!(
        "(`{name}`: `{content_type}`) is {size} bytes",
        size = bytes.len()
    );

    let uuid_name = uuid::Uuid::new_v4().to_string();
    std::fs::write(format!("{PATH_PREFIX}{}", &uuid_name), bytes).unwrap();
    state.add_image(&uuid_name);

    if !state.is_started() {
        state.start().await;
    }
}

const SUPPORTED_IMAGE_FORMATS: [&str; 10] = [
    ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".svg", ".avif", "ico", "gif",
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
