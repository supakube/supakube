use dioxus::prelude::*;
use assets::files::*;

// Method to render components to HTML
pub fn render_page(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

#[component]
pub fn Layout(title: String, children: Element) -> Element {
    rsx!(
        head {
            title {
                "{title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
            link {
                href: output_css.name,
                rel: "stylesheet",
                type: "text/css"
            }
        }
        body {
            {children}
        }
    )
}
