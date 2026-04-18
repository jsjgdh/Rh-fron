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
        match executions.read().as_ref() {
            Some(list) => {
                if list.is_empty() {
                    rsx! { div { class: "activity-empty", "No recent activity recorded." } }
                } else {
                    rsx! {
                for exe in list.iter().take(5) {
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
            None => rsx! { div { class: "loading-text", "Fetching activity..." } }
        }
    }
}

#[component]
fn ActivityRow(name: String, status: String, mode: String, created: String, id: String, on_click: EventHandler<()>) -> Element {
    let status_class = match status.as_str() {
        "Completed" => "badge badge-success",
        "Failed" => "badge badge-danger",
        _ => "badge badge-warning",
    };

    let mode_class = if mode == "Live" { "badge badge-primary" } else { "badge badge-outline" };

    let date_part = created.split('T').next().unwrap_or("").to_string();
    let id_short = id.split('-').next().unwrap_or("").to_string();

    rsx! {
        div { 
            class: "type-row interactive-row",
            onclick: move |_| on_click.call(()),
            div { class: "type-cell-code", "{name}" }
            div {
                span { class: "{status_class}", "{status}" }
                span { class: "{mode_class}", style: "margin-left: 8px;", "{mode}" }
            }
            div { class: "type-cell-small", "{date_part}" }
            div { class: "type-cell-small type-cell-code", "{id_short}..." }
        }
    }
}
