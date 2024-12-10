use crate::{layout::{Layout, SideBar}, render};
use db::User;
use dioxus::prelude::*;
use web_assets::files::favicon_svg;

pub fn index(users: Vec<User>) -> String {
    let page = rsx! {
        Layout {    // <-- Use our layout
            title: "Users Table",
            selected_item: SideBar::Users,
            table {
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
                                img {
                                    src: favicon_svg.name,
                                    width: "16",
                                    height: "16"
                                }
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

            // 👇 this is our new form
            form {
                action: "/new_user",
                method: "POST",
                label { r#for: "user_email", "Email:" }
                input { id: "user_email", name: "email", r#type: "email", required: "true" }
                button { "Submit" }
            }
        }
    };

    render(page)
}
