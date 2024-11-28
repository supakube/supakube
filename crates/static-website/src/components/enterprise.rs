use std::fs::{self, File};
use std::io::Write;

use super::image_hero::ImageHero;
use crate::components::benefits::Benefits;
use crate::components::customer_logos::Customers;
use crate::components::footer::Footer;
use crate::components::image_feature::ImageFeature;
use crate::components::problem_solution::ProblemSolution;
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
                class: "mt-24 flex flex-col items-center",
                ImageHero {
                    title: "On Premise or Private Cloud. Enterprise Generative AI.",
                    subtitle: "Collaborate with the highest levels of security and privacy across all AI models.
    Use any model on your own server or cloud."
                }
                Customers {}

                ProblemSolution {
                    video: "https://www.youtube.com/embed/mNFd0Bur238?si=69vNCg09KvoCKzW3",
                    title: "How can your enterprise unlock 100's of Gen AI use cases?",
                    subtitle: "A Chat-GPT Replacement Without The Data Leakage",
                }


                Benefits {
                    title: "Services",
                    subtitle: "Services",
                    benefit1: "AI Training",
                    benefit1_desc: "Empower your team with the skills to harness generative AI confidently and effectively. Our training covers both general AI knowledge and specific product training on bionicGPT, ensuring that your team can use the platform to its fullest potential. Whether your team is new to AI or looking to advance their expertise, we provide insights into AI-driven data workflows, and secure deployment. Our hands-on sessions transform AI concepts into practical applications, giving your team the tools to integrate generative AI and ensure data security and compliance.",
                    benefit2: "AI Consulting",
                    benefit2_desc: "Our AI Consulting services focus on transforming your specific business needs into effective AI solutions. We work closely with your team to identify opportunities, design strategies, and integrate AI in ways that drive real value. From feasibility studies to model selection and data management, we’re here to guide you through each step, ensuring your AI initiatives align with industry best practices and regulatory standards.",
                    benefit3: "AI Development",
                    benefit3_desc: "Our team specialises in custom AI development, tailored extensions to bionicGPT, and powerful AI agents. From enhancing existing functionalities to creating bespoke AI workflows, we help you deploy highly effective solutions that fit seamlessly into your tech stack. We also provide support for integrating AI agents that automate tasks, streamline data handling, and optimize operations. With bionicGPT as your foundation, you can scale up securely and efficiently with innovations that align perfectly with your goals.",
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
            }
            Footer {}
        }
    }
}
