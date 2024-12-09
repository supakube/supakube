use dioxus::prelude::*;
use assets::files::*;
use daisy_rsx::BlankSlate;
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
                nav {
                    aria_label: "breadcrumb",
                    ol {
                        class: "flex flex-wrap items-center gap-1.5 break-words text-sm sm:gap-2.5",
                        li {
                            class: "items-center gap-1.5 hidden md:block",
                            "Your Application"
                        }
                        li {
                            ">"
                        }
                        li {
                            "Dashboard"
                        }
                    }
                }
            ),
            selected_item: SideBar::Dashboard,
            BlankSlate {
                heading: "Welcome to Your App",
                description: "Read the docs to find out more",
                visual: favicon_svg.name
            }
        }
    };

    render_page(page)
}