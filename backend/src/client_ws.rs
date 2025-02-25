use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::info;

use crate::state::AppState;

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn client_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    info!("{addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| client_ws(socket, state))
}

async fn client_ws(socket: WebSocket, state: AppState) {
    // This is the actual websocket handler. This is where we can send and receive messages.
    // We can also close the connection from the server side.
    let (mut tx_ws, mut rx_ws) = socket.split();

    let init_msg = rx_ws.next().await.unwrap();
    info!("Received: {:?}", init_msg.unwrap());

    let mut rx_channel = state.tx.subscribe();

    loop {
        let image = rx_channel.recv().await;

        if let Ok(image) = image {
            tx_ws.send(Message::Binary(image.into())).await.unwrap();
        }
    }
}
