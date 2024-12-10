use crate::{layout::{Layout, SideBar}, render};
use daisy_rsx::*;
use db::User;
use dioxus::prelude::*;
use web_assets::files::favicon_svg;

pub fn index(users: Vec<User>) -> String {
    let page = rsx! {
        Layout {    // <-- Use our layout
            title: "Users Table",
            selected_item: SideBar::Users,
            BlankSlate {
                heading: "Looks like you don't have any API keys",
                visual: favicon_svg.name,
                description: "API Keys allow you to access our programming interface",
            }
            Card {
                class: "mt-12 has-data-table",
                CardHeader {
                    title: "Users"
                }
                CardBody {
                    table {
                        class: "table table-sm",
                        thead {
                            tr {
                                th { "ID" }
                                th { "Email" }
                            }
                        }
                        tbody {
                            for user in users {
                                tr {
                                    td {
                                        strong {
                                            "{user.id}"
                                        }
                                    }
                                    td {
                                        "{user.email}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Card {
                class: "mt-12",
                CardHeader {
                    title: "Add User"
                }
                CardBody {
                    form {
                        action: "/new_user",
                        method: "POST",
                        label { r#for: "user_email", "Email:" }
                        input { id: "user_email", name: "email", r#type: "email", required: "true" }
                        button { "Submit" }
                    }
                }
            }
        }
    };

    render(page)
}
