use crate::components::modal::Modal;
use dioxus::prelude::*;
use web_sys::HtmlDialogElement;

// SelectOption type for representing each option
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

// Props for the ModalSelect component
#[derive(Props, Clone, PartialEq)]
pub struct ModalSelectProps {
    id: String,
    name: String,
    options: Vec<SelectOption>,
    initial_selection: Vec<String>,
    // synced source of truth, will reset to this on close
    synced_selection: Option<Signal<Vec<String>>>,
    placeholder: Option<String>,
    on_change: EventHandler<Vec<String>>,
}

#[component]
pub fn ModalSelect(props: ModalSelectProps) -> Element {
    let modal_ref = use_signal(|| None::<HtmlDialogElement>);
    let modal_id = props.id.clone();
    let name = props.name.clone();
    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| "Select options".to_string());

    let mut locally_selected = use_signal(move || props.initial_selection.clone());

    let open_modal = move |_| {
        if let Some(r) = modal_ref() {
            r.show_modal().expect("Could not show modal");
        }
    };

    let display_text = if locally_selected().is_empty() {
        placeholder
    } else {
        let selected_labels: Vec<String> = props
            .options
            .iter()
            .filter(|opt| locally_selected().contains(&opt.value))
            .map(|opt| opt.label.clone())
            .collect();

        if selected_labels.len() <= 2 {
            selected_labels.join(", ")
        } else {
            format!("{} items selected", selected_labels.len())
        }
    };

    let mut toggle_option = move |value: String| {
        let mut new_selection = locally_selected().clone();
        if new_selection.contains(&value) {
            new_selection.retain(|v| v != &value);
        } else {
            new_selection.push(value);
        }
        locally_selected.set(new_selection);
    };

    // make callback
    let apply_selections = move |_| {
        props.on_change.call(locally_selected());
        // don't really need to sync
        if let Some(r) = modal_ref() {
            r.close();
        }
    };

    rsx! {
        div {
            class: "relative",
            div {
                class: "select select-bordered w-full flex justify-between items-center cursor-pointer",
                onclick: open_modal,
                div {
                    class: "flex-grow truncate text-left",
                    "{display_text}"
                }
                div {
                    class: "text-gray-500",
                    svg {
                        class: "h-4 w-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        path {
                            fill_rule: "evenodd",
                            d: "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z",
                            clip_rule: "evenodd"
                        }
                    }
                }
            }
            select {
                name: "{name}",
                multiple: true,
                class: "hidden",
                for option in props.options.clone() {
                    option {
                        value: "{option.value}",
                        selected: locally_selected().contains(&option.value),
                        "{option.label}"
                    }
                }
            }

            Modal {
                id: modal_id,
                modal_ref: modal_ref,
                // Reset when closed without applying
                on_close: move |_| {
                    if let Some(currently_selected) = props.synced_selection {
                        locally_selected.set(currently_selected())
                    }
                },
                div {
                    class: "py-2",
                    // Modal title and handle bar (mobile feel)
                    div {
                        class: "mb-4 text-center relative pb-6",
                        div {
                            class: "w-12 h-1.5 bg-gray-300 rounded-full absolute -top-1 left-1/2 transform -translate-x-1/2",
                        }
                        h3 {
                            class: "text-lg font-medium mt-4",
                            "Select options"
                        }
                    }

                    // Options list
                    div {
                        class: "overflow-y-auto max-h-80",
                        ul {
                            class: "menu bg-base-200 rounded-box w-full",
                            for option in props.options {
                                li {
                                    a {
                                        class: if locally_selected().contains(&option.value.clone()) { "active" } else { "" },
                                        onclick: move |_| toggle_option(option.value.clone()),
                                        if locally_selected().contains(&option.value.clone()) {
                                            svg {
                                                class: "h-5 w-5 mr-2",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                view_box: "0 0 20 20",
                                                fill: "currentColor",
                                                path {
                                                    fill_rule: "evenodd",
                                                    d: "M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z",
                                                    clip_rule: "evenodd"
                                                }
                                            }
                                        } else {
                                            div {
                                                class: "w-5 h-5 mr-2"
                                            }
                                        }

                                        span { "{option.label}" }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "mt-6 flex justify-end gap-2",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| {
                                if let Some(currently_selected) = props.synced_selection {
                                    locally_selected.set(currently_selected())
                                }
                                if let Some(r) = modal_ref() {
                                    r.close();
                                }
                            },
                            "Cancel"
                        }

                        button {
                            class: "btn",
                            onclick: apply_selections,
                            "Done"
                        }
                    }
                }
            }
        }
    }
}
