#![allow(non_snake_case)]
mod root;
mod static_files;
use axum::{routing::get, Router};
use axum_extra::routing::RouterExt;
use std::net::SocketAddr;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .typed_get(static_files::static_path)
        .route("/", get(root::loader))
        .layer(LiveReloadLayer::new());

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
