#![allow(non_snake_case)]
use crate::{components::NavBar, router::Route};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        NavBar {},
        Outlet::<Route> {}
    }
}
