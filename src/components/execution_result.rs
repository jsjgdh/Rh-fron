//! Execution result display component.
//!
//! Shows the outcome of a workflow execution including the result,
//! actions taken, and the full execution trace.

use dioxus::prelude::*;

/// Props for the execution result component.
#[derive(Props, Clone, PartialEq)]
pub struct ExecutionResultProps {
    pub success: bool,
    pub final_step: String,
    pub actions: Vec<String>,
}

/// Displays the result of a workflow execution.
#[component]
pub fn ExecutionResult(props: ExecutionResultProps) -> Element {
    rsx! {
        div { class: "card",
            div { class: "card-header",
                div { class: "card-title", "Result" }
                if props.success {
                    span { class: "badge badge-success", "SUCCESS" }
                } else {
                    span { class: "badge badge-danger", "FAILED" }
                }
            }

            p { style: "color: var(--text-secondary);",
                "Final step: "
                strong { "{props.final_step}" }
            }

            if !props.actions.is_empty() {
                div { style: "margin-top: 16px;",
                    div { class: "form-label", "Actions Triggered" }
                    for action in props.actions.iter() {
                        div { class: "trace-step",
                            div { class: "trace-step-action", "{action}" }
                        }
                    }
                }
            }
        }
    }
}
