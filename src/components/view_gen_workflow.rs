use dioxus::prelude::*;
use crate::components::workflow_graph::WorkflowGraph;
use crate::app::Route;

#[derive(Clone, Copy, PartialEq)]
enum DetailTab {
    Overview,
    Logic,
    Visual,
    Audit,
}

#[component]
pub fn ViewGenWorkflow(name: String, version: String) -> Element {
    let mut current_tab = use_signal(|| DetailTab::Overview);
    let nav = use_navigator();

    let detail_res = use_resource({
        let name = name.clone();
        let version = version.clone();
        move || {
            let name = name.clone();
            let version = version.clone();
            async move {
                crate::api::get_workflow_detail(&name, &version).await.ok()
            }
        }
    });

    rsx! {
        div { class: "dashboard-studio",
            
            // ── Studio Navigation & Actions ──────────────────────────
            div { class: "tabs-header",
                style: "margin-bottom: 32px; border-bottom: 1px solid var(--border-subtle);",
                
                div { class: "label-caps", style: "margin-right: 24px; color: var(--brand-emerald);", "STUDIO V2" }
                
                for (tab, label) in [(DetailTab::Overview, "Overview"), (DetailTab::Logic, "Logic"), (DetailTab::Visual, "Visual Flow"), (DetailTab::Audit, "Execution Audit")] {
                    div { 
                        class: if *current_tab.read() == tab { "tab-item active" } else { "tab-item" },
                        onclick: move |_| current_tab.set(tab),
                        "{label}" 
                    }
                }

                button {
                    class: "btn btn-primary btn-sm",
                    style: "margin-left: auto; height: 32px;",
                    onclick: move |_| { nav.push(Route::ViewEdit { name: name.clone(), version: version.clone() }); },
                    "Open in Builder"
                }
            }

            match *current_tab.read() {
                DetailTab::Overview => rsx! {
                    div { class: "dashboard-studio",
                        style: "padding: 0;",
                        
                        // 1. High-Level Telemetry
                        div { class: "studio-card-row",
                            div { class: "studio-glass-card",
                                div { class: "studio-label", "Policy Identifier" }
                                div { class: "studio-value", style: "font-family: var(--font-mono); font-size: 18px;", "{name}" }
                            }
                            div { class: "studio-glass-card",
                                div { class: "studio-label", "Release Target" }
                                div { class: "studio-value", "v{version}" }
                            }
                            div { class: "studio-glass-card",
                                div { class: "studio-label", "Integrity Status" }
                                div { class: "studio-value", 
                                    span { class: "badge badge-success", style: "padding: 8px 16px; border-radius: 4px;", "Verified" } 
                                }
                            }
                        }

                        // 2. Technical Posture
                        div { class: "grid-2",
                            section { class: "studio-glass-card",
                                div { class: "studio-label", "Synthesis Metadata" }
                                if let Some(Some(detail)) = detail_res.read().as_ref() {
                                    div { class: "pipeline-list",
                                        style: "margin-top: 16px;",
                                        div { class: "pipeline-step",
                                            div { class: "pipeline-index", "01" }
                                            div {
                                                div { class: "pipeline-title", "Surface Parameters" }
                                                p { style: "color: var(--text-secondary); font-size: 13px;", 
                                                    "Accepts {detail.get(\"inputs\").and_then(|i| i.as_array()).map(|a| a.len()).unwrap_or(0)} unique input signals." 
                                                }
                                            }
                                        }
                                        div { class: "pipeline-step",
                                            div { class: "pipeline-index", "02" }
                                            div {
                                                div { class: "pipeline-title", "Logic Density" }
                                                p { style: "color: var(--text-secondary); font-size: 13px;", 
                                                    "Orchestrated across {detail.get(\"steps\").and_then(|i| i.as_array()).map(|a| a.len()).unwrap_or(0)} discrete states." 
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            section { class: "studio-glass-card",
                                div { class: "studio-label", "Asset Inventory" }
                                div { class: "sidebar-nav", style: "margin-top: 16px;",
                                    div { class: "nav-item", span { class: "nav-icon", "◇" } span { "AST Representation (JSON)" } }
                                    div { class: "nav-item", span { class: "nav-icon", "◇" } span { "RheLang Source (Raw)" } }
                                    div { class: "nav-item", span { class: "nav-icon", "◇" } span { "Compiled IR (Binary)" } }
                                }
                            }
                        }
                    }
                },
                DetailTab::Logic => rsx! {
                    section { class: "studio-glass-card",
                        div { class: "studio-label", "Compiled RheLang Source" }
                        if let Some(Some(detail)) = detail_res.read().as_ref() {
                            pre { 
                                class: "ide-panel", 
                                style: "margin-top: 16px; font-size: 13px;",
                                "{detail.get(\"source\").and_then(|v| v.as_str()).unwrap_or(\"No audit data.\")}" 
                            }
                        }
                    }
                },
                DetailTab::Visual => rsx! {
                    div { class: "studio-theme", 
                        style: "border-radius: var(--radius-lg); overflow: hidden; border: 1px solid var(--border-subtle);",
                        div { class: "builder-pane-header", "Forensic Visualizer" }
                        div { class: "vg-canvas-svg", style: "height: 600px;",
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
                    section { class: "studio-glass-card",
                        div { class: "studio-label", "Execution Forensic History" }
                        div { class: "type-table", style: "margin-top: 16px;",
                            div { class: "type-row type-row-head",
                                div { "Trigger" }
                                div { "Status" }
                                div { "Timestamp" }
                                div { "Trace ID" }
                            }
                            crate::components::activity_list::ActivityList { 
                                on_select: move |id| { nav.push(Route::ExecutionDetail { id }); } 
                            }
                        }
                    }
                }
            }
        }
    }
}
