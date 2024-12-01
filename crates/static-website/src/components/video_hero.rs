use dioxus::prelude::*;

use crate::routes::marketing;

#[component]
pub fn ImageHero(title: String, subtitle: String, video: String, claim: String) -> Element {
    rsx! {
        section {
            class: "md:flex flex-row lg:max-w-5xl gap-8",
            div {
                div {
                    h1 {
                        class: "text-5xl font-bold",
                        "{title}"
                    }
                    p {
                        class: "py-6",
                        "{subtitle}"
                    }
                    div {
                        a {
                            class: "btn btn-primary",
                            href: marketing::Contact {}.to_string(),
                            "Book a Demo"
                        }
                        strong {
                            class: "ml-4",
                            "{claim}"
                        }
                    }
                }
            }
            div {
                iframe {
                    width: "560",
                    height: "315",
                    src: video,
                    title: "YouTube video player",
                    "frameborder": "0",
                    allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                    referrerpolicy: "strict-origin-when-cross-origin",
                    allowfullscreen: "true"
                }
            }
        }
    }
}
