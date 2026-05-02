use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct EmptyStateProps {
    pub icon: String,
    pub title: String,
    pub description: String,
    pub action: Option<EventHandler<()>>,
    #[props(default = "Get Started".to_string())]
    action_label: String,
}

#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    rsx! {
        div { 
            class: "empty-state",
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 64px 32px; text-align: center;",
            div { style: "font-size: 4rem; margin-bottom: 24px;", "{props.icon}" }
            h3 { style: "font-size: 1.5rem; font-weight: 700; color: var(--text-primary); margin: 0 0 12px 0;", "{props.title}" }
            p { style: "color: var(--text-faint); font-size: 14px; margin: 0 0 24px 0; max-width: 400px;", "{props.description}" }
            if let Some(on_click) = props.action {
                button {
                    class: "btn btn-primary",
                    onclick: move |_| on_click.call(()),
                    "{props.action_label}"
                }
            }
        }
    }
}