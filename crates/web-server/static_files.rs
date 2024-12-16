use axum::body::Body;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use serde::Deserialize;
use tokio_util::io::ReaderStream;
use web_assets::files::StaticFile;

#[derive(TypedPath, Deserialize)]
#[typed_path("/static/*path")]
pub struct StaticFilePath {
    pub path: String,
}

pub async fn static_path(StaticFilePath { path }: StaticFilePath) -> impl IntoResponse {
    let path = format!("/static/{}", path);

    let data = StaticFile::get(&path);

    if let Some(data) = data {
        let file = match tokio::fs::File::open(data.file_name).await {
            Ok(file) => file,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap()
            }
        };

        // convert the `AsyncRead` into a `Stream`
        let stream = ReaderStream::new(file);

        return Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime).unwrap(),
            )
            .body(Body::from_stream(stream))
            .unwrap();
    }
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
