use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    pub title: String,
    pub open: bool,
    pub on_close: EventHandler<()>,
    pub children: Element,
    #[props(default = false)]
    alert: bool,
    #[props(default = false)]
    dangerous: bool,
    #[props(default = "500px".to_string())]
    width: String,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !props.open {
        return rsx! {};
    }

    let role = if props.alert { "alertdialog" } else { "dialog" };
    let aria_modal = "true";
    let described_by = format!("modal-desc-{}", props.title.replace(" ", "-").to_lowercase());
    let border_style = if props.dangerous {
        "border-color: var(--status-error);"
    } else {
        ""
    };
    let full_style = format!("position: fixed; top: 50%; left: 50%; transform: translate(-50%,-50%); z-index: 1001; background: var(--surface); border: 1px solid var(--border); border-radius: 16px; padding: 32px; width: {}; max-width: 90vw; box-shadow: var(--shadow-lg); {}", props.width, border_style);

    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| props.on_close.call(()),
            div {
                class: "modal-content fade-in",
                role: "{role}",
                aria_modal: "{aria_modal}",
                aria_labelledby: "modal-title",
                aria_describedby: "{described_by}",
                style: "{full_style}",
                div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 16px;",
                    h2 { 
                        id: "modal-title",
                        style: "font-size: 1.5rem; margin: 0;", 
                        "{props.title}" 
                    }
                    button {
                        class: "btn btn-ghost",
                        style: "padding: 8px; margin: -8px -8px 0 0;",
                        aria_label: "Close dialog",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }
                div { 
                    id: "{described_by}",
                    style: "color: var(--text-secondary); font-size: 14px; margin-bottom: 24px;",
                    {props.children}
                }
            }
        }
    }
}

#[component]
pub fn ModalFooter(props: ModalFooterProps) -> Element {
    rsx! {
        div { class: "modal-footer", style: "display: flex; gap: 12px; justify-content: flex-end; margin-top: 24px;",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ModalFooterProps {
    pub children: Element,
}