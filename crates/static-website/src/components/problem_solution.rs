use dioxus::prelude::*;

#[component]
pub fn ProblemSolution(image: String, title: String, subtitle: String) -> Element {
    rsx! {
        section {
            class: "mt-24 flex lg:max-w-5xl gap-8",
            div {
                class: "flex-1",
                h1 {
                    class: "text-2xl font-bold",
                    "{title}"
                }
                p {
                    class: "py-6",
                    "{subtitle}"
                }
            }
            div {
                class: "flex-1",
                img {
                    loading: "lazy",
                    class: "rounded-xl ring-1 ring-gray-400/10 lg:max-w-2xl",
                    alt: "Product screenshot",
                    src: "{image}",
                }
            }
        }
    }
}
