use dioxus::prelude::*;

use crate::layout::{render_page, Layout};

pub fn index() -> String {
    let page = rsx! {
        Layout {
            title: "My App",
            h1 {
                "test2"
            }
        }
    };

    render_page(page)
}
