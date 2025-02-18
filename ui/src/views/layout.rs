#![allow(non_snake_case)]
use crate::{
    components::NavBar,
    router::Route,
    state::{State, APP_ERRORS, APP_STATE},
};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    let app_state = APP_STATE.read();
    let app_errors = APP_ERRORS.read();

    rsx! {
        if *app_state.state.read() == State::Ready {
            NavBar {},
            Outlet::<Route> {},
        } else if *app_state.state.read() == State::Loading {
            "Loading"
        } else {
            code { "{app_state:?}"},
            code { "{app_errors:?}"}
        }

    }
}
