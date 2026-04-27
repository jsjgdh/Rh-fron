//! Recent activity list component.

use dioxus::prelude::*;
use crate::api;

#[derive(Props, PartialEq, Clone)]
pub struct ActivityListProps {
    pub on_select: EventHandler<String>,
}

#[component]
pub fn ActivityList(props: ActivityListProps) -> Element {
    let executions = use_resource(|| async move {
        api::list_recent_executions().await.unwrap_or_default()
    });

    rsx! {
        div { class: "activity-container",
            match executions.read().as_ref() {
                Some(list) => {
                    if list.is_empty() {
                        rsx! { div { style: "padding: 40px; text-align: center; color: var(--text-faint); font-size: 14px;", "No activity records found." } }
                    } else {
                        rsx! {
                            for exe in list.iter().take(10) {
                                ActivityRow { 
                                    name: exe.workflow_name.clone(),
                                    status: exe.status.clone(),
                                    mode: exe.execution_mode.clone(),
                                    created: exe.created_at.clone(),
                                    id: exe.execution_id.clone(),
                                    on_click: {
                                        let id = exe.execution_id.clone();
                                        move |_| props.on_select.call(id.clone())
                                    }
                                }
                            }
                        }
                    }
                }
                None => rsx! {
                    div { style: "padding: 40px; text-align: center;",
                        div { class: "spinner", style: "margin: 0 auto 16px;" }
                        div { style: "color: var(--text-faint); font-size: 14px;", "Synchronizing records..." }
                    }
                }
            }
        }
    }
}

#[component]
fn ActivityRow(name: String, status: String, mode: String, created: String, id: String, on_click: EventHandler<()>) -> Element {
    let status_class = match status.as_str() {
        "Completed" => "status-pill status-pill-success",
        "Failed" => "status-pill status-pill-danger",
        _ => "status-pill",
    };

    let date_part = created.split('T').next().unwrap_or("").to_string();
    let id_short = id.split('-').next().unwrap_or("").to_string();

    rsx! {
        div {
            class: "activity-row",
            onclick: move |_| on_click.call(()),
            div { style: "display: flex; align-items: center; gap: 12px;",
                span { style: "font-size: 14px; color: var(--text-primary); font-weight: 500;", "{name}" }
                span { 
                    class: "{status_class}",
                    "{status}" 
                }
            }
            div { style: "display: flex; align-items: center; gap: 20px;",
                span { style: "font-size: 12px; color: var(--text-faint);", "{date_part}" }
                span {
                    class: "mono",
                    style: "font-size: 11px; opacity: 0.7;",
                    aria_label: "Execution ID",
                    "{id_short}"
                }
            }
        }
    }
}
