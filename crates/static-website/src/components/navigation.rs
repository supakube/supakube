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
                        img {
                            alt: "Logo",
                            width: "22",
                            height: "22",
                            src: "/logo.svg",
                            class: "navigation-logo-icon",
                        }
                        span { class: "navigation-logo-text", "Supakube" }
                    }
                    nav { class: "navigation-menu",
                        a { class: "navigation-menu-item", href: marketing::Pricing {}.to_string(), "Pricing" }
                        a { class: "navigation-menu-item", href: docs::Index {}.to_string(), "Documentation" }
                        a { class: "navigation-menu-item", href: blog::Index {}.to_string(), "Blog" }
                        a { class: "navigation-menu-item", href: marketing::PartnersPage {}.to_string(), "Partners" }
                        a { class: "navigation-menu-item", href: marketing::ServicesPage {}.to_string(), "Services" }
                        a { class: "navigation-menu-item", href: marketing::Contact {}.to_string(), "Contact Us" }
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
                        // GitHub Icon
                        a {
                            target: "_blank",
                            rel: "noreferrer",
                            href: "https://github.com/supakube/supakube",
                            img { src: "https://img.shields.io/github/stars/supakube/supakube" }
                            span { class: "sr-only", "GitHub" }
                        }
                        // Theme Toggle Button
                        button { class: "navigation-icon-button",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "hidden [html.dark_&]:block",
                                // Sun Icon SVG content
                                circle { cx: "12", cy: "12", r: "4" }
                                path { d: "M12 2v2" }
                                path { d: "M12 20v2" }
                                path { d: "m4.93 4.93 1.41 1.41" }
                                path { d: "m17.66 17.66 1.41 1.41" }
                                path { d: "M2 12h2" }
                                path { d: "M20 12h2" }
                                path { d: "m6.34 17.66-1.41 1.41" }
                                path { d: "m19.07 4.93-1.41 1.41" }
                            }
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "hidden [html.light_&]:block",
                                // Moon Icon SVG content
                                path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                            }
                            span { class: "sr-only", "Toggle theme" }
                        }
                    }
                }
            }
        }
    }
}
