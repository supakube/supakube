use dioxus::prelude::*;

#[component]
pub fn Team() -> Element {
    rsx! {
        section {
            class: "lg:max-w-5xl mx-auto text-gray-600 body-font",
            div {
                class: "container py-24 mx-auto",
                div {
                    class: "flex flex-col text-center w-full mb-20",
                    h1 {
                        class: "sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900",
                        "Our Team"
                    }
                    p {
                        class: "lg:w-2/3 mx-auto leading-relaxed text-base",
                        "Whatever cardigan tote bag tumblr hexagon brooklyn asymmetrical gentrify, subway tile poke farm-to-table. Franzen you probably haven't heard of them."
                    }
                }
                div {
                    class: "flex flex-wrap -m-2",
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/80x80",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "Holden Caulfield"
                                }
                                p {
                                    class: "text-gray-500",
                                    "UI Designer"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/84x84",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "Henry Letham"
                                }
                                p {
                                    class: "text-gray-500",
                                    "CTO"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/88x88",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "Oskar Blinde"
                                }
                                p {
                                    class: "text-gray-500",
                                    "Founder"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/90x90",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "John Doe"
                                }
                                p {
                                    class: "text-gray-500",
                                    "DevOps"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/94x94",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "Martin Eden"
                                }
                                p {
                                    class: "text-gray-500",
                                    "Software Engineer"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/98x98",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "Boris Kitua"
                                }
                                p {
                                    class: "text-gray-500",
                                    "UX Researcher"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/100x90",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "Atticus Finch"
                                }
                                p {
                                    class: "text-gray-500",
                                    "QA Engineer"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/104x94",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "Alper Kamu"
                                }
                                p {
                                    class: "text-gray-500",
                                    "System"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-2 lg:w-1/3 md:w-1/2 w-full",
                        div {
                            class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                            img {
                                alt: "team",
                                class: "w-16 h-16 bg-gray-100 object-cover object-center flex-shrink-0 rounded-full mr-4",
                                src: "https://dummyimage.com/108x98",
                            }
                            div {
                                class: "flex-grow",
                                h2 {
                                    class: "text-gray-900 title-font font-medium",
                                    "Rodrigo Monchi"
                                }
                                p {
                                    class: "text-gray-500",
                                    "Product Manager"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
