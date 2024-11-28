use std::fs::{self, File};
use std::io::Write;

use super::image_hero::ImageHero;
use crate::components::extra_footer::ExtraFooter;
use crate::components::footer::Footer;
use crate::components::image_feature::ImageFeature;
use crate::components::customer_logos::Partners;
use crate::layouts::layout::Layout;
use crate::routes::marketing::Index;
use axum::response::Html;
use axum::Router;
use axum_extra::routing::RouterExt;
use dioxus::prelude::*;

pub fn routes() -> Router {
    Router::new().typed_get(index)
}

pub async fn generate() {
    let html = crate::render(HomePage).await;

    let mut file = File::create("dist/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = crate::render(Pricing).await;

    fs::create_dir_all("dist/pricing").expect("Couyldn't create folder");
    let mut file = File::create("dist/pricing/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = crate::render(PartnersPage).await;

    fs::create_dir_all("dist/partners").expect("Couyldn't create folder");
    let mut file = File::create("dist/partners/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = crate::render(ServicesPage).await;

    fs::create_dir_all("dist/services").expect("Couyldn't create folder");
    let mut file = File::create("dist/services/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = crate::render(ContactPage).await;

    fs::create_dir_all("dist/contact").expect("Couyldn't create folder");
    let mut file = File::create("dist/contact/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

pub async fn index(Index {}: Index) -> Html<String> {
    let html = crate::render(HomePage).await;

    Html(html)
}

#[component]
pub fn Pricing() -> Element {
    rsx! {
        Layout {
            title: "Pricing",
            description: "Bionic Pricing",
            mobile_menu: None,
            div {
                div { class: "mt-12 mx-auto max-w-7xl px-6 lg:px-8",
                    div { class: "mx-auto max-w-2xl sm:text-center",
                        h2 { class: "text-3xl font-bold tracking-tight sm:text-4xl", "Pricing" }
                        p { class: "mt-6 text-lg leading-8",
                            "\n          Bionic works best when it's integrated with your systems.\n          We offer packages to integrate Bionic with your Operations, Compliance and Security. \n        "
                        }
                    }
                }
            }
            div {
                class: "mx-auto lg:flex lg:flex-row justify-center gap-4 lg:max-w-5xl w-full p-5",
                div {
                    class: "card card-bordered lg:w-1/3",
                    div {
                        class: "card-body flex flex-col justify-between list-tick",
                        div {
                            class: "flex flex-col gap-3",
                            h3 { class: "card-title", "Cloud" }
                            span { class: "badge badge-primary badge-outline", "Free" }
                            p { "Ideal for evaluating Bionic and testing RAG use cases." }
                            h4 { class: "font-extrabold", "Generative AI" }
                            ul {
                                li { "Code and Text Generation" }
                                li { "Role based access control" }
                                li { "Chat History" }
                            }
                            h4 { class: "font-extrabold", "Retrieval Augmented Generation" }
                            ul {
                                li { "All major document formats processed." }
                                li { "Automated chunking and vector generation." }
                            }
                            h4 { class: "font-extrabold", "Open AI Compatible API" }
                            ul {
                                li { "Track and monitor API usage." }
                                li { "Share models between projects." }
                                li { "Key creation and revocation." }
                            }
                            h4 { class: "font-extrabold", "Document Pipelines" }
                            ul {
                                li { "Batch or stream document processing." }
                                li { "100s of sources." }
                            }
                        }
                        div { class: "mt-5 flex flex-col gap-2",
                            hr {}
                            h3 { class: "font-extrabold", "Free" }
                            a {
                                href: "https://app.bionic-gpt.com",
                                class: "btn btn-secondary btn-outline",
                                "\n            Get Started\n          "
                            }
                        }
                    }
                }
            }
            ExtraFooter {
                title: "The secure open source Chat-GPT replacement
                that runs in a trusted execution environment for
                maximum data security and compliance",
                image: "/landing-page/bionic-console.png",
                cta: "Find out more",
                cta_url: crate::routes::marketing::Index {}.to_string()
            }
        }
        Footer {}
    }
}

#[component]
pub fn PartnersPage() -> Element {
    rsx! {
        Layout {
            title: "Partners",
            mobile_menu: None,
            description: "Partners",
            section {
                class: "mt-12 mb-12 mx-auto prose lg:prose-xl justify-center px-4", // Add padding to the section
                div {
                    class: "w-full lg:w-3/4 lg:max-w-3xl mx-auto px-4 md:px-6 lg:px-8 text-left", // Adjust max width and add padding at multiple screen sizes
                    h2 {
                        class: "text-4xl font-extrabold mt-4 text-center",
                        "Become a bionicGPT Partner"
                    }
                    img {
                        src: "/landing-page/partners-bionic.png",
                        alt: "bionicGPT Partnership",
                        class: "mx-auto mt-4 mb-6 w-1/2", // Centers the image and sets it to 50% width of the container
                    }
                    h4 {
                        class: "text-2xl font-bold mt-8",
                        "Unlock Revenue with Secure, Enterprise-Grade AI Solutions"
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

#[component]
pub fn ServicesPage() -> Element {
    rsx! {
        Layout {
            title: "Services",
            mobile_menu: None,
            description: "Services",
            section {
                class: "mt-12 mb-12 mx-auto prose lg:prose-xl justify-center px-4", // Add padding to the section
                div {
                    class: "w-full lg:w-3/4 lg:max-w-3xl mx-auto px-4 md:px-6 lg:px-8 text-left", // Adjust max width and add padding at multiple screen sizes
                    h2 {
                        class: "text-4xl font-extrabold mt-4 text-center",
                        "bionicGPT Services"
                    }
                    p {
                        class: "mt-4 mb-6",
                        "At bionicGPT, we offer a comprehensive suite of services designed to maximize your organisation’s AI potential, from foundational training to advanced, custom AI solutions tailored to your needs."
                    }
                    h4 {
                        class: "text-2xl font-bold mt-8",
                        "AI Training"
                    }
                    p {
                        class: "mt-4 mb-6",
                        "Empower your team with the skills to harness generative AI confidently and effectively. Our training covers both general AI knowledge and specific product training on bionicGPT, ensuring that your team can use the platform to its fullest potential. Whether your team is new to AI or looking to advance their expertise, we provide insights into AI-driven data workflows, and secure deployment. Our hands-on sessions transform AI concepts into practical applications, giving your team the tools to integrate generative AI and ensure data security and compliance."
                    }
                    h4 {
                        class: "text-2xl font-bold mt-8",
                        "AI Consulting"
                    }
                    p {
                        class: "mt-4 mb-6",
                        "Our AI Consulting services focus on transforming your specific business needs into effective AI solutions. We work closely with your team to identify opportunities, design strategies, and integrate AI in ways that drive real value. From feasibility studies to model selection and data management, we’re here to guide you through each step, ensuring your AI initiatives align with industry best practices and regulatory standards."
                    }
                    h4 {
                        class: "text-2xl font-bold mt-8",
                        "AI Development"
                    }
                    p {
                        class: "mt-4 mb-6",
                        "Our team specialises in custom AI development, tailored extensions to bionicGPT, and powerful AI agents. From enhancing existing functionalities to creating bespoke AI workflows, we help you deploy highly effective solutions that fit seamlessly into your tech stack. We also provide support for integrating AI agents that automate tasks, streamline data handling, and optimize operations. With bionicGPT as your foundation, you can scale up securely and efficiently with innovations that align perfectly with your goals."
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
            Footer {}
        }
    }
}

#[component]
pub fn ContactPage() -> Element {
    rsx! {
        Layout {
            title: "Enterprise Generative AI",
            mobile_menu: None,
            description: "The Industry Standard For Enterprise Generative AI",
            section {
                class: "mt-12 text-center mb-12",
                h1 {
                    class: "text-4xl font-extrabold mt-4",
                    "Our Team is Waiting to Hear From You"
                }
                h2 {
                    class: "text-2xl font-bold mt-4",
                    "Contact the Experts in Gen AI Deployments"
                }
                p {
                    class: "font-bold mt-4",
                    "Email founders (at) bionic-gpt.com"
                }
                p {
                    class: "mt-4 mb-4",
                    "Or Schedule a Meeting with Calendly"
                }
                a {
                    class: "btn btn-primary",
                    href: "https://calendly.com/bionicgpt",
                    "Schedule a Meeting"
                }
            }
            ExtraFooter {
                title: "The secure open source Chat-GPT replacement
                that runs in a trusted execution environment for
                maximum data security and compliance",
                image: "/landing-page/bionic-console.png",
                cta: "Find out more",
                cta_url: crate::routes::marketing::Index {}.to_string()
            }
            Footer {}
        }
    }
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Layout {
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,

            div {
                class: "mt-12 flex flex-col items-center",
                ImageHero {
                    title: "Generative AI Private Data",
                    subtitle: "We use hardware based confidential computing to
                        run AI in a highly secure enclave for maximum 
                        protection of your data in the cloud or on premise"
                }
                Partners {}

                ImageFeature {
                    title: "Data Governance",
                    sub_title: "A Chat-GPT Replacement Without The Data Leakage",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    title1: "Regulatory Compliance.",
                    text1: "Run Generative AI and become compliant with GDPR, CCPA, PIPEDA, POPI, LGPD, HIPAA, PCI-DSS, and More",
                    title2: "Chat Console.",
                    text2: "A familiar chat console with text and code generation and the ability to select an assistant tuned on your data.",
                    title3: "Data Governance.",
                    text3: "By deploying Bionic close to your data you are able to benefit from Generative AI
        and still conform to data privacy and controls.",
                    image: "/landing-page/bionic-console.png",
                }
            }
            Footer {}
        }
    }
}
