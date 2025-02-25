#![allow(non_snake_case)]
use crate::views::{
    Container, Errors, Empty, Home, ResourceNotFound, NotFoundFallback, Loading, ProfileCreate, ProfileIndex, ProfileShow,
    ProgramCreate,
};
use dioxus::prelude::*;
use validator::ValidationErrors;
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
        #[route("/profile/:profile_id")]
        ProfileShow {profile_id: usize},
        #[route("/profile/:profile_id/program/create")]
        ProgramCreate {
            profile_id: usize
        },
        #[route("/error")]
        Errors { errors: ValidationErrors },
        #[route("/not-found")]
        ResourceNotFound { errors: Option<ValidationErrors> },

        #[route("/:..route")]
        NotFoundFallback { route: Vec<String> },
    #[end_layout]
    // don't use this
    #[route("/_empty")]
    Empty {  },
}
