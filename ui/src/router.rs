#![allow(non_snake_case)]
use crate::views::{Container, Errors, Home, NotFound, ProfileCreate};
use dioxus::prelude::*;
// use dioxus_router::prelude::*;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    // The home page is at the / route
    #[layout(Container)]
        #[route("/")]
        Home {},
        #[route("/profile/create")]
        ProfileCreate {},
        #[route("/error")]
        Errors { },
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
