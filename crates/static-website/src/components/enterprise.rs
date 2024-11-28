use std::fs::{self, File};
use std::io::Write;

use super::image_hero::ImageHero;
use crate::components::footer::Footer;
use crate::components::image_feature::ImageFeature;
use crate::components::customer_logos::Partners;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub async fn generate() {

    let html = crate::render(EnterprisePage).await;

    fs::create_dir_all("dist/enterprise").expect("Couyldn't create folder");
    let mut file = File::create("dist/enterprise/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

#[component]
pub fn EnterprisePage() -> Element {
    rsx! {
        Layout {
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,

            div {
                class: "mt-12 flex flex-col items-center",
                ImageHero {
                    title: "On Premise or Private Cloud Enterprise Generative AI",
                    subtitle: "Collaborate with the highest levels of security and privacy across all AI models.
Use any model on your own server or cloud."
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
