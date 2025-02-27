#![allow(non_snake_case)]

use crate::views::{
    Container, Empty, Errors, Home, NotFoundFallback, ProfileCreateOnboardView, ProfileCreateView,
    ProfileEditView, ProfileIndexView, ProfileShowView, ProgramCreate,
};
use dioxus::prelude::*;

// use dioxus_router::prelude::*;
//

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    // The home page is at the / route
    #[layout(Container)]
        #[route("/")]
        Home {},
        #[route("/profile/create/onboard")]
        ProfileCreateOnboardView {},
        #[route("/profile")]
        ProfileIndexView {},
        #[route("/profile/create")]
        ProfileCreateView {},
        #[route("/profile/:profile_id")]
        ProfileShowView {profile_id: usize},
        #[route("/profile/:profile_id/edit")]
        ProfileEditView {profile_id: usize},
        #[route("/profile/:profile_id/program/create")]
        ProgramCreate {
            profile_id: usize
        },
        #[route("/errors")]
        Errors { },
        #[route("/:..route")]
        NotFoundFallback { route: Vec<String> },
    #[end_layout]
    // don't use this
    #[route("/_empty")]
    Empty {  },
}
