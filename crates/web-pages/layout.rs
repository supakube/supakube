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
    Customers,
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
                    heading: "Trading",
                    content:  rsx!(
                        NavItem {
                            id: SideBar::Dashboard.to_string(),
                            selected_item_id: props.selected_item.to_string(),
                            href: super::root::Index { },
                            icon: favicon_svg.name,
                            title: "Dashboard"
                        }
                        NavItem {
                            id: SideBar::Customers.to_string(),
                            selected_item_id: props.selected_item.to_string(),
                            href: super::root::Index { },
                            icon: favicon_svg.name,
                            title: "Trades"
                        }
                    )
                }
            ),
            sidebar_header: rsx!(
                div {
                    class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-neutral text-neutral-content",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        class: "lucide lucide-gallery-vertical-end size-4",
                        path {
                            d: "M7 2h10",
                        }
                        path {
                            d: "M5 6h14",
                        }
                        rect {
                            width: "18",
                            height: "12",
                            x: "3",
                            y: "10",
                            rx: "2",
                        }
                    }
                }
                div {
                    class: "ml-3 flex flex-col gap-0.5 leading-none",
                    span {
                        class: "font-semibold uppercase",
                        "Your Application"
                    }
                    span {
                        class: "",
                        "v1.0.1"
                    }
                } 
            ),
            sidebar_footer: rsx!(
                div {
                    class: "text-center text-sm",
                    "You can place items at the bottom"
                }  
            ),
            div {
                class: "px-4",
                {props.children}
            }  
        }
    }
}