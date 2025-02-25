use crate::state::AppState;

pub async fn start(state: AppState) {
    tokio::spawn(async move {
        state.try_next_image();
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    });
}
