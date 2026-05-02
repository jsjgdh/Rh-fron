use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RetryErrorProps {
    pub message: String,
    pub on_retry: EventHandler<()>,
    #[props(default = "Retry".to_string())]
    pub retry_label: String,
}

#[component]
pub fn RetryError(props: RetryErrorProps) -> Element {
    rsx! {
        div { 
            class: "retry-error",
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 64px 32px; text-align: center;",
            div { style: "font-size: 3rem; margin-bottom: 16px;", "⚠️" }
            h3 { style: "font-size: 1.25rem; font-weight: 700; color: var(--text-primary); margin: 0 0 8px 0;", "Something went wrong" }
            p { style: "color: var(--text-faint); font-size: 14px; margin: 0 0 24px 0; max-width: 400px;", "{props.message}" }
            button {
                class: "btn btn-primary",
                onclick: move |_| props.on_retry.call(()),
                "{props.retry_label}"
            }
        }
    }
}