#![allow(non_snake_case)]
use crate::components::dock::Dock;
use crate::components::navbar::NavBar;
use crate::logging::{self, info};
use crate::router;
use crate::services::profile::get_profile;
use crate::{router::Route, views::Loading};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileData, ProfileShowParams};
use validator::ValidationErrors;

#[component]
pub fn Container() -> Element {
    logging::info("Rendering Container");

    let profile_res = use_resource(move || async move {
        logging::info("Loading profile resource callback");
        // implicitly use is_default if current is None, in that case, check the error
        get_profile(Some(ProfileShowParams {
            id: None,
            name: None,
        }))
        .await
    })
    .suspend()?;

    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();

    let nav = use_navigator();
    use_hook(move || match profile_res() {
        Ok(profile) => {
            info("Updating current profile in container");
            *current_profile_ctx.write() = Some(profile.clone());
        }
        Err(e) => {
            let should_create_profile = e.field_errors().iter().any(|(field, errors)| {
                field == "is_default" && errors.iter().any(|err| err.code == "exists")
            });
            if should_create_profile {
                // no default profile exists, we're onboarding
                info("Naving to onboartdindexview");
                nav.replace(router::Route::OnboardIndexView {});
            } else {
                let mut app_errors = use_context::<Signal<ValidationErrors>>();
                app_errors.set(e.clone());
                nav.replace(router::Route::Errors {});
            }
        }
    });
    // TODO: fix
    let router = router();
    let route = use_memo(move || router.current::<Route>());
    let show_dock = use_memo(move || !route.to_string().starts_with("/onboard"));
    rsx! {
        NavBar {},
        div {
            class: "page container mx-auto flex flex-col",
            SuspenseBoundary {
                fallback: |_| rsx!{
                    div {
                        class: "flex items-center justify-center flex-1",
                        Loading {  }
                    }
                },
                div {
                    class: "mx-4 my-2 h-full",
                    Outlet::<Route> {},
                    div { class: "my-2", p {"Route: {route()}"} }
                }
            }

        }
        if show_dock() {
            Dock {}
        }
    }
}
