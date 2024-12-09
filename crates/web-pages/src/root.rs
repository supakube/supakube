use crate::{layout::Layout, render};
use db::User;
use dioxus::prelude::*;

pub fn index(users: Vec<User>) -> String {
    let page = rsx! {
        Layout {    // <-- Use our layout
            title: "Users Table",
            stylesheets: vec![],
            header: rsx!(),
            sidebar: rsx!(),
            sidebar_header: rsx!(),
            sidebar_footer: rsx!(),
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
    };

    render(page)
}
