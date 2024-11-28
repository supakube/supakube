use dioxus::prelude::*;

#[component]
pub fn Benefits(benefit1: String, benefit1_desc: String) -> Element {
    rsx! {
        section {
            class: "mt-24 flex lg:max-w-5xl gap-8",
            div {
                class: "flex-1",
                h1 {
                    class: "text-xl font-bold",
                    "{benefit1}"
                }
                p {
                    class: "py-6",
                    "{benefit1_desc}"
                }
            }
            div {
                class: "flex-1",
                h1 {
                    class: "text-xl font-bold",
                    "{benefit1}"
                }
                p {
                    class: "py-6",
                    "{benefit1_desc}"
                }
            }
            div {
                class: "flex-1",
                h1 {
                    class: "text-xl font-bold",
                    "{benefit1}"
                }
                p {
                    class: "py-6",
                    "{benefit1_desc}"
                }
            }
        }
    }
}
