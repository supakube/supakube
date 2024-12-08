#![allow(non_snake_case)]
mod layout;
mod root;
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    // build our application with a route
    let app = Router::new()
        .route("/", get(root::loader));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}