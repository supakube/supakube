use crate::routes::{blog, docs, marketing};
use dioxus::prelude::*;

#[component]
pub fn Navigation(mobile_menu: Element) -> Element {
    rsx! {
        header { class: "navigation",
            div {
                class: "navigation-container",
                div {
                    class: "navigation-logo",
                    a {
                        class: "navigation-logo-link", href: marketing::Index {}.to_string(),
                        "hx-boost": "true",
                        img {
                            alt: "Logo",
                            width: "22",
                            height: "22",
                            src: "/logo.svg",
                            class: "navigation-logo-icon",
                        }
                        span { class: "navigation-logo-text", "Bionic-GPT" }
                    }
                    nav { class: "navigation-menu",
                        a { class: "navigation-menu-item", "hx-boost": "true", href: marketing::Pricing {}.to_string(), "Pricing" }
                        a { class: "navigation-menu-item", "hx-boost": "true", href: blog::Index {}.to_string(), "Blog" }
                        a { class: "navigation-menu-item", "hx-boost": "true", href: marketing::PartnersPage {}.to_string(), "Partners" }
                        a { class: "navigation-menu-item", "hx-boost": "true", href: marketing::EnterprisePage {}.to_string(), "Enterprise" }
                        a { class: "navigation-menu-item", "hx-boost": "true", href: marketing::Contact {}.to_string(), "Contact Us" }
                    }
                }

                // Mobile Toggle Button
                button {
                    class: "navigation-toggle-button",
                    r#type: "button",
                    "aria_has_popup": "dialog",
                    aria_expanded: "false",
                    aria_controls: "mobile-menu",
                    "data_state": "closed",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        class: "!size-6",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M3.75 9h16.5m-16.5 6.75h16.5",
                        }
                    }
                    span { class: "sr-only", "Toggle Menu" }
                }
                div { class: "flex flex-1 items-center justify-between gap-2 md:justify-end",

                    // Icons
                    nav { class: "navigation-icons",

                        a {
                            class: "navigation-menu-item",
                            "hx-boost": "true",
                            href: docs::Index {}.to_string(),
                            "Documentation"
                        }
                        // GitHub Icon
                        a {
                            class: "ml-4",
                            target: "_blank",
                            rel: "noreferrer",
                            href: "https://github.com/supakube/supakube",
                            img { src: "https://img.shields.io/github/stars/supakube/supakube" }
                            span { class: "sr-only", "GitHub" }
                        }
                    }
                }
            }
        }
    }
}
