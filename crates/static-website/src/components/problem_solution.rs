use dioxus::prelude::*;

#[component]
pub fn ProblemSolution(video: String, title: String, subtitle: String) -> Element {
    rsx! {
        section {
            class: "mt-24 flex lg:max-w-5xl gap-8",
            div {
                class: "",
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
            div {
                class: "",
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
