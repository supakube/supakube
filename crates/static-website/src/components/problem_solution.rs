use dioxus::prelude::*;

#[component]
pub fn ProblemSolution(title: String, subtitle: String) -> Element {
    rsx! {
        section {
            class: "mt-24 flex lg:max-w-5xl ",
            div {
                class: "flex-1 text-center",
                img {
                    width: "100%",
                    height: "100%",
                    src: "/temporary-image.svg"
                }
            }
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
        }
    }
}
