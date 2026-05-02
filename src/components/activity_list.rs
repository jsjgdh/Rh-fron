//! Recent activity list component with search and filtering.

use dioxus::prelude::*;
use crate::api::{self, ExecutionSummary};
use chrono;

#[derive(Props, PartialEq, Clone)]
pub struct ActivityListProps {
    pub on_select: EventHandler<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateRange {
    Last24h,
    Last7d,
    Last30d,
    Custom(String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusFilter {
    All,
    Success,
    Failed,
    Running,
    Suspended,
}

#[component]
pub fn ActivityList(props: ActivityListProps) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let mut date_range = use_signal(|| DateRange::Last24h);
    let mut status_filter = use_signal(|| StatusFilter::All);
    let mut is_filter_open = use_signal(|| false);
    let mut cursor = use_signal(|| Option::<String>::None);
    let mut has_more = use_signal(|| true);
    let mut is_loading_more = use_signal(|| false);

    let executions = use_resource(move || {
        let q = search_query.read().clone();
        let dr = date_range.read().clone();
        let sf = status_filter.read().clone();
        let c = cursor.read().clone();
        async move {
            let mut results = api::list_recent_executions(if q.is_empty() { None } else { Some(&q) }, c.as_deref()).await.unwrap_or_default();

            results = filter_by_date_range(results, &dr);
            results = filter_by_status(results, &sf);

            if results.len() < 50 {
                has_more.set(false);
            }

            results
        }
    });

    let load_more = move |_: MouseEvent| {
        if !*is_loading_more.read() && *has_more.read() {
            is_loading_more.set(true);
            if let Some(execs) = executions.read().as_ref() {
                if let Some(last_exec) = execs.last() {
                    cursor.set(Some(last_exec.execution_id.clone()));
                }
            }
            is_loading_more.set(false);
        }
    };

    rsx! {
        div { class: "activity-container",
            div { style: "padding: 16px; border-bottom: 1px solid var(--border);",
                div { style: "display: flex; gap: 12px; margin-bottom: 12px;",
                    input {
                        class: "search-input",
                        style: "flex: 1; padding: 8px 12px; border-radius: 6px; border: 1px solid var(--border); background: var(--bg-card); color: var(--text-primary); font-size: 13px;",
                        placeholder: "Search by workflow name or execution ID...",
                        value: "{search_query}",
                        oninput: move |evt| search_query.set(evt.value())
                    }
                    button {
                        class: "btn btn-ghost",
                        style: "padding: 8px 12px;",
                        onclick: move |_| is_filter_open.set(!is_filter_open()),
                        if *is_filter_open.read() { "✕ Filters" } else { "☰ Filters" }
                    }
                }

                if *is_filter_open.read() {
                    div { style: "display: flex; flex-wrap: wrap; gap: 16px; padding: 12px; background: var(--bg-secondary); border-radius: 8px;",
                        div { style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 11px; font-weight: 700; text-transform: uppercase; color: var(--text-faint);", "Date Range" }
                            select {
                                class: "form-input",
                                style: "padding: 6px 12px; font-size: 13px; margin-bottom: 0;",
                                value: match *date_range.read() {
                                    DateRange::Last24h => "24h",
                                    DateRange::Last7d => "7d",
                                    DateRange::Last30d => "30d",
                                    DateRange::Custom(_, _) => "custom",
                                },
                                onchange: move |evt| {
                                    date_range.set(match evt.value().as_str() {
                                        "24h" => DateRange::Last24h,
                                        "7d" => DateRange::Last7d,
                                        "30d" => DateRange::Last30d,
                                        _ => DateRange::Last24h,
                                    })
                                },
                                option { value: "24h", "Last 24 hours" }
                                option { value: "7d", "Last 7 days" }
                                option { value: "30d", "Last 30 days" }
                            }
                        }

                        div { style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 11px; font-weight: 700; text-transform: uppercase; color: var(--text-faint);", "Status" }
                            div { style: "display: flex; gap: 6px;",
                                StatusPill {
                                    label: "All",
                                    active: matches!(*status_filter.read(), StatusFilter::All),
                                    on_click: move |_| status_filter.set(StatusFilter::All)
                                }
                                StatusPill {
                                    label: "Success",
                                    active: matches!(*status_filter.read(), StatusFilter::Success),
                                    on_click: move |_| status_filter.set(StatusFilter::Success)
                                }
                                StatusPill {
                                    label: "Failed",
                                    active: matches!(*status_filter.read(), StatusFilter::Failed),
                                    on_click: move |_| status_filter.set(StatusFilter::Failed)
                                }
                                StatusPill {
                                    label: "Running",
                                    active: matches!(*status_filter.read(), StatusFilter::Running),
                                    on_click: move |_| status_filter.set(StatusFilter::Running)
                                }
                            }
                        }
                    }
                }
            }

            match executions.read().as_ref() {
                Some(list) => {
                    if list.is_empty() {
                        rsx! { div { style: "padding: 40px; text-align: center; color: var(--text-faint); font-size: 14px;", "No activity records found." } }
                    } else {
                        rsx! {
                            for exe in list.iter().take(50) {
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
fn StatusPill(label: &'static str, active: bool, on_click: EventHandler<()>) -> Element {
    let class = if active {
        "status-pill status-pill-success"
    } else {
        "status-pill"
    };

    rsx! {
        button {
            class: "{class}",
            style: "cursor: pointer; padding: 4px 10px;",
            onclick: move |_| on_click.call(()),
            "{label}"
        }
    }
}

fn filter_by_date_range(mut executions: Vec<ExecutionSummary>, range: &DateRange) -> Vec<ExecutionSummary> {
    let now = chrono::Utc::now();
    let cutoff = match range {
        DateRange::Last24h => now - chrono::Duration::hours(24),
        DateRange::Last7d => now - chrono::Duration::days(7),
        DateRange::Last30d => now - chrono::Duration::days(30),
        DateRange::Custom(start, _) => {
            chrono::DateTime::parse_from_rfc3339(start)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or(now - chrono::Duration::days(30))
        }
    };

    executions.retain(|e| {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&e.created_at) {
            dt.with_timezone(&chrono::Utc) > cutoff
        } else {
            true
        }
    });

    executions
}

fn filter_by_status(executions: Vec<ExecutionSummary>, status: &StatusFilter) -> Vec<ExecutionSummary> {
    match status {
        StatusFilter::All => executions,
        StatusFilter::Success => executions.into_iter().filter(|e| e.status == "Completed").collect(),
        StatusFilter::Failed => executions.into_iter().filter(|e| e.status == "Failed").collect(),
        StatusFilter::Running => executions.into_iter().filter(|e| e.status == "Running").collect(),
        StatusFilter::Suspended => executions.into_iter().filter(|e| e.status == "Suspended").collect(),
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ActivityRowProps {
    pub name: String,
    pub status: String,
    pub mode: String,
    pub created: String,
    pub id: String,
    pub on_click: EventHandler<()>,
}

#[component]
fn ActivityRow(props: ActivityRowProps) -> Element {
    let status_class = match props.status.as_str() {
        "Completed" => "status-pill status-pill-success",
        "Failed" => "status-pill status-pill-danger",
        "Running" => "status-pill status-pill-info",
        "Suspended" => "status-pill status-pill-warning",
        _ => "status-pill",
    };

    let date_part = props.created.split('T').next().unwrap_or("").to_string();
    let id_short = props.id.split('-').next().unwrap_or("").to_string();

    rsx! {
        div {
            class: "activity-row",
            role: "button",
            tabindex: "0",
            onclick: move |_| props.on_click.call(()),
            onkeydown: move |e| {
                if let dioxus::prelude::Key::Enter = e.key() {
                    props.on_click.call(());
                }
                if let dioxus::prelude::Key::Character(c) = e.key() {
                    if c.as_str() == " " {
                        props.on_click.call(());
                    }
                }
            },
            div { style: "display: flex; align-items: center; gap: 12px;",
                span { style: "font-size: 14px; color: var(--text-primary); font-weight: 500;", "{props.name}" }
                span {
                    class: "{status_class}",
                    "{props.status}"
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