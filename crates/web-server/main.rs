mod config;
mod errors;
mod static_files;
use crate::errors::CustomError;
use axum::response::Html;
use axum::{extract::Extension, routing::get, Router};
use std::net::SocketAddr;
use tower_livereload::LiveReloadLayer;
use web_pages::root;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(loader))
        .route("/static/*path", get(static_files::static_path))
        .layer(Extension(config))
        .layer(LiveReloadLayer::new())
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on... {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn loader(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    let html = root::index(users);

    Ok(Html(html))
}
