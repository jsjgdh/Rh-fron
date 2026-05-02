use dioxus::prelude::*;
use crate::app::show_toast;

#[derive(Props, PartialEq, Clone)]
pub struct ConfirmDialogProps {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub is_destructive: bool,
    pub on_confirm: EventHandler<()>,
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn ConfirmDialog(props: ConfirmDialogProps) -> Element {
    rsx! {
        div {
            class: "confirm-dialog-overlay",
onclick: move |evt| {
                evt.stop_propagation();
            },
            div {
                class: "confirm-dialog",
                div { class: "confirm-dialog-header",
                    h3 { "{props.title}" }
                }
                div { class: "confirm-dialog-body",
                    p { "{props.message}" }
                }
                div { class: "confirm-dialog-footer",
                    button {
                        class: "btn",
                        onclick: move |_| props.on_cancel.call(()),
                        "{props.cancel_label}"
                    }
                    button {
                        class: if props.is_destructive { "btn btn-danger" } else { "btn btn-primary" },
                        onclick: move |_| props.on_confirm.call(()),
                        "{props.confirm_label}"
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct DeleteConfirmProps {
    pub item_name: String,
    pub item_type: String,
    pub on_confirm: EventHandler<()>,
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn DeleteConfirmDialog(props: DeleteConfirmProps) -> Element {
    rsx! {
        ConfirmDialog {
            title: "Delete {props.item_type}",
            message: "Are you sure you want to delete \"{props.item_name}\"? This action cannot be undone.",
            confirm_label: "Delete",
            cancel_label: "Cancel",
            is_destructive: true,
            on_confirm: props.on_confirm,
            on_cancel: props.on_cancel,
        }
    }
}