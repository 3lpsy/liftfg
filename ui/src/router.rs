#![allow(non_snake_case)]
use crate::views::{Home, Layout, NotFound};
use dioxus::prelude::*;
// use dioxus_router::prelude::*;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    // The home page is at the / route
    #[layout(Layout)]
        #[route("/")]
        Home {},
    #[end_layout]
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
