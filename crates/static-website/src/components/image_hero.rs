use dioxus::prelude::*;

#[component]
pub fn ImageHero(title: String, subtitle: String) -> Element {
    rsx! {
        section {
            div {
                class: "text-center",
                div {
                    class: "max-w-md",
                    h1 {
                        class: "text-5xl font-bold",
                        "{title}"
                    }
                    p {
                        class: "py-6",
                        "{subtitle}"
                    }
                    div {
                        class: "flex gap-2 justify-center",
                        a {
                            class: "btn btn-primary",
                            href: "{crate::routes::SIGN_IN_UP}",
                            "Get started with Cloud Edition"
                        }
                        a {
                            class: "btn btn-secondary btn-outline",
                            href: crate::routes::marketing::Contact {}.to_string(),
                            "Schedule a Meeting"
                        }
                    }
                }
            }
        }
    }
}
