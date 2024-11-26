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
                pages: vec![
                    Page {
                        date: "",
                        title: "The Console",
                        description: "The Console",
                        folder: "docs/guides/console/",
                        markdown: include_str!("../content/docs/guides/console/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Datasets",
                        description: "Datasets",
                        folder: "docs/guides/datasets/",
                        markdown: include_str!("../content/docs/guides/datasets/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "AI Assistants (RAG)",
                        description: "AI Assistants (RAG",
                        folder: "docs/guides/aiassistants/",
                        markdown: include_str!("../content/docs/guides/aiassistants/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Teams",
                        description: "Teams",
                        folder: "docs/guides/teams/",
                        markdown: include_str!("../content/docs/guides/teams/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
        ],
    }
}
