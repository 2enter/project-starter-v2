use crate::state::AppState;
use axum::extract::{
    State, WebSocketUpgrade,
    ws::{Message, WebSocket},
};
use axum::response::IntoResponse;
use axum_extra::{TypedHeader, headers};
use common::model::WsMsg;
use futures::{SinkExt, StreamExt};
use tracing::debug;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown")
    };
    ws.on_upgrade(|socket| handle_socket(socket, app_state, user_agent))
}

async fn handle_socket(socket: WebSocket, app_state: AppState, user_agent: String) {
    tracing::info!("WebSocket connection established {user_agent:?}");
    let (mut sender, mut receiver) = socket.split();
    let mut rx = app_state.ws_sender.subscribe();

    // Spawn a task to forward broadcast messages to the WebSocket
    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            if sender.send(message.into()).await.is_err() {
                break; // Stop if the WebSocket connection is closed
            }
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        // parse message into text

        let Message::Text(text) = msg else {
            if let Message::Close(Some(_)) = msg {
                tracing::info!("Client disconnected");
                continue;
            }
            continue;
        };

        tracing::info!("Received WS message: {text:?}");

        // parse text into WSMsg
        let Ok(ws_msg) = serde_json::from_str::<WsMsg>(text.to_string().as_str()) else {
            continue;
        };

        // use the WSMsg instance
        match ws_msg {
            // WsMsg::Place(_) => {
            //     let _ = ws_broadcast(ws_msg, &app_state.ws_sender);
            // }
            _ => {
                debug!("ws message received: {:?}", ws_msg);
            }
        }
    }
}
