use crate::components::footer::Footer;
use crate::components::image_hero::ImageHero;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

#[component]
pub fn PartnersPage() -> Element {
    rsx! {
        Layout {
            title: "Partners",
            mobile_menu: None,
            description: "Partners",
            section {
                class: "mt-24 flex flex-col items-center",

                ImageHero {
                    title: "Become a bionicGPT Partner",
                    subtitle: "Unlock Revenue with Secure, Enterprise-Grade AI Solutions"
                }

                div {
                    class: "w-full lg:w-3/4 lg:max-w-3xl mx-auto px-4 md:px-6 lg:px-8 text-left",
                    img {
                        src: "/landing-page/partners-bionic.png",
                        alt: "bionicGPT Partnership",
                        class: "mx-auto mt-4 mb-6 w-1/2", // Centers the image and sets it to 50% width of the container
                    }
                    p {
                        class: "mt-4 mb-6",
                        "At bionicGPT, we offer a unique opportunity to partner with a secure, enterprise-ready generative AI platform designed for flexibility, compliance, and scalability. Our solution is deployable on-premise or in your private cloud, enabling enterprises to leverage the power of generative AI within the secure confines of their own infrastructure."
                    }
                    h4 {
                        class: "text-2xl font-bold mt-8",
                        "Why Partner with Us?"
                    }
                    p {
                        class: "mt-4 mb-6",
                        "As a bionicGPT partner, you can tap into a growing market of enterprises seeking safe, private, and powerful AI solutions. Our platform’s features, including no-code RAG pipelines, team-based permissions, full observability, and customizable rate limiting, making it an ideal fit for security-conscious businesses and organisations in highly regulated sectors. And, with your local expertise and support, you can transform these capabilities into tangible value for your clients."
                    }
                    h4 {
                        class: "text-2xl font-bold mt-8",
                        "A Success Story"
                    }
                    p {
                        class: "mt-4 mb-6",
                        "Based in Washington, D.C., ",
                        a {
                            href: "https://gtedge.ai",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "GTEdge AI"
                        },
                        " has been a trusted partner of bionicGPT for over a year, delivering successful installations for multiple clients. By leveraging bionicGPT, GTEdge AI offers a comprehensive turnkey solution deployed directly in their data center. This partnership has enabled GTEdge AI to enhance and expand their AI training and consultancy services."
                    }

                    h4 {
                        class: "text-2xl font-bold mt-8",
                        "Partner Benefits"
                    }
                    ul {
                        class: "list-disc list-inside mt-4 mb-6",
                        li { class: "mt-2", strong { "Revenue Growth:" }, " Earn from licensing new users, support, and upgrades, while also providing AI consulting, training, and development services." }
                        li { class: "mt-2", strong { "In-Demand Solution:" }, " Our platform’s private, secure deployment model opens doors to businesses prioritising data privacy and compliance." }
                        li { class: "mt-2", strong { "End-to-End Support:" }, " Get onboarding assistance and ongoing technical support to ensure a seamless experience for you and your clients." }
                        li { class: "mt-2", strong { "Flexible Deployments:" }, " Offer clients flexible deployment options, including on-premise or private cloud, for total control over data and security." }
                    }
                    div {
                        class: "mt-10 flex flex-col items-center",
                        hr { class: "w-full mb-4" }
                        a {
                            href: "/contact",
                            class: "btn btn-secondary btn-outline",
                            "Book a Call"
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
