use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![
            Category {
                name: "Introducing Bionic".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Introduction",
                    description: "Introducing Bionic",
                    folder: "docs/",
                    markdown: include_str!("../content/docs/index.md"),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
            Category {
                name: "Learn Bionic".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Installation",
                    description: "Installation",
                    folder: "docs/guides/installation/",
                    markdown: include_str!("../content/docs/guides/installation/index.md"),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
        ],
    }
}
