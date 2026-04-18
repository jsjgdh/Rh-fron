use dioxus::prelude::*;
use crate::api;

#[derive(Props, PartialEq, Clone)]
pub struct ExecutionHistoryProps {
    pub on_select: EventHandler<String>,
}

#[component]
pub fn ExecutionHistory(props: ExecutionHistoryProps) -> Element {
    let executions = use_resource(|| async move {
        api::list_recent_executions().await.unwrap_or_default()
    });

    let history = executions.read();
    
    rsx! {
        div { class: "dashboard-stack",
            section { class: "card",
                div { class: "card-header",
                    div {
                        div { class: "card-title", "Historical Execution Ledger" }
                        div { class: "card-description", "Review every trace recorded by the Rhexiom cluster." }
                    }
                }

                div { class: "type-table",
                    div { class: "type-row type-row-5 type-row-head",
                        div { "Policy Workflow" }
                        div { "Final Status" }
                        div { "Mode" }
                        div { "Terminal Step" }
                        div { "Timestamp" }
                    }

                    if let Some(h_list) = history.as_ref() {
                        if h_list.is_empty() {
                            div { class: "status-message", "No executions recorded in this workspace." }
                        } else {
                            for exe in h_list {
                                {
                                    let id = exe.execution_id.clone();
                                    let status_lower = exe.status.to_lowercase();
                                    let mode_class = if exe.execution_mode == "Live" { "badge badge-primary" } else { "badge badge-outline" };
                                    rsx! {
                                        div { 
                                            class: "type-row type-row-5 interactive-row",
                                            style: "cursor: pointer;",
                                            onclick: move |_| props.on_select.call(id.clone()),
                                            div { 
                                                div { style: "font-weight: 600;", "{exe.workflow_name}" }
                                                div { class: "stat-note", "v{exe.version}" }
                                            }
                                            div { 
                                                span { class: "badge badge-{status_lower}", "{exe.status}" }
                                            }
                                            div {
                                                span { class: "{mode_class}", "{exe.execution_mode}" }
                                            }
                                            div { class: "text-secondary", style: "font-size: 13px;", "{exe.current_step}" }
                                            div { class: "stat-note", "{exe.created_at}" }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "status-message", "Loading ledger data..." }
                    }
                }
            }
        }
    }
}
