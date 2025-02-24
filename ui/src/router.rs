#![allow(non_snake_case)]
use crate::views::{
    Container, Errors, Home, NotFound, NotFoundRoot, ProfileCreate, ProfileIndex, ProgramCreate,
};
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
        #[route("/onboard/profile/create")]
        ProfileCreate {},
        #[route("/profile")]
        ProfileIndex {},
        #[route("/profile/:profile_id/program/create")]
        ProgramCreate {
            profile_id: usize
        },
        #[route("/error")]
        Errors { },
        #[route("/:..route")]
        NotFound { route: Vec<String> },
    #[end_layout]
    #[route("/_x")]
    NotFoundRoot { route: Vec<String> },
}
