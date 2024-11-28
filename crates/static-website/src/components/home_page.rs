use super::image_hero::ImageHero;
use crate::components::benefits::Benefits;
use crate::components::customer_logos::Customers;
use crate::components::footer::Footer;
use crate::components::image_feature::ImageFeature;
use crate::components::problem_solution::ProblemSolution;
use crate::components::testamonials::Testamonials;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;
use std::fs::File;
use std::io::Write;

pub async fn generate() {
    let html = crate::render(HomePage).await;

    let mut file = File::create("dist/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Layout {
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,

            div {
                class: "mt-24 flex flex-col items-center",
                ImageHero {
                    title: "Increase company productivity with Private Generative AI",
                    subtitle: "We use hardware based confidential computing to
                        run AI in a highly secure enclave for maximum 
                        protection of your data in the cloud or on premise"
                }
                Customers {}

                ProblemSolution {
                    video: "https://www.youtube.com/embed/slRiOOM17tM?si=yBb5noZUF44ZIo70",
                    title: "How do you get the benefits of AI and keep your data private?",
                    subtitle: "A Chat-GPT Replacement Without The Data Leakage",
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

                Testamonials {}
            }
            Footer {}
        }
    }
}