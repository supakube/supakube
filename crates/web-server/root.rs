use axum::response::Html;

pub async fn loader() -> Html<String> {

    Html(web_pages::root::index())
}