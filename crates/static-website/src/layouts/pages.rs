use super::layout::Layout;
use crate::{components::footer::Footer, components::navigation::Section, generator::Page};
use dioxus::prelude::*;

#[component]
pub fn MarkdownPage(post: Page) -> Element {
    let content = crate::markdown::markdown_to_html(post.markdown);
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            mobile_menu: None,
            section: Section::None,
            article {
                class: "mx-auto max-w-2xl p-4",
                h1 {
                    class: "reset-tw",
                    "{post.title}"
                }
                div {
                    class: "reset-tw",
                    dangerous_inner_html: "{content}"
                }
            }
            Footer {}
        }
    }
}
