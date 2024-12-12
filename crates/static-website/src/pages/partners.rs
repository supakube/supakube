use crate::components::footer::Footer;
use crate::components::navigation::Section;
use crate::layouts::layout::Layout;
use daisy_rsx::marketing::benefits::Benefits;
use daisy_rsx::marketing::features::{Feature, Features};
use daisy_rsx::marketing::hero::Hero;
use daisy_rsx::marketing::testamonials::Testamonials;
use dioxus::prelude::*;

pub fn partners_page() -> String {
    let titles = &[
        "No Code Rag",
        "Team-based permissions",
        "Full Observability",
        "Rate limiting",
        "Military Grade Security",
        "Operations",
    ];

    let descriptions = &[
        "Including no-code RAG pipelines",
        "Data is siloed at the tema level",
        "Auto-assign tasks, send Slack messages, and much more...",
        "Audit-proof software built for critical financial...",
        "Craft beautiful, delightful experiences for both...",
        "Keep your company’s lights on with customizable...",
    ];

    let features: Vec<Feature> = titles
        .iter()
        .zip(descriptions.iter())
        .map(|(title, description)| Feature {
            title: title.to_string(),
            description: description.to_string(),
        })
        .collect();

    let page = rsx! {
        Layout {
            title: "Partners",
            mobile_menu: None,
            description: "Partners",
            section: Section::Partners,
            div {
                class: "p-5 mt-24 flex flex-col items-center",

                Hero {
                    title: "Become a Bionic-GPT Partner",
                    subtitle: "Unlock Revenue with Secure, Enterprise-Grade AI Solutions",
                    cta: "saf",
                    cta_link: "saf"
                }

                Benefits {
                    title: "Partners",
                    subtitle: "Why Partner with Us?",
                    benefit1: "Revenue Growth",
                    benefit1_desc: "Earn from licensing new users, support, and upgrades,
                        while also providing AI consulting, training, and development services.",
                    benefit2: "In-Demand Solution",
                    benefit2_desc: "Our platform’s private, secure deployment model opens doors
                        to businesses prioritising data privacy and compliance.",
                    benefit3: "End-to-End Support",
                    benefit3_desc: "Get onboarding assistance and ongoing technical
                        support to ensure a seamless experience for you and your clients.",
                }

                Features {
                    title: "Bionic-GPT Features",
                    description: "As a bionicGPT partner, you can tap into a growing market of businesses seeking safe,
                        private, and powerful AI solutions.",
                    features
                }

                Testamonials {
                    text1: "Having the flexibility to use the best model for the job has been a game-changer. Bionic-GPT’s support for multiple models ensures we can tailor solutions to specific challenges, delivering optimal results every time.",
                    job1: "Data Scientist",
                    person1: "Emma Trident",
                    img1: "",
                    text2: "Bionic-GPT’s observability feature, which logs all messages into and out of the models, has been critical for ensuring compliance in our organization. It gives us peace of mind and robust accountability.",
                    job2: "Compliance Officer",
                    person2: "Patrick O'leary",
                    img2: "",
                }

                section {
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
    };

    crate::render(page)
}
