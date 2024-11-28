use std::fs::{self, File};
use std::io::Write;

use crate::components::extra_footer::ExtraFooter;
use crate::components::footer::Footer;
use crate::components::team::Team;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

use super::partners::PartnersPage;

pub async fn generate() {
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

    let html = crate::render(ContactPage).await;

    fs::create_dir_all("dist/contact").expect("Couyldn't create folder");
    let mut file = File::create("dist/contact/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

#[component]
pub fn Pricing() -> Element {
    rsx! {
        Layout {
            title: "Pricing",
            description: "Bionic Pricing",
            mobile_menu: None,
            div {
                div { class: "mt-24 mx-auto max-w-7xl px-6 lg:px-8",
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
                div { class: "card card-bordered lg:w-1/3",
                    div { class: "card-body flex flex-col justify-between list-tick",
                        div { class: "flex flex-col gap-3",
                            h3 { class: "card-title", "Encrypted Cloud" }
                            span { class: "badge badge-primary badge-outline", "Secure Compute" }
                            p { "Runs in our secure enclave with end to end encryption" }
                            h4 { class: "font-extrabold", "Everything from Cloud Edition and..." }
                            ul {
                                li { "Secure enclave running CPU resources." }
                                li { "Secure enclave running GPU resources." }
                                li { "Fully siloed and security hardenend." }
                            }
                            h4 { class: "font-extrabold", "Maximum Data Protection" }
                            ul {
                                li { "Built for running Generative AI on highly confidential data." }
                                li {
                                    "Hardware based secure compute thanks to our partnership with Nvidia and Google."
                                }
                                li { "Bring/Hold your own keys." }
                                li { "Provable supply chain with server attestation." }
                            }
                        }
                        div { class: "mt-5 flex flex-col gap-2",
                            hr {}
                            h3 { class: "font-extrabold", "Custom Pricing" }
                            a {
                                href: "/contact",
                                class: "btn btn-secondary btn-outline",
                                "\n            Book a Call\n          "
                            }
                        }
                    }
                }
                div { class: "card card-bordered lg:w-1/3",
                    div { class: "card-body flex flex-col justify-between list-tick",
                        div { class: "flex flex-col gap-3",
                            h3 { class: "card-title", "On Premise / Private Cloud" }
                            span { class: "badge badge-primary badge-outline", "Enterprise Edition" }
                            p { "On Premise or Private Cloud" }
                            h4 { class: "font-extrabold",
                                "Everything from Cloud Edition and Encrypted Cloud..."
                            }
                            ul {
                                li { "Maximum privacy and security." }
                                li { "Support for running on bare metal." }
                                li { "Single Sign On." }
                            }
                            h4 { class: "font-extrabold", "Support" }
                            ul {
                                li { "Hardware recommendations." }
                                li { "Possibility of custom builds." }
                                li { "Dedicated customer success and engineering resources." }
                                li { "Custom Integrations." }
                                li { "Custom SLAs and support." }
                            }
                        }
                        div { class: "mt-5 flex flex-col gap-2",
                            hr {}
                            h3 { class: "font-extrabold", "Custom Pricing" }
                            a {
                                href: "/contact",
                                class: "btn btn-secondary btn-outline",
                                "\n            Book a Call\n          "
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
                class: "mt-24 text-center mb-12",
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
            Team {

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
