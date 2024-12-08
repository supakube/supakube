use dioxus::prelude::*;
use daisy_rsx::{Button, ButtonScheme, ButtonSize};

use crate::layout::{render_page, Layout};

pub fn index() -> String {
    let page = rsx! {
        Layout {
            title: "My App",
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