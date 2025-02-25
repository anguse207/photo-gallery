use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::info;

use crate::state::AppState;

pub async fn client_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    info!("Received upgrade request");
    // // finalize the upgrade process by returning upgrade callback.
    // // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| client_ws(socket, state))
}

async fn client_ws(socket: WebSocket, state: AppState) {
    info!("Client connected");

    let (mut tx_ws, mut _rx_ws) = socket.split();

    // let init_msg = rx_ws.next().await.unwrap();
    // info!("Received: {:?}", init_msg.unwrap());

    let mut rx_channel = state.tx.subscribe();

    loop {
        let image = rx_channel.recv().await;

        if let Ok(image) = image {
            match tx_ws.send(Message::Binary(image.into())).await {
                Ok(_) => info!("Image sent to client"),
                Err(_) => {
                    info!("Client disconnected");
                    break;
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
}
