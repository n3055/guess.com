use lazy_static::lazy_static;
use tokio::sync::broadcast;

lazy_static! {
    pub static ref GAME_UPDATES_TX: broadcast::Sender<String> = {
        let (tx, _) = broadcast::channel(100);
        tx
    };
}

pub async fn start_ws_server() {
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr = format!("{}:3001", host);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind WS server to {}: {}", addr, e);
            return;
        }
    };
    println!("WebSocket server listening on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = GAME_UPDATES_TX.clone();
        tokio::spawn(async move {
            let mut game_id = String::new();
            let callback = |req: &tokio_tungstenite::tungstenite::handshake::server::Request, resp| {
                let path = req.uri().path();
                if let Some(id) = path.strip_prefix("/ws/") {
                    game_id = id.to_string();
                }
                Ok(resp)
            };
            
            let ws_stream = match tokio_tungstenite::accept_hdr_async(stream, callback).await {
                Ok(s) => s,
                Err(_) => return,
            };
            
            if game_id.is_empty() {
                return;
            }

            let mut rx = tx.subscribe();
            let (mut write, mut _read) = futures_util::StreamExt::split(ws_stream);

            use futures_util::SinkExt;
            while let Ok(msg) = rx.recv().await {
                if let Some(target_game_id) = msg.strip_suffix(":refresh") {
                    if target_game_id == game_id {
                        if write.send(tokio_tungstenite::tungstenite::Message::Text("refresh".into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });
    }
}
