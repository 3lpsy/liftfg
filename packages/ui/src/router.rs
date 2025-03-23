#![allow(non_snake_case)]

use crate::views::{
    Container, Empty, Home, NotFoundFallback, OnboardIndexView, OnboardProfileCreateView,
    OnboardTermsIndexView, ProfileCreateView, ProfileEditView, ProfileIndexView, ProfileShowView,
    ProfileWorkoutCreateView, ProfileWorkoutIndexView, WorkoutCreateView, WorkoutEditView,
};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    rsx! { h1 { "Welcome to the Dioxus Blog!" } }
}

/// An enum of all of the possible routes in the app.
#[derive(Routable, Debug, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The home page is at the / route
    #[layout(Container)]
        #[route("/")]
        Index {},
        #[route("/home")]
        Home {},
        #[route("/onboard")]
        OnboardIndexView {},
        #[route("/onboard/profile/create")]
        OnboardProfileCreateView {},
        #[route("/onboard/terminology")]
        OnboardTermsIndexView {},
        #[route("/current/profile-workout")]
        ProfileWorkoutIndexView {},
        #[route("/profile")]
        ProfileIndexView {},
        #[route("/profile/create")]
        ProfileCreateView {},
        #[route("/profile/:profile_id")]
        ProfileShowView {profile_id: usize},
        #[route("/profile/:profile_id/edit")]
        ProfileEditView {profile_id: usize},
        // part of create workout flow
        // should be route param because it may not be default
        #[route("/profile/:profile_id/profile-workout/create")]
        ProfileWorkoutCreateView {
            profile_id: usize
        },
        #[route("/workout/create")]
        WorkoutCreateView {},
        #[route("/workout/:workout_id")]
        WorkoutEditView {workout_id: usize},
        #[route("/:..route")]
        NotFoundFallback { route: Vec<String> },
    #[end_layout]
    // don't use this
    #[route("/_empty")]
    Empty {  },
}
