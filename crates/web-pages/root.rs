use dioxus::prelude::*;
use daisy_rsx::{Button, ButtonScheme, ButtonSize};
use crate::layout::{render_page, Layout, SideBar};
use axum_extra::routing::TypedPath;
use serde::Deserialize;

#[derive(TypedPath, Deserialize)]
#[typed_path("/")]
pub struct Index {
}

pub fn index() -> String {
    let page = rsx! {
        Layout {
            title: "My App",
            header: rsx!(
                h1 {
                    "test6"
                }
            ),
            selected_item: SideBar::Dashboard,
            h1 {
                "test6"
            }
            Button {
                button_scheme: ButtonScheme::Primary,
                button_size: ButtonSize::Medium,
                "Hello"
            }
        }
    };

    render_page(page)
}