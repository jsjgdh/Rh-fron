use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct LoadingStateProps {
    #[props(default = "Loading…".to_string())]
    pub message: String,
    #[props(default = false)]
    pub inline: bool,
}

#[component]
pub fn LoadingState(props: LoadingStateProps) -> Element {
    if props.inline {
        rsx! {
            span { class: "loading-inline", style: "display: inline-flex; align-items: center; gap: 8px;",
                span { class: "loading-spinner" }
                span { "{props.message}" }
            }
        }
    } else {
        rsx! {
            div { class: "loading-state", style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 64px 32px;",
                div { class: "loading-spinner-large" }
                p { style: "color: var(--text-faint); font-size: 14px; margin-top: 16px;", "{props.message}" }
            }
        }
    }
}