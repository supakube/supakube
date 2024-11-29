use crate::routes::{blog, docs, marketing};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Section {
    None,
    Home,
    Enterprise,
    Partners,
    Pricing,
    Blog,
    Docs,
    Contact,
}

#[component]
pub fn NavItem(link: String, name: String, section: Section, current_section: Section) -> Element {
    let mut class = "";
    if section == current_section {
        class = "active";
    }
    rsx!(
        a { 
            class: format!("navigation-menu-item {}", class), 
            "hx-boost": "true",
            href: link, 
            "{name}" 
        } 
    )
}

#[component]
pub fn Navigation(mobile_menu: Element, section: Section) -> Element {
    rsx! {
        header { class: "navigation",
            div {
                class: "navigation-container",
                div {
                    class: "navigation-logo",
                    a {
                        class: "navigation-logo-link", 
                        href: marketing::Index {}.to_string(),
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
                    nav { 
                        class: "navigation-menu",
                        NavItem {
                            link: marketing::EnterprisePage {}.to_string(),
                            name: "Enterprise".to_string(),
                            section: Section::Enterprise,
                            current_section: section.clone(),
                        }
                        NavItem {
                            link: marketing::Pricing {}.to_string(),
                            name: "Pricing".to_string(),
                            section: Section::Pricing,
                            current_section: section.clone(),
                        }
                        NavItem {
                            link: blog::Index {}.to_string(),
                            name: "Blog".to_string(),
                            section: Section::Blog,
                            current_section: section.clone(),
                        }
                        NavItem {
                            link: marketing::Contact {}.to_string(),
                            name: "Contact Us".to_string(),
                            section: Section::Contact,
                            current_section: section.clone(),
                        }
                        NavItem {
                            link: marketing::PartnersPage {}.to_string(),
                            name: "Partners".to_string(),
                            section: Section::Partners,
                            current_section: section.clone(),
                        }
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
                    nav { 
                        class: "navigation-icons",

                        NavItem {
                            link: docs::Index {}.to_string(),
                            name: "Documentation".to_string(),
                            section: Section::Docs,
                            current_section: section.clone(),
                        }
                        // GitHub Icon
                        a {
                            class: "ml-4 mr-4",
                            target: "_blank",
                            rel: "noreferrer",
                            href: "https://github.com/supakube/supakube",
                            img { src: "https://img.shields.io/github/stars/supakube/supakube" }
                            span { class: "sr-only", "GitHub" }
                        }
                        theme-switch {

                        }
                    }
                }
            }
        }
    }
}
