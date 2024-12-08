use dioxus::prelude::*;
use daisy_rsx::Button;

use crate::layout::{render_page, Layout};

pub fn index() -> String {
    let page = rsx! {
        Layout {
            title: "My App",
            h1 {
                "test5"
            }
            Button {
                "Hello"
            }
        }
    };

    render_page(page)
}
