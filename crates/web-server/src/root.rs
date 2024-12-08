use axum::response::Html;
use dioxus::prelude::*;
use crate::layout::{Layout, render_page};

pub async fn loader() -> Html<String> {

    Html(index())
}

fn index() -> String {
    let page = rsx! {
        Layout {
            title: "My App",
            h1 {
                "test"
            }
        }
    };

    render_page(page)
}