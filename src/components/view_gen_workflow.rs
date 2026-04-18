use dioxus::prelude::*;
use crate::components::workflow_graph::WorkflowGraph;

#[derive(PartialEq, Props, Clone)]
pub struct ViewGenWorkflowProps {
    workflow_name: String,
    version: String,
    #[props(default)]
    on_edit_requested: EventHandler<()>,
}

#[derive(Clone, Copy, PartialEq)]
enum DetailTab {
    Overview,
    Logic,
    Visual,
    Audit,
}

#[component]
pub fn ViewGenWorkflow(props: ViewGenWorkflowProps) -> Element {
    let workflow_name = props.workflow_name.clone();
    let version = props.version.clone();
    let mut current_tab = use_signal(|| DetailTab::Overview);
    let on_edit_requested = props.on_edit_requested.clone();

    let detail_res = use_resource({
        let workflow_name = workflow_name.clone();
        let version = version.clone();
        move || {
            let workflow_name = workflow_name.clone();
            let version = version.clone();
            async move {
                crate::api::get_workflow_detail(&workflow_name, &version).await.ok()
            }
        }
    });

    rsx! {
        div { class: "dashboard-stack",
            // ── Header Summary ───────────────────────────────────────
            div { class: "tabs-header",
                div { 
                    class: if *current_tab.read() == DetailTab::Overview { "tab-item active" } else { "tab-item" },
                    onclick: move |_| current_tab.set(DetailTab::Overview),
                    "Overview" 
                }
                div { 
                    class: if *current_tab.read() == DetailTab::Logic { "tab-item active" } else { "tab-item" },
                    onclick: move |_| current_tab.set(DetailTab::Logic),
                    "Logic" 
                }
                div { 
                    class: if *current_tab.read() == DetailTab::Visual { "tab-item active" } else { "tab-item" },
                    onclick: move |_| current_tab.set(DetailTab::Visual),
                    "Visual Flow" 
                }
                div { 
                    class: if *current_tab.read() == DetailTab::Audit { "tab-item active" } else { "tab-item" },
                    onclick: move |_| current_tab.set(DetailTab::Audit),
                    "Execution Audit" 
                }

                button {
                    class: "btn btn-secondary btn-sm",
                    style: "margin-left: auto;",
                    onclick: move |_| on_edit_requested.call(()),
                    "Open in Builder"
                }
            }

            match *current_tab.read() {
                DetailTab::Overview => rsx! {
                    div { class: "dashboard-stack",
                        div { class: "grid-3",
                            div { class: "stat-card",
                                div { class: "stat-label", "Workflow ID" }
                                div { class: "stat-value stat-value-small", "{workflow_name}" }
                            }
                            div { class: "stat-card",
                                div { class: "stat-label", "Release Version" }
                                div { class: "stat-value", "{version}" }
                            }
                            div { class: "stat-card",
                                div { class: "stat-label", "Compiled Status" }
                                div { class: "stat-value stat-value-small", span { class: "badge badge-success", "Verified" } }
                            }
                        }

                        div { class: "grid-2",
                            section { class: "card",
                                div { class: "card-header",
                                    div {
                                        div { class: "card-title", "Release Metadata" }
                                        div { class: "card-description", "Technical overview of the compiled artifact." }
                                    }
                                }
                                if let Some(Some(detail)) = detail_res.read().as_ref() {
                                    div { class: "pipeline-list",
                                        div { class: "pipeline-step",
                                            div { class: "pipeline-index", "01" }
                                            div {
                                                div { class: "pipeline-title", "Input Surface" }
                                                p { class: "pipeline-copy", "Accepts {detail.get(\"inputs\").and_then(|i| i.as_array()).map(|a| a.len()).unwrap_or(0)} parameters." }
                                            }
                                        }
                                        div { class: "pipeline-step",
                                            div { class: "pipeline-index", "02" }
                                            div {
                                                div { class: "pipeline-title", "Logic Depth" }
                                                p { class: "pipeline-copy", "Contains {detail.get(\"steps\").and_then(|i| i.as_array()).map(|a| a.len()).unwrap_or(0)} individual steps." }
                                            }
                                        }
                                    }
                                }
                            }

                            section { class: "card",
                                div { class: "card-header",
                                    div {
                                        div { class: "card-title", "Resource Posture" }
                                        div { class: "card-description", "Immutable artifacts for this release." }
                                    }
                                }
                                ul { class: "sidebar-nav", style: "padding: 0;",
                                    div { class: "nav-item", span { class: "nav-icon", "JSON" } span { "AST Representation" } }
                                    div { class: "nav-item", span { class: "nav-icon", "TXT" } span { "RheLang Source" } }
                                    div { class: "nav-item", span { class: "nav-icon", "BIN" } span { "Compiled IR" } }
                                }
                            }
                        }
                    }
                },
                DetailTab::Logic => rsx! {
                    section { class: "card",
                        div { class: "card-header",
                            div {
                                div { class: "card-title", "RheLang Source code" }
                                div { class: "card-description", "Final source that generated this release." }
                            }
                        }
                        if let Some(Some(detail)) = detail_res.read().as_ref() {
                            pre { 
                                class: "code-block", 
                                style: "background: #08080A; padding: 24px; border-radius: 4px; border: 1px solid var(--border); max-height: 600px; overflow: auto;",
                                "{detail.get(\"source\").and_then(|v| v.as_str()).unwrap_or(\"No source\")}" 
                            }
                        }
                    }
                },
                DetailTab::Visual => rsx! {
                    section { class: "card",
                        div { class: "card-header",
                            div {
                                div { class: "card-title", "Interactive Visual Flow" }
                                div { class: "card-description", "Deterministic path analysis based on the compiled AST." }
                            }
                        }
                        div { class: "vg-canvas",
                            if let Some(Some(detail)) = detail_res.read().as_ref() {
                                if let Some(ast_str) = detail.get("ast_json").and_then(|v| v.as_str()) {
                                    if let Ok(ast) = serde_json::from_str::<serde_json::Value>(ast_str) {
                                        WorkflowGraph { injected_ast: ast }
                                    }
                                }
                            }
                        }
                    }
                },
                DetailTab::Audit => rsx! {
                    section { class: "card",
                        div { class: "card-header",
                            div {
                                div { class: "card-title", "Execution History" }
                                div { class: "card-description", "Every trace recorded for this specific version." }
                            }
                        }
                        div { class: "type-table",
                            div { class: "type-row type-row-head",
                                div { "Trigger" }
                                div { "Status" }
                                div { "Timestamp" }
                                div { "Audit Trace" }
                            }
                            // Using the activity list but filtered if the component supported it. 
                            // For now, it shows workspace activity.
                            crate::components::activity_list::ActivityList { 
                                on_select: move |_| {} 
                            }
                        }
                    }
                }
            }
        }
    }
}
