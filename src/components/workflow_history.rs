use dioxus::prelude::*;
use crate::api::{get_workflow_history, WorkflowHistoryEntry};
use crate::app::Route;

#[component]
pub fn WorkflowHistory(name: String, current_version: String) -> Element {
    let nav = use_navigator();
    let name_for_history = name.clone();
    let history = use_resource(move || {
        let name = name_for_history.clone();
        async move { get_workflow_history(&name).await.ok() }
    });

    rsx! {
        div { class: "card",
            div { class: "section-title", style: "margin-top: 0;", "VERSION HISTORY" }
            if let Some(Some(history_data)) = history.read().as_ref() {
                div { style: "display: flex; flex-direction: column; gap: 12px;",
                    for entry in history_data.iter() {
                        HistoryEntry {
                            entry: entry.clone(),
                            is_current: entry.version == current_version,
                            name: name.clone(),
                        }
                    }
                }
            } else {
                div { style: "text-align: center; padding: 40px;",
                    p { style: "color: var(--text-faint);", "Loading history..." }
                }
            }
        }
    }
}

#[component]
fn HistoryEntry(entry: WorkflowHistoryEntry, is_current: bool, name: String) -> Element {
    let nav = use_navigator();
    let entry_clone = entry.clone();
    let name_for_diff = name.clone();
    let name_for_diff2 = name.clone();

    rsx! {
        div {
            style: format!("padding: 16px; border-radius: 4px; border: 1px solid var(--border); {}",
                if is_current { "background: rgba(59, 130, 246, 0.1);" } else { "" }
            ),
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",
                div { style: "display: flex; align-items: center; gap: 12px;",
                    span { class: "badge",
                        style: if is_current {
                            "background: var(--accent-primary); color: white;"
                        } else {
                            "background: var(--bg);"
                        },
                        "v{entry.version}"
                    }
                    if entry.is_breaking_change {
                        span { class: "badge",
                            style: "background: rgba(239, 68, 68, 0.2); color: #ef4444;",
                            "⚠ BREAKING"
                        }
                    }
                }
                span { style: "font-size: 12px; color: var(--text-faint);",
                    "{entry.created_at.split('T').next().unwrap_or(&entry.created_at)}"
                }
            }
            if !entry.changelog.is_empty() {
                p { style: "font-size: 13px; color: var(--text-secondary); margin: 8px 0;",
                    "{entry.changelog}"
                }
            }
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-top: 12px;",
                span { style: "font-size: 12px; color: var(--text-faint);",
                    "by {entry.created_by.chars().take(8).collect::<String>()}..."
                }
                if let Some(prev) = entry.previous_version.clone() {
                    div { style: "display: flex; gap: 8px;",
                        button {
                            class: "btn btn-ghost",
                            style: "padding: 4px 12px; font-size: 12px;",
                            onclick: move |_| {
                                let name = name_for_diff.clone();
                                let prev = prev.clone();
                                let v2 = entry_clone.version.clone();
                                nav.push(Route::ViewDiff { name, v1: prev, v2 });
                            },
                            "View Diff"
                        }
                    }
                }
            }
        }
    }
}
