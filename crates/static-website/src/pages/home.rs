use crate::components::benefits::Benefits;
use crate::components::customer_logos::Customers;
use crate::components::faq_accordian::Faq;
use crate::components::features::Features;
use crate::components::footer::Footer;
use crate::components::image_hero::ImageHero;
use crate::components::problem_solution::ProblemSolution;
use crate::components::small_image_feature::SmallImageFeature;
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
                    sub_title: "A Chat-GPT Replacement Without The Data Leakage",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/bionic-console.png",
                    flip: true
                }

                SmallImageFeature {
                    title: "Data Governance",
                    sub_title: "A Chat-GPT Replacement Without The Data Leakage",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/bionic-console.png",
                    flip: false
                }

                Features {}

                Testamonials {}

                Faq {}
            }
            Footer {}
        }
    }
}
