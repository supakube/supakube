use dioxus::prelude::*;

#[component]
pub fn River(
    title: String,
    sub_title: String,
    text: String,
    img: String
) -> Element {
    rsx! {
        section { 
            class: "flex flex-row lg:max-w-5xl",
            div {
                class: "flex-1"
            }
            div {
                class: "flex-1"
            }
        }
    }
}
