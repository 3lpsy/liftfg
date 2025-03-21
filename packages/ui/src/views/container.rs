#![allow(non_snake_case)]
use crate::components::dock::Dock;
use crate::components::navbar::NavBar;
use crate::router;
use crate::services::get;
use crate::{router::Route, views::Loading};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileData, ProfileShowParams};
use validator::ValidationErrors;

#[component]
pub fn Container() -> Element {
    let app_errors: Signal<ValidationErrors> = use_signal(|| ValidationErrors::new());
    use_context_provider(|| app_errors.clone());

    let profile_res = use_resource(move || async move {
        get::<ProfileShowParams, ProfileData>("profile_show", None).await
    })
    .suspend()?;

    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();

    let nav = navigator();
    let router = router();
    let is_onboard = use_memo(move || {
        router
            .current::<router::Route>()
            .to_string()
            .starts_with("/onboard")
    });

    use_hook(move || match profile_res() {
        Ok(profile) => {
            *current_profile_ctx.write() = Some(profile.clone());
            if is_onboard() {
                nav.push(router::Route::Home {});
            }
        }
        Err(e) => {
            let should_create_profile = e.field_errors().iter().any(|(field, errors)| {
                field == "is_default" && errors.iter().any(|err| err.code == "exists")
            });
            if should_create_profile {
                // no default profile exists, we're onboarding
                nav.replace(router::Route::OnboardIndexView {});
            } else {
                let mut app_errors = use_context::<Signal<ValidationErrors>>();
                app_errors.set(e.clone());
            }
        }
    });

    rsx! {
        NavBar {},
        div {
            // bootleg container with padding
            // don't want to use container as want scrollbar on edge
            class: "page py-2 px-2 sm:px-8 md:px-12 lg:px-16 mx-auto flex flex-col",
            SuspenseBoundary {
                fallback: |_| rsx!{
                    div {
                        class: "flex items-center justify-center flex-1",
                        Loading {  }
                    }
                },
                ErrorBoundary {
                    handle_error: |err| {
                        rsx! {
                            "An unhandled error has occured: {err:?}"
                        }
                    },

                    div {
                        class: "flex-auto mx-4 my-2 min-h-full",
                        if ! app_errors().is_empty() {
                            code { "{app_errors():?}"}
                        }
                        Outlet::<Route> {},
                    }
                }
            }

        }
        if !is_onboard() {
            Dock {}
        }
    }
}
