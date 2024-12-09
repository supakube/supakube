#![allow(non_snake_case)]
use assets::files::*;
use daisy_rsx::{NavGroup, NavItem};
use dioxus::prelude::*;

pub fn render_page(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SideBar {
    Dashboard,
}

impl std::fmt::Display for SideBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    selected_item: SideBar,
    title: String,
    header: Element,
    children: Element,
}

pub fn Layout(props: LayoutProps) -> Element {
    let stylesheets = vec![output_css.name.to_string()];

    rsx! {
        super::base_layout::BaseLayout {
            title: props.title,
            stylesheets: stylesheets,
            fav_icon_src: favicon_svg.name,
            header: rsx!(
                {props.header}
            ),
            sidebar: rsx!(
                NavGroup {
                    heading: "Generative AI",
                    content:  rsx!(
                        NavItem {
                            id: SideBar::Dashboard.to_string(),
                            selected_item_id: props.selected_item.to_string(),
                            href: super::root::Index { },
                            icon: favicon_svg.name,
                            title: "Dashboard"
                        }
                    )
                }
            ),
            sidebar_header: rsx!(
                h1 {
                    "Header"
                }  
            ),
            sidebar_footer: rsx!(
                h1 {
                    "Footer"
                }  
            ),
            {props.children}
        }
    }
}