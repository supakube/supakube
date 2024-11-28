use dioxus::prelude::*;

#[component]
pub fn Features() -> Element {
    rsx! {
        section {
            class: "lg:max-w-5xl body-font mt-24",
            div {
                class: "py-8 px-4 mx-auto max-w-screen-xl sm:py-16 lg:px-6",
                div {
                    class: "max-w-screen-md mb-8 lg:mb-16",
                    h2 {
                        class: "mb-4 text-4xl tracking-tight font-extrabold text-gray-900 dark:text-white",
                        "Designed for business teams like yours"
                    }
                    p {
                        class: "text-gray-500 sm:text-xl dark:text-gray-400",
                        "Here at Flowbite we focus on markets where technology, innovation, and capital can unlock long-term value and drive economic growth."
                    }
                }
                div {
                    class: "space-y-8 md:grid md:grid-cols-2 lg:grid-cols-3 md:gap-12 md:space-y-0",
                    for i in 0..6 {
                        div {
                            div {
                                class: "flex justify-center items-center mb-4 w-10 h-10 rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900",

                                match i {
                                    0 => rsx!(img { src: "/feature-icons/marketing.svg" }),
                                    1 => rsx!(img { src: "/feature-icons/marketing.svg" }),
                                    2 => rsx!(img { src: "/feature-icons/marketing.svg" }),
                                    3 => rsx!(img { src: "/feature-icons/marketing.svg" }),
                                    4 => rsx!(img { src: "/feature-icons/marketing.svg" }),
                                    5 => rsx!(img { src: "/feature-icons/marketing.svg" }),
                                    _ => rsx!()
                                }
                            }

                            h3 {
                                class: "mb-2 text-xl font-bold dark:text-white",
                                match i {
                                    0 => "Marketing",
                                    1 => "Legal",
                                    2 => "Business Automation",
                                    3 => "Finance",
                                    4 => "Enterprise Design",
                                    5 => "Operations",
                                    _ => ""
                                }
                            }
                            p {
                                class: "text-gray-500 dark:text-gray-400",
                                match i {
                                    0 => "Plan it, create it, launch it. Collaborate seamlessly...",
                                    1 => "Protect your organization, devices and stay compliant...",
                                    2 => "Auto-assign tasks, send Slack messages, and much more...",
                                    3 => "Audit-proof software built for critical financial...",
                                    4 => "Craft beautiful, delightful experiences for both...",
                                    5 => "Keep your company’s lights on with customizable...",
                                    _ => ""
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
