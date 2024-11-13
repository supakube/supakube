use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::Extension,
    routing::get,
    Router,
};
use futures::{stream, SinkExt, StreamExt};
use futures::{FutureExt, TryStreamExt};
use std::{env, net::SocketAddr};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    // Get the connection string from the environment variable
    let database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL must be set");

    // PostgreSQL connection using the connection string from the environment variable.
    let (client, mut connection) = tokio_postgres::connect(&database_url, NoTls).await.unwrap();

    // Make transmitter and receiver.
    let (tx, rx) = futures_channel::mpsc::unbounded();
    let stream = stream::poll_fn(move |cx| connection.poll_message(cx)).map_err(|e| panic!("{}", e));
    let connection = stream.forward(tx).map(|r| r.unwrap());
    tokio::spawn(connection);

    // Create a broadcast channel for notifications
    let (broadcast_tx, _) = broadcast::channel::<String>(100);
    let broadcast_tx = Arc::new(broadcast_tx);

    // Clone for moving into task
    let broadcast_tx_clone = broadcast_tx.clone();

    // Wait for notifications and broadcast them
    tokio::spawn(async move {
        let mut rx = rx;

        while let Some(message) = rx.next().await {
            if let tokio_postgres::AsyncMessage::Notification(notification) = message {
                println!("Notification {:?}", notification);
                // Send notification over broadcast channel
                let payload = notification.payload();
                if let Err(e) = broadcast_tx_clone.send(payload.to_string()) {
                    eprintln!("Broadcast send error: {}", e);
                }
            }
        }
    });

    // Execute LISTEN command
    if let Err(e) = client.batch_execute("LISTEN item_updates;").await {
        eprintln!("Error {}", e);
    }

    // Set up the WebSocket route using Axum
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(broadcast_tx.clone()));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid u16 number");

    // Define the address to listen on (0.0.0.0)
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // run it
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Handler for the WebSocket upgrade
async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(broadcast_tx): Extension<Arc<broadcast::Sender<String>>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, broadcast_tx))
}

async fn handle_ws(socket: WebSocket, broadcast_tx: Arc<broadcast::Sender<String>>) {
    let (mut sender, mut receiver) = socket.split();

    // Create a broadcast receiver
    let mut broadcast_rx = broadcast_tx.subscribe();

    // Spawn a task to receive messages from the WebSocket client (if needed)
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Handle messages from the client if needed
                println!("Received message from client: {}", text);
            }
        }
    });

    // Forward messages from broadcast channel to WebSocket client
    tokio::spawn(async move {
        while let Ok(msg) = broadcast_rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                // Client disconnected
                break;
            }
        }
    });
}
