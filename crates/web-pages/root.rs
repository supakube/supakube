use dioxus::prelude::*;
use daisy_rsx::{Button, ButtonScheme};

use crate::layout::{render_page, Layout};

pub fn index() -> String {
    let page = rsx! {
        Layout {
            title: "My App",
            h1 {
                "test5"
            }
            Button {
                button_scheme: ButtonScheme::Primary,
                "Hello"
            }
        }
    };

    render_page(page)
}
