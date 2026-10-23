//! `devr-server`: serves the DEVR+ editor to a browser and runs its backend.
//!
//! ```text
//! devr-server [--port 7878] [--ui path/to/ui/dist] [--host 127.0.0.1]
//! ```
//!
//! Then open `http://127.0.0.1:7878`. The page talks to the backend over
//! `/ws` with the same messages the desktop app uses. It listens on this
//! computer only unless `--host` says otherwise: anyone who can open the page
//! can read and run anything this user can.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use devr_core::Core;
use serde_json::{Value, json};
use tokio::sync::mpsc;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let mut port = 7878u16;
    let mut host = "127.0.0.1".to_owned();
    let mut ui = PathBuf::from("ui/dist");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--port" => port = args.next().context("--port needs a number")?.parse()?,
            "--host" => host = args.next().context("--host needs an address")?,
            "--ui" => ui = args.next().context("--ui needs a folder")?.into(),
            other => anyhow::bail!("unknown argument `{other}`"),
        }
    }
    let app = Router::new()
        .route("/ws", get(websocket))
        .fallback_service(ServeDir::new(&ui))
        .with_state(());
    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("DEVR+ is running at http://{addr}  (UI from {})", ui.display());
    axum::serve(listener, app).await?;
    Ok(())
}

async fn websocket(ws: WebSocketUpgrade, State(()): State<()>) -> impl IntoResponse {
    ws.max_message_size(256 * 1024 * 1024).on_upgrade(session)
}

/// One browser tab: one backend.
async fn session(mut socket: WebSocket) {
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let events = tx.clone();
    let core = Core::new(Arc::new(move |event: &str, payload: Value| {
        let _ = events.send(json!({ "event": event, "payload": payload }).to_string());
    }));

    loop {
        tokio::select! {
            outgoing = rx.recv() => {
                let Some(text) = outgoing else { break };
                if socket.send(Message::Text(text.into())).await.is_err() {
                    break;
                }
            }
            incoming = socket.recv() => {
                let Some(Ok(message)) = incoming else { break };
                let Message::Text(text) = message else { continue };
                let Ok(request) = serde_json::from_str::<Value>(&text) else { continue };
                let core = core.clone();
                let tx = tx.clone();
                tokio::spawn(async move {
                    let id = request["id"].clone();
                    let method = request["method"].as_str().unwrap_or_default().to_owned();
                    let params = request["params"].clone();
                    let reply = match core.call(&method, params).await {
                        Ok(result) => json!({ "id": id, "result": result }),
                        Err(err) => json!({ "id": id, "error": format!("{err:#}") }),
                    };
                    let _ = tx.send(reply.to_string());
                });
            }
        }
    }
    core.shutdown().await;
}
