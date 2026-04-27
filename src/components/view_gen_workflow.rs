use dioxus::prelude::*;
use crate::components::workflow_graph::WorkflowGraph;
use crate::components::workflow_history::WorkflowHistory;
use crate::app::Route;

#[derive(Clone, Copy, PartialEq)]
enum DetailTab {
    Overview,
    Logic,
    Visual,
    History,
    Audit,
}

#[component]
pub fn ViewGenWorkflow(name: String, version: String) -> Element {
    let mut current_tab = use_signal(|| DetailTab::Overview);
    let mut _selected_trace_step = use_signal(|| Option::<usize>::None);
    let nav = use_navigator();

    let name_c = name.clone();
    let version_c = version.clone();
    let detail_res = use_resource(move || {
        let name = name_c.clone();
        let version = version_c.clone();
        async move {
            crate::api::get_workflow_detail(&name, &version).await.ok()
        }
    });

    let ast_payload = detail_res
        .read()
        .as_ref()
        .and_then(|detail| detail.as_ref())
        .and_then(|detail| detail.get("ast_json"))
        .and_then(|value| value.as_str())
        .and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok());

    rsx! {
        div { class: "fade-in",
            
            // ── Studio Header ────────────────────────────────────────
            div { 
                style: "display: flex; justify-content: space-between; align-items: flex-end; margin-bottom: 40px;",
                div {
                    div { class: "badge badge-success", style: "margin-bottom: 12px;", "Live v{version}" }
                    h1 { class: "page-title", style: "margin: 0; font-size: 4.5rem;", "{name}" }
                }
                {
                    let name_edit = name.clone();
                    let version_edit = version.clone();
                    rsx! {
                        div { style: "display: flex; gap: 12px;",
                            button {
                                class: "btn",
                                onclick: move |_| { nav.push(Route::ViewEdit { name: name_edit.clone(), version: version_edit.clone() }); },
                                "EDIT LOGIC"
                            }
                            button { class: "btn btn-primary", "RUN POLICY" }
                        }
                    }
                }
            }

            // ── Tabs ─────────────────────────────────────────────────
            div {
                style: "display: flex; gap: 32px; border-bottom: 1px solid var(--border); margin-bottom: 32px;",
                for (tab, label) in [
                    (DetailTab::Overview, "METRICS"),
                    (DetailTab::Logic, "SOURCE"),
                    (DetailTab::Visual, "GRAPH"),
                    (DetailTab::History, "HISTORY"),
                    (DetailTab::Audit, "FORENSICS")
                ] {
                    button { 
                        class: "btn btn-ghost",
                        style: format!("border: none; border-radius: 0; padding: 12px 4px; font-family: var(--font-display); font-size: 1.2rem; border-bottom: 2px solid {}; color: {};", 
                            if *current_tab.read() == tab { "var(--accent-primary)" } else { "transparent" },
                            if *current_tab.read() == tab { "var(--text-primary)" } else { "var(--text-faint)" }
                        ),
                        onclick: move |_| current_tab.set(tab),
                        "{label}" 
                    }
                }
            }

            match *current_tab.read() {
                DetailTab::Overview => rsx! {
                    div { class: "grid-3",
                        div { class: "card stat-card",
                            div { class: "stat-label", "Integration Density" }
                            div { class: "stat-value", "4" }
                            div { style: "font-size: 11px; color: var(--text-faint);", "Connected Service Hooks" }
                        }
                        div { class: "card stat-card",
                            div { class: "stat-label", "Cyclomatic Complexity" }
                            div { class: "stat-value", "LOW" }
                            div { style: "font-size: 11px; color: var(--text-faint);", "Deterministic Path Score" }
                        }
                        div { class: "card stat-card",
                            div { class: "stat-label", "Total Executions" }
                            div { class: "stat-value", "1.2K" }
                            div { style: "font-size: 11px; color: var(--text-faint);", "Last 30 Days" }
                        }
                    }

                    div { class: "grid-2",
                        section { class: "card",
                            div { class: "section-title", style: "margin-top: 0;", "INPUT SIGNALS" }
                            if let Some(ast) = ast_payload.as_ref() {
                                div { class: "pipeline-list",
                                    for input in ast.get("inputs").and_then(|i| i.as_array()).unwrap_or(&vec![]) {
                                        div { style: "display: flex; justify-content: space-between; padding: 12px 0; border-bottom: 1px solid var(--border-subtle);",
                                            span { class: "mono", style: "font-weight: 700;", "{input.get(\"name\").and_then(|v| v.as_str()).unwrap_or(\"?\")}" }
                                            span { class: "badge", style: "background: var(--bg);", "{input.get(\"field_type\").and_then(|v| v.as_str()).unwrap_or(\"?\")}" }
                                        }
                                    }
                                }
                            }
                        }
                        section { class: "card",
                            div { class: "section-title", style: "margin-top: 0;", "SERVICE HOOKS" }
                            div { style: "display: flex; flex-direction: column; gap: 12px;",
                                if let Some(ast) = ast_payload.as_ref() {
                                    {
                                        let hooks: Vec<(String, String)> = ast
                                            .get("steps")
                                            .and_then(|value| value.as_array())
                                            .map(|steps| {
                                                steps.iter()
                                                    .flat_map(|step| step.get("body").and_then(|value| value.as_array()).into_iter().flatten())
                                                    .filter_map(|stmt| stmt.get("Action"))
                                                    .filter_map(|action| action.get("name").and_then(|value| value.as_str()))
                                                    .filter_map(|name| name.split_once('_').map(|(provider, action)| (provider.to_uppercase(), action.to_string())))
                                                    .collect()
                                            })
                                            .unwrap_or_default();

                                        if hooks.is_empty() {
                                            rsx! { div { class: "pipeline-step", style: "margin: 0; padding: 12px;", "No external service hooks" } }
                                        } else {
                                            rsx! {
                                                for (provider, action) in hooks {
                                                    div { class: "pipeline-step", style: "margin: 0; padding: 12px;",
                                                        span { style: "font-weight: 700;", "{provider}" }
                                                        span { style: "color: var(--text-faint);", "→ {action}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                DetailTab::Logic => rsx! {
                    div { class: "card", style: "padding: 0;",
                        if let Some(Some(detail)) = detail_res.read().as_ref() {
                            pre { 
                                class: "ide-panel", 
                                style: "margin: 0; min-height: 600px;",
                                "{detail.get(\"source\").and_then(|v| v.as_str()).unwrap_or(\"Source not available.\")}" 
                            }
                        }
                    }
                },
                DetailTab::Visual => rsx! {
                    div { class: "card", style: "padding: 0; height: 700px; display: flex; flex-direction: column;",
                        div { 
                            style: "padding: 16px 32px; border-bottom: 1px solid var(--border); display: flex; justify-content: space-between; align-items: center;",
                            div { class: "section-title", style: "margin: 0; border: none;", "LOGic GRAPH" }
                            div { style: "display: flex; gap: 8px;",
                                button { class: "btn", style: "padding: 4px 12px;", "ZOOM IN" }
                                button { class: "btn", style: "padding: 4px 12px;", "ZOOM OUT" }
                            }
                        }
                        div { class: "vg-canvas-svg", style: "flex: 1;",
                            if let Some(ast) = ast_payload.clone() {
                                WorkflowGraph { injected_ast: ast }
                            }
                        }
                    }
                },
                DetailTab::History => rsx! {
                    WorkflowHistory { name: name.clone(), current_version: version.clone() }
                },
                DetailTab::Audit => rsx! {
                    div { class: "grid-2",
                        section { class: "card", style: "padding: 0;",
                            div { style: "padding: 24px; border-bottom: 1px solid var(--border);",
                                div { class: "section-title", style: "margin: 0; border: none;", "EXECUTION HISTORY" }
                            }
                            crate::components::activity_list::ActivityList { 
                                on_select: move |id| { nav.push(Route::ExecutionDetail { id }); } 
                            }
                        }
                        section { class: "card",
                            div { class: "section-title", style: "margin-top: 0;", "TRACE REPLAY" }
                            div { style: "text-align: center; padding: 40px 0;",
                                div { style: "font-size: 3rem; color: var(--border-strong); margin-bottom: 16px;", "▶" }
                                p { style: "color: var(--text-faint); font-size: 13px;", "Select a historical execution to replay the deterministic path." }
                            }
                        }
                    }
                }
            }
        }
    }
}
