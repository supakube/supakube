use dioxus::prelude::*;

use super::layout::Layout;
use crate::{
    components::navigation::Section,
    generator::{Category, Page, Summary},
};

#[component]
pub fn Document(summary: Summary, category: Category, doc: Page) -> Element {
    rsx! {
        Layout {
            title: "{doc.title}",
            description: "{doc.description}",
            section: Section::Docs,
            mobile_menu: rsx! (MobileMenu {
                summary: summary.clone()
            }),
            main {
                class: "flex-1",

                div {
                    class: "mt-12 flex flex-row",
                    LeftNav {
                        summary
                    }
                    Content {
                        doc
                    }
                }
            }
        }
    }
}

#[component]
fn MobileMenu(summary: Summary) -> Element {
    rsx! {
        for category in &summary.categories {
            ul {
                for page in &category.pages {
                    li {
                        a {
                            href: "/{page.folder}",
                            "{page.title}",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LeftNav(summary: Summary) -> Element {
    rsx! {
        div {
            class: "h-[calc(100vh-68px)] hidden lg:flex pl-4",
            nav {
                class: "h-[calc(100vh-86px)] overflow-scroll p-3",
                for category in &summary.categories {
                    p {
                        class: "font-semibold mb-2",
                        "{category.name}"
                    }
                    ul {
                        class: "mb-6",
                        for page in &category.pages {
                            li {
                                class: "mb-2",
                                a {
                                    class: "rounded-md hover:text-sky-500 dark:hover:text-sky-400",
                                    href: "/{page.folder}",
                                    "hx-boost": "true",
                                    "{page.title}"
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}

#[component]
fn Content(doc: Page) -> Element {
    let content = crate::markdown::markdown_to_html(doc.markdown);
    rsx! {
        section {
            class: "mt-2 overflow-scroll h-[calc(100vh-68px)] w-full",
            div {
                class: "mb-12",
                article {
                    class: "mx-auto prose",
                    div {
                        dangerous_inner_html: "{content}"
                    }
                }
            }
        }
    }
}
