use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    id: String,
    title: Option<String>,
    description: Option<String>,
    modal_ref: Signal<Option<HtmlDialogElement>>,
    on_close: Option<EventHandler<()>>,
    children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let id = props.id;
    let title_ele = match &props.title {
        Some(title) => rsx! { h3 { "{title}" } },
        None => rsx! {},
    };
    let description_ele = match &props.description {
        Some(description) => rsx! { p { "{description}" } },
        None => rsx! {},
    };
    rsx! {
        dialog {
            onmounted: move |event| {
                let mut modal_ref = props.modal_ref;
                modal_ref.set(Some(event.as_web_event().dyn_into::<HtmlDialogElement>().unwrap()));
            },
            id: "{id}",
            class: "modal modal-bottom sm:modal-middle",
            div {
                class: "modal-box",
                {title_ele},
                {description_ele}
                form {
                    method:"dialog",
                    button {
                        class:"btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                        onclick: move |_| {
                            if let Some(on_close) = props.on_close {
                                on_close(())
                            }
                            let modal_ref = props.modal_ref;
                            if let Some(r) = modal_ref() {
                                r.close();
                            }
                        },
                        "x"
                    }
                }
                {props.children}
            }
            form {
                method:"dialog",
                class: "modal-backdrop",
                onclick: move |_| {
                    if let Some(on_close) = props.on_close {
                        on_close(())
                    }
                    let modal_ref = props.modal_ref;
                    if let Some(r) = modal_ref() {
                        r.close();
                    }
                }
            }
        }
    }
}
