use dioxus::prelude::*;
use crate::api;

#[derive(Props, PartialEq, Clone)]
pub struct ExecutionHistoryProps {
    pub on_select: EventHandler<String>,
}

#[component]
pub fn ExecutionHistory(props: ExecutionHistoryProps) -> Element {
    let executions = use_resource(|| async move {
        api::list_recent_executions(None, None).await.unwrap_or_default()
    });

    let history = executions.read();
    
    rsx! {
        div { class: "dashboard-stack control-ledger",
            section { class: "industrial-card",
                div { style: "margin-bottom: 32px;",
                    div { class: "label-caps", "Historical Execution Ledger" }
                    p { class: "panel-copy", style: "font-size: 14px; color: var(--text-secondary);", "Review every trace recorded by the Rhexiom cluster." }
                }

                div { class: "type-table",
                    div { class: "type-row type-row-head",
                        style: "grid-template-columns: 2fr 1fr 1fr 1fr 1fr;",
                        div { "Policy Workflow" }
                        div { "Final Status" }
                        div { "Mode" }
                        div { "Terminal Step" }
                        div { "Timestamp" }
                    }

                    if let Some(h_list) = history.as_ref() {
                        if h_list.is_empty() {
                            div { style: "padding: 80px 0; text-align: center; color: var(--text-faint);",
                                div { class: "label-caps", style: "font-size: 24px; opacity: 0.1;", "EMPTY" }
                                p { style: "font-size: 14px; margin-top: 12px;", "No executions recorded in this workspace." }
                            }
                        } else {
                            for exe in h_list {
                                {
                                    let id = exe.execution_id.clone();
                                    let status_lower = exe.status.to_lowercase();
                                    let is_success = status_lower == "executed" || status_lower == "pass" || status_lower == "success";
                                    rsx! {
                                        div { 
                                            class: "type-row",
                                            style: "grid-template-columns: 2fr 1fr 1fr 1fr 1fr; cursor: pointer; transition: background 0.2s ease;",
                                            onclick: move |_| props.on_select.call(id.clone()),
                                            div { 
                                                div { style: "font-weight: 700; color: var(--text-primary);", "{exe.workflow_name}" }
                                                div { style: "font-size: 11px; color: var(--text-faint); margin-top: 2px;", "v{exe.version}" }
                                            }
                                            div { 
                                                span { 
                                                    class: if is_success { "status-pill status-pill-success" } else { "status-pill" },
                                                    style: if !is_success { "background: #FEE2E2; color: #991B1B;" } else { "" },
                                                    "{exe.status}" 
                                                }
                                            }
                                            div {
                                                span { 
                                                    class: "status-pill", 
                                                    style: "background: var(--bg); color: var(--text-faint);",
                                                    "{exe.execution_mode}" 
                                                }
                                            }
                                            div { style: "font-family: 'IBM Plex Mono', monospace; font-size: 12px; color: var(--text-secondary);", "{exe.current_step}" }
                                            div { style: "font-size: 11px; color: var(--text-faint);", "{exe.created_at}" }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { style: "padding: 80px 0; text-align: center; color: var(--text-faint);", "Stabilizing ledger data..." }
                    }
                }
            }
        }
    }
}
