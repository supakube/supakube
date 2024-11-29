use dioxus::prelude::*;

#[component]
pub fn ContactCard(img: String, name: String, role: String) -> Element {
    rsx! {
        div {
            class: "p-2 lg:w-1/3 md:w-1/2 w-full",
            div {
                class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                img {
                    alt: "team",
                    class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                    src: "{img}",
                }
                div {
                    class: "flex-grow",
                    h2 {
                        class: "font-medium",
                        "{name}"
                    }
                    p {
                        class: "text-gray-500",
                        "{role}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Team() -> Element {
    rsx! {
        section {
            class: "lg:max-w-5xl mx-auto",
            div {
                class: "container py-24 mx-auto",
                div {
                    class: "flex flex-col text-center w-full mb-20",
                    h1 {
                        class: "sm:text-3xl text-2xl font-medium mb-4",
                        "Our Team"
                    }
                    p {
                        class: "lg:w-2/3 mx-auto leading-relaxed",
                        "Whatever cardigan tote bag tumblr hexagon brooklyn asymmetrical gentrify, subway tile poke farm-to-table. Franzen you probably haven't heard of them."
                    }
                }
                div {
                    class: "flex flex-wrap -m-2",ContactCard {
                        name: "Kulbinder Dio".to_string(),
                        role: "UI Designer".to_string(),
                        img: "https://dummyimage.com/80x80".to_string()
                    }
                    ContactCard {
                        name: "Ian Purton".to_string(),
                        role: "CTO".to_string(),
                        img: "https://dummyimage.com/80x80".to_string()
                    }
                    ContactCard {
                        name: "Oskar Blinde".to_string(),
                        role: "Founder".to_string(),
                        img: "https://dummyimage.com/80x80".to_string()
                    }
                    ContactCard {
                        name: "John Doe".to_string(),
                        role: "DevOps".to_string(),
                        img: "https://dummyimage.com/90x90".to_string()
                    }
                    ContactCard {
                        name: "Martin Eden".to_string(),
                        role: "Software Engineer".to_string(),
                        img: "https://dummyimage.com/94x94".to_string()
                    }
                    ContactCard {
                        name: "Boris Kitua".to_string(),
                        role: "UX Researcher".to_string(),
                        img: "https://dummyimage.com/98x98".to_string()
                    }
                    ContactCard {
                        name: "Atticus Finch".to_string(),
                        role: "QA Engineer".to_string(),
                        img: "https://dummyimage.com/100x90".to_string()
                    }
                    ContactCard {
                        name: "Alper Kamu".to_string(),
                        role: "System".to_string(),
                        img: "https://dummyimage.com/104x94".to_string()
                    }
                    ContactCard {
                        name: "Rodrigo Monchi".to_string(),
                        role: "Product Manager".to_string(),
                        img: "https://dummyimage.com/108x98".to_string()
                    }
                    
                }
            }
        }
    }
}
