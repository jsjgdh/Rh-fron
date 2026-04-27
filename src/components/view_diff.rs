use dioxus::prelude::*;
use crate::api;
use crate::api::ChangeType;

#[component]
pub fn ViewDiff(name: String, v1: String, v2: String) -> Element {
    let name_c1 = name.clone();
    let v1_c = v1.clone();
    let d1 = use_resource(move || {
        let name = name_c1.clone();
        let v1 = v1_c.clone();
        async move { api::get_workflow_detail(&name, &v1).await.ok() }
    });

    let name_c2 = name.clone();
    let v2_c = v2.clone();
    let d2 = use_resource(move || {
        let name = name_c2.clone();
        let v2 = v2_c.clone();
        async move { api::get_workflow_detail(&name, &v2).await.ok() }
    });

    // Fetch the actual diff data
    let name_diff = name.clone();
    let v1_diff = v1.clone();
    let v2_diff = v2.clone();
    let diff = use_resource(move || {
        let name = name_diff.clone();
        let v1 = v1_diff.clone();
        let v2 = v2_diff.clone();
        async move { api::compare_workflows(&name, &v1, &v2).await.ok() }
    });

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "VISUAL DIFF" }
            p { style: "color: var(--text-secondary); margin-bottom: 40px;",
                "Comparing {name} v{v1} → v{v2}"
            }

            // Breaking change warning
            if let Some(Some(diff_data)) = diff.read().as_ref() {
                if diff_data.breaking_changes {
                    div {
                        class: "card",
                        style: "background: rgba(239, 68, 68, 0.1); border-left: 4px solid #ef4444; margin-bottom: 24px;",
                        div { style: "display: flex; align-items: center; gap: 12px;",
                            span { style: "font-size: 24px;", "⚠️" }
                            div {
                                div { style: "font-weight: 700; color: #ef4444;", "BREAKING CHANGES DETECTED" }
                                div { style: "font-size: 13px; color: var(--text-secondary);",
                                    "This version contains changes that may break existing integrations."
                                }
                            }
                        }
                    }
                }
            }

            div { class: "grid-2",
                div { class: "card", style: "padding: 0;",
                    div { class: "section-title", style: "margin: 24px; border: none;", "VERSION {v1}" }
                    if let Some(Some(detail)) = d1.read().as_ref() {
                        pre {
                            class: "ide-panel",
                            style: "margin: 0; border-radius: 0; min-height: 500px; font-size: 12px;",
                            "{detail.get(\"source\").and_then(|v| v.as_str()).unwrap_or(\"\")}"
                        }
                    }
                }
                div { class: "card", style: "padding: 0;",
                    div { class: "section-title", style: "margin: 24px; border: none;", "VERSION {v2}" }
                    if let Some(Some(detail)) = d2.read().as_ref() {
                        pre {
                            class: "ide-panel",
                            style: "margin: 0; border-radius: 0; min-height: 500px; font-size: 12px; border-left: 2px solid var(--accent-primary);",
                            "{detail.get(\"source\").and_then(|v| v.as_str()).unwrap_or(\"\")}"
                        }
                    }
                }
            }

            // Change Summary
            div { class: "section-title", "CHANGE SUMMARY" }
            if let Some(Some(diff_data)) = diff.read().as_ref() {
                div { class: "card", style: "margin-bottom: 24px;",
                    p { style: "font-size: 16px; color: var(--text-primary); margin: 0;",
                        "{diff_data.summary}"
                    }
                }

                // Input Changes
                if !diff_data.input_changes.is_empty() {
                    div { class: "section-title", "INPUT CHANGES" }
                    div { class: "card", style: "margin-bottom: 24px;",
                        div { style: "display: flex; flex-direction: column; gap: 8px;",
                            for change in &diff_data.input_changes {
                                div {
                                    style: format!("padding: 12px; border-radius: 4px; display: flex; align-items: center; gap: 12px; {}",
                                        match change.change_type {
                                            ChangeType::Added => "background: rgba(34, 197, 94, 0.1); border-left: 3px solid #22c55e;",
                                            ChangeType::Removed => "background: rgba(239, 68, 68, 0.1); border-left: 3px solid #ef4444;",
                                            ChangeType::Modified => "background: rgba(234, 179, 8, 0.1); border-left: 3px solid #eab308;",
                                        }
                                    ),
                                    span { style: "font-size: 14px; font-weight: 600; min-width: 80px;",
                                        match change.change_type {
                                            ChangeType::Added => { "ADDED" },
                                            ChangeType::Removed => { "REMOVED" },
                                            ChangeType::Modified => { "MODIFIED" },
                                        }
                                    }
                                    span { style: "font-family: var(--font-mono); font-size: 14px;",
                                        "{change.field_name}"
                                    }
                                    if let Some(new_type) = &change.new_type {
                                        if let Some(old_type) = &change.old_type {
                                            span { style: "font-size: 12px; color: var(--text-secondary);",
                                                "({old_type} → {new_type})"
                                            }
                                        } else {
                                            span { style: "font-size: 12px; color: var(--text-secondary);",
                                                "({new_type})"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Step Changes
                if !diff_data.step_changes.is_empty() {
                    div { class: "section-title", "STEP CHANGES" }
                    div { class: "card", style: "margin-bottom: 24px;",
                        div { style: "display: flex; flex-direction: column; gap: 8px;",
                            for change in &diff_data.step_changes {
                                div {
                                    style: format!("padding: 12px; border-radius: 4px; display: flex; align-items: center; gap: 12px; {}",
                                        match change.change_type {
                                            ChangeType::Added => "background: rgba(34, 197, 94, 0.1); border-left: 3px solid #22c55e;",
                                            ChangeType::Removed => "background: rgba(239, 68, 68, 0.1); border-left: 3px solid #ef4444;",
                                            ChangeType::Modified => "background: rgba(234, 179, 8, 0.1); border-left: 3px solid #eab308;",
                                        }
                                    ),
                                    span { style: "font-size: 14px; font-weight: 600; min-width: 80px;",
                                        match change.change_type {
                                            ChangeType::Added => { "ADDED" },
                                            ChangeType::Removed => { "REMOVED" },
                                            ChangeType::Modified => { "MODIFIED" },
                                        }
                                    }
                                    span { style: "font-family: var(--font-mono); font-size: 14px;",
                                        "{change.step_name}"
                                    }
                                    if let Some(summary) = &change.body_diff_summary {
                                        span { style: "font-size: 12px; color: var(--text-secondary);",
                                            "({summary})"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Action Changes
                if !diff_data.action_changes.is_empty() {
                    div { class: "section-title", "ACTION CHANGES" }
                    div { class: "card", style: "margin-bottom: 24px;",
                        div { style: "display: flex; flex-direction: column; gap: 8px;",
                            for change in &diff_data.action_changes {
                                div {
                                    style: format!("padding: 12px; border-radius: 4px; display: flex; align-items: center; gap: 12px; {}",
                                        match change.change_type {
                                            ChangeType::Added => "background: rgba(34, 197, 94, 0.1); border-left: 3px solid #22c55e;",
                                            ChangeType::Removed => "background: rgba(239, 68, 68, 0.1); border-left: 3px solid #ef4444;",
                                            ChangeType::Modified => "background: rgba(234, 179, 8, 0.1); border-left: 3px solid #eab308;",
                                        }
                                    ),
                                    span { style: "font-size: 14px; font-weight: 600; min-width: 80px;",
                                        match change.change_type {
                                            ChangeType::Added => { "ADDED" },
                                            ChangeType::Removed => { "REMOVED" },
                                            ChangeType::Modified => { "MODIFIED" },
                                        }
                                    }
                                    span { style: "font-family: var(--font-mono); font-size: 14px;",
                                        "{change.action_name}"
                                    }
                                    span { style: "font-size: 12px; color: var(--text-secondary);",
                                        "in step {change.step_name}"
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Loading state
                div { class: "card",
                    p { style: "color: var(--text-secondary); text-align: center; padding: 40px;",
                        "Loading diff analysis..."
                    }
                }
            }
        }
    }
}
