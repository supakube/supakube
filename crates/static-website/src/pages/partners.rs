use crate::components::benefits::Benefits;
use crate::components::features::Features;
use crate::components::footer::Footer;
use crate::components::image_hero::ImageHero;
use crate::components::navigation::Section;
use crate::components::testamonials::Testamonials;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

#[component]
pub fn PartnersPage() -> Element {
    rsx! {
        Layout {
            title: "Partners",
            mobile_menu: None,
            description: "Partners",
            section: Section::Partners,
            div {
                class: "mt-24 flex flex-col items-center",

                ImageHero {
                    title: "Become a Bionic-GPT Partner",
                    subtitle: "Unlock Revenue with Secure, Enterprise-Grade AI Solutions"
                }

                Benefits {
                    title: "Partners",
                    subtitle: "Why Partner with Us?",
                    benefit1: "Enter a growing Market",
                    benefit1_desc: "As a bionicGPT partner, you can tap into a growing market of enterprises seeking safe, private, and powerful AI solutionse",
                    benefit2: "Enterprise Ready",
                    benefit2_desc: "AI platform designed for flexibility, compliance, and scalability.",
                    benefit3: "On Prem or Private Cloud",
                    benefit3_desc: "Enabling enterprises to leverage the power of generative AI within the secure confines of their own infrastructure.",
                }

                Features {}

                Testamonials {}

                section {
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
