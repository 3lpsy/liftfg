#![allow(non_snake_case)]
use crate::icons;
use crate::router;
use crate::services::profile::get_profile;
use dioxus::prelude::*;
use fgdb::data::profile::ProfileData;
use fgdb::data::profile::ProfileShowParams;
use validator::ValidationErrors;

#[component]
pub fn ProgramCreateView(profile_id: usize) -> Element {
    let mut profile_sig: Signal<Option<ProfileData>> = use_signal(|| None);
    let profile_res = use_resource(move || async move {
        get_profile(Some(ProfileShowParams {
            id: Some(profile_id as i32),
            name: None,
        }))
        .await
    })
    .suspend()?;
    // first we write get_programs
    // second, add a includes setup for profile to include programs on response

    let nav = use_navigator();
    use_effect(move || match profile_res() {
        Ok(profile) => {
            profile_sig.set(Some(profile.clone()));
        }
        Err(e) => {
            let mut app_errors = use_context::<Signal<ValidationErrors>>();
            app_errors.set(e.clone());
            nav.replace(router::Route::Errors {});
        }
    });
    rsx! {
        div {
            class: "flex justify-between items-center",
            h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Add Program" },
            Link {
                class: "btn btn-outline",
                to: router::Route::Home  {},
                "Create Program"
            }

        }
        div {
            class: "justify-center",
            p {
                "A program guides your workout and will determine what exercises you will be prompted for based on desired muscle groups and volume."
            }
        }

        div {
            class: "divider"
        }

        h2 { class: "text-xl sm:text-2xl font-bold text-base-content", "Selected Programs" }
        p { "None" }
        div  {
            class: "mt-2",
            label {
                class: "input w-full",
                icons::SearchIcon{},
                input {
                    r#type:"search",
                    required: true,
                    placeholder: "Filter"
                }
            }
        }

    }
}
