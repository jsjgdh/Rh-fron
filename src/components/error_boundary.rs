use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct ErrorBoundaryProps {
    pub fallback: String,
    pub children: Element,
}

pub fn ErrorBoundary(fallback_message: String, children: Element) -> Element {
    rsx! {
        div {
            style: "display: contents;",
            {children}
        }
    }
}

#[component]
pub fn ErrorCard(message: String, on_retry: EventHandler<()>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 48px 24px; min-height: 300px;",
            div {
                style: "text-align: center; max-width: 480px;",
                div { style: "font-size: 3rem; margin-bottom: 16px;", "⚠️" }
                h2 { style: "font-size: 1.5rem; margin-bottom: 16px; color: var(--text-primary);", "Something went wrong" }
                p { style: "color: var(--text-faint); margin-bottom: 24px; font-size: 14px;", "{message}" }
                div { style: "display: flex; gap: 12px; justify-content: center;",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| on_retry.call(()),
                        "Try Again"
                    }
                    a {
                        class: "btn",
                        href: "/dashboard",
                        "Go to Dashboard"
                    }
                }
            }
        }
    }
}

#[component]
pub fn LoadingState(variant: LoadingVariant, text: Option<String>) -> Element {
    match variant {
        LoadingVariant::Spinner => rsx! {
            div {
                style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 48px; gap: 16px;",
                div { class: "spinner spinner-lg", style: "color: var(--accent-primary);" }
                if let Some(t) = text {
                    p { style: "color: var(--text-faint); font-size: 14px;", "{t}" }
                }
            }
        },
        LoadingVariant::Skeleton => rsx! {
            div {
                style: "display: flex; flex-direction: column; gap: 16px; padding: 24px;",
                div { class: "skeleton", style: "height: 24px; width: 60%;" }
                div { class: "skeleton", style: "height: 16px; width: 80%;" }
                div { class: "skeleton", style: "height: 16px; width: 40%;" }
            }
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingVariant {
    Spinner,
    Skeleton,
}