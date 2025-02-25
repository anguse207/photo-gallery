use tracing::info;

use crate::state::AppState;

pub async fn start(state: AppState) {
    tokio::spawn(async move {
        info!("Starting runtime...");

        state.try_next_image();
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        loop {
            state.try_next_image();
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
}
