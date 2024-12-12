use crate::components::footer::Footer;
use crate::components::navigation::Section;
use crate::layouts::layout::Layout;
use daisy_rsx::marketing::benefits::Benefits;
use daisy_rsx::marketing::security::Security;
use dioxus::prelude::*;
use std::fs::File;
use std::io::Write;
use daisy_rsx::marketing::customer_logos::Customers;
use daisy_rsx::marketing::faq_accordian::{Faq, FaqText};
use daisy_rsx::marketing::features::{Feature, Features};
use daisy_rsx::marketing::problem_solution::ProblemSolution;
use daisy_rsx::marketing::small_image_feature::SmallImageFeature;
use daisy_rsx::marketing::testamonials::Testamonials;
use daisy_rsx::marketing::video_hero::VideoHero;

pub async fn generate() {
    let html = home_page();

    let mut file = File::create("dist/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

pub fn home_page() -> String {
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
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Home,

            div {
                class: "p-5 mt-24 flex flex-col items-center",
                VideoHero {
                    video: "https://www.youtube.com/embed/slRiOOM17tM?si=yBb5noZUF44ZIo70",
                    title: "The #1 Enterprise Generative AI and Data Privacy Solution.",
                    subtitle: "We are a Chat-GPT replacement focused on data privacy and compliance.",
                    claim: "100's of installations globally.",
                    cta: "More cred",
                    cta_link: "More cred",
                }
                Customers {}

                ProblemSolution {
                    image: "/landing-page/private-deployment.svg",
                    title: "How do you get the benefits of AI and keep your data private?",
                    problem: "A Chat-GPT Replacement Without The Data Leakage",
                    solution: "A Chat-GPT Replacement Without The Data Leakage",
                }

                Benefits {
                    title: "Benefits",
                    subtitle: "More benefits",
                    benefit1: "Military grade encryption and data security best practices",
                    benefit1_desc: "A Chat-GPT Replacement Without The Data Leakage",
                    benefit2: "Military grade encryption and data security best practices",
                    benefit2_desc: "A Chat-GPT Replacement Without The Data Leakage",
                    benefit3: "Military grade encryption and data security best practices",
                    benefit3_desc: "A Chat-GPT Replacement Without The Data Leakage",
                }

                SmallImageFeature {
                    title: "Data Governance",
                    sub_title: "A Chat-GPT Replacement Without The Data Leakage",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/bionic-console.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Data Governance",
                    sub_title: "AI Assistants",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/assistants.png",
                    flip: true
                }

                SmallImageFeature {
                    title: "Data Governance",
                    sub_title: "A Chat-GPT Replacement Without The Data Leakage",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/teams.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Observability",
                    sub_title: "Powerful Observability Features",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/dashboard.png",
                    flip: true
                }

                Features {
                    title: "Bionic-GPT Features",
                    description: "A fully implemented solution for all your needs",
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

                Faq {
                    class: "m-24",
                    questions: vec![
                        FaqText {
                            question: String::from("How does Bionic-GPT ensure data privacy?"),
                            answer: String::from("Bionic-GPT runs entirely within your environment, meaning your data never leaves your control. Unlike traditional AI models, there’s no need to send information to external servers, eliminating the risk of leaks or unauthorized access."),
                        },
                        FaqText {
                            question: String::from("Is Bionic-GPT as powerful as Chat-GPT?"),
                            answer: String::from("Yes! Bionic-GPT delivers the same advanced AI capabilities as Chat-GPT, with the added advantage of running securely within your infrastructure. You get the full power of GPT without compromising privacy or control."),
                        },
                        FaqText {
                            question: String::from("Can Bionic-GPT be tailored to my specific needs?"),
                            answer: String::from("Absolutely. Bionic-GPT allows you to customize and fine-tune the AI using your own data, ensuring it provides accurate, context-aware insights and performs tasks specific to your business requirements."),
                        },
                        FaqText {
                            question: String::from("How do I monitor and manage usage?"),
                            answer: String::from("Bionic-GPT includes powerful observability and auditability features. You can track usage, monitor performance, and ensure compliance with detailed logs and insights into how the AI is being used."),
                        },
                        FaqText {
                            question: String::from("Is Bionic-GPT suitable for regulated industries?"),
                            answer: String::from("Yes. Bionic-GPT is designed with security and compliance in mind, making it ideal for industries with strict data protection requirements. It keeps sensitive information private while meeting regulatory standards."),
                        },
                    ]}

                Security {}
            }
            Footer {}
        }
    };

    crate::render(page)
}
