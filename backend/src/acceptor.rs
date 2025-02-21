use axum::extract::{multipart::Field, Multipart};
use tracing::{error, info};

pub async fn handle_files(mut multipart: Multipart) {
    while let Some(file) = multipart.next_field().await.unwrap() {
        handle_file(file).await;
    }
}

async fn handle_file(file: Field<'_>) {
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
