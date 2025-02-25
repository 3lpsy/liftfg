#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn HomeIcon() -> Element {
    rsx! {
        svg {
            class: "size-[1.2em]",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            g {
                fill: "currentColor",
                stroke_linejoin: "miter",
                stroke_linecap: "butt",
                polyline {
                    points: "1 11 12 2 23 11",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_miterlimit: "10",
                    stroke_width: "2"
                }
                path {
                    d: "m5,13v7c0,1.105.895,2,2,2h10c1.105,0,2-.895,2-2v-7",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2"
                }
                line {
                    x1: "12",
                    y1: "22",
                    x2: "12",
                    y2: "18",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2"
                }
            }
        }
    }
}

#[component]
pub fn ProfileIcon() -> Element {
    rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-5 w-5",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
                }
            }

    }
}

#[component]
pub fn ArrowRight() -> Element {
    rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-6",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                    d: "M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3"
                }
            }

    }
}

#[component]
pub fn ArrowLeft() -> Element {
    rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-6",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                    d: "M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18"
                }
            }

    }
}

#[component]
pub fn DropDownIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-5 w-5",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M4 6h16M4 12h8m-8 6h16"
            }
        }
    }
}

#[component]
pub fn InboxIcon() -> Element {
    rsx! {
        svg {
            class: "size-[1.2em]",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            g {
                fill: "currentColor",
                stroke_linejoin: "miter",
                stroke_linecap: "butt",
                polyline {
                    points: "3 14 9 14 9 17 15 17 15 14 21 14",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_miterlimit: "10",
                    stroke_width: "2"
                }
                rect {
                    x: "3",
                    y: "3",
                    width: "18",
                    height: "18",
                    rx: "2",
                    ry: "2",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2"
                }
            }
        }
    }
}

pub fn GearIcon() -> Element {
    rsx! {
        svg {
            class: "size-[1.2em]",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            g {
                fill: "currentColor",
                stroke_linejoin: "miter",
                stroke_linecap: "butt",
                circle {
                    cx: "12",
                    cy: "12",
                    r: "3",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2"
                }
                path {
                    d: "m22,13.25v-2.5l-2.318-.966c-.167-.581-.395-1.135-.682-1.654l.954-2.318-1.768-1.768-2.318.954c-.518-.287-1.073-.515-1.654-.682l-.966-2.318h-2.5l-.966,2.318c-.581.167-1.135.395-1.654.682l-2.318-.954-1.768,1.768.954,2.318c-.287.518-.515,1.073-.682,1.654l-2.318.966v2.5l2.318.966c.167.581.395,1.135.682,1.654l-.954,2.318,1.768,1.768,2.318-.954c.518.287,1.073.515,1.654.682l.966,2.318h2.5l.966-2.318c.581-.167,1.135-.395,1.654-.682l2.318.954,1.768-1.768-.954-2.318c.287-.518.515-1.073.682-1.654l2.318-.966Z",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2"
                }
            }
        }
    }
}
