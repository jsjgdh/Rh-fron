//! Version list and comparison component.

use dioxus::prelude::*;

/// Version management view.
#[component]
pub fn VersionList() -> Element {
    let mut selected_workflow = use_signal(|| Option::<String>::None);
    let mut selected_version = use_signal(|| Option::<String>::None);
    let mut active_tab = use_signal(|| "source".to_string());

    let workflows_res =
        use_resource(|| async move { crate::api::list_workflows().await.unwrap_or_default() });

    let detail_res = use_resource(move || async move {
        if let (Some(workflow), Some(version)) = (
            selected_workflow.read().clone(),
            selected_version.read().clone(),
        ) {
            crate::api::get_workflow_detail(&workflow, &version)
                .await
                .ok()
        } else {
            None
        }
    });

    rsx! {
        div { class: "dashboard-stack control-artifacts",
            section { class: "industrial-card glass detail-hero",
                div { style: "display: flex; justify-content: space-between; align-items: center;",
                    div {
                        div { class: "label-caps", style: "color: var(--accent);", "Artifact ledger" }
                        h2 { class: "app-title", style: "font-size: 24px; margin-top: 8px;", "Inspect every stored workflow release." }
                        p { class: "panel-copy", style: "margin-top: 12px; color: var(--text-secondary);", "Browse deployed versions and switch between source, AST, and IR payloads without leaving the workspace." }
                    }
                    span { class: "status-pill", style: "background: var(--bg); color: var(--text-faint);", "archive" }
                }
            }

            div { class: "grid-metrics", style: "grid-template-columns: 1fr 1.6fr;",
                section { class: "industrial-card",
                    div { class: "label-caps", "Stored releases" }
                    
                    div { class: "type-table", style: "margin-top: 24px;",
                        if let Some(workflows) = workflows_res.read().as_ref() {
                            if workflows.is_empty() {
                                div { style: "padding: 80px 0; text-align: center; color: var(--text-faint);",
                                    div { class: "label-caps", style: "font-size: 24px; opacity: 0.1;", "EMPTY" }
                                    p { style: "font-size: 14px; margin-top: 12px;", "No workflow versions have been stored yet." }
                                }
                            } else {
                                for workflow in workflows {
                                    for version in &workflow.versions {
                                        {
                                            let is_active = selected_workflow.read().as_ref() == Some(&workflow.name)
                                                && selected_version.read().as_ref() == Some(version);
                                            let workflow_name = workflow.name.clone();
                                            let version_name = version.clone();
                                            rsx! {
                                                div {
                                                    class: "type-row",
                                                    style: if is_active { "background: var(--bg); cursor: default;" } else { "cursor: pointer;" },
                                                    onclick: move |_| {
                                                        selected_workflow.set(Some(workflow_name.clone()));
                                                        selected_version.set(Some(version_name.clone()));
                                                    },
                                                    div { style: "flex: 1;",
                                                        div { style: "font-weight: 700; color: var(--text-primary);", "{workflow.name}" }
                                                        div { style: "font-size: 11px; color: var(--text-faint); margin-top: 2px;", "Release {version}" }
                                                    }
                                                    div {
                                                        span { 
                                                            class: "status-pill", 
                                                            style: if is_active { "background: var(--accent); color: white;" } else { "background: var(--bg); color: var(--text-faint);" },
                                                            if is_active { "Inspecting" } else { "Stored" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                section { class: "industrial-card",
                    div { style: "display: flex; justify-content: space-between; align-items: flex-start;",
                        div {
                            if let (Some(workflow), Some(version)) =
                                (selected_workflow.read().clone(), selected_version.read().clone())
                            {
                                div { class: "label-caps", "{workflow} version ledger" }
                                h3 { class: "app-title", style: "font-size: 20px; margin-top: 8px;", "Release {version}" }
                            } else {
                                div { class: "label-caps", "Artifact Explorer" }
                                h3 { class: "app-title", style: "font-size: 20px; margin-top: 8px;", "Select a version" }
                            }
                        }
                    }

                    if selected_workflow.read().is_some() {
                        div { style: "margin-top: 32px;",
                            div { class: "tabs-header",
                                div {
                                    class: if *active_tab.read() == "source" { "tab-item active" } else { "tab-item" },
                                    onclick: move |_| active_tab.set("source".to_string()),
                                    "Source"
                                }
                                div {
                                    class: if *active_tab.read() == "ast" { "tab-item active" } else { "tab-item" },
                                    onclick: move |_| active_tab.set("ast".to_string()),
                                    "AST"
                                }
                                div {
                                    class: if *active_tab.read() == "ir" { "tab-item active" } else { "tab-item" },
                                    onclick: move |_| active_tab.set("ir".to_string()),
                                    "IR"
                                }
                            }

                            div { 
                                style: "margin-top: 24px; padding: 24px; background: var(--panel-lighter); border: 1px solid var(--border); border-radius: var(--radius-sm); font-family: 'IBM Plex Mono', monospace; font-size: 13px; color: var(--text-primary); white-space: pre-wrap; min-height: 400px; line-height: 1.6;",
                                if let Some(Some(detail)) = detail_res.read().as_ref() {
                                    if *active_tab.read() == "source" {
                                        if let Some(source) = detail.get("source").and_then(|value| value.as_str()) {
                                            "{source}"
                                        }
                                    } else if *active_tab.read() == "ast" {
                                        if let Some(ast_str) = detail.get("ast_json").and_then(|value| value.as_str()) {
                                            if let Ok(ast) = serde_json::from_str::<serde_json::Value>(ast_str) {
                                                if let Ok(pretty) = serde_json::to_string_pretty(&ast) {
                                                    "{pretty}"
                                                } else {
                                                    "{ast_str}"
                                                }
                                            } else {
                                                "{ast_str}"
                                            }
                                        }
                                    } else if let Some(ir_str) = detail.get("ir_json").and_then(|value| value.as_str()) {
                                        if let Ok(ir) = serde_json::from_str::<serde_json::Value>(ir_str) {
                                            if let Ok(pretty) = serde_json::to_string_pretty(&ir) {
                                                "{pretty}"
                                            } else {
                                                "{ir_str}"
                                            }
                                        } else {
                                            "{ir_str}"
                                        }
                                    }
                                } else {
                                    "Synchronizing release artifacts..."
                                }
                            }
                        }
                    } else {
                        div { style: "margin-top: 120px; text-align: center; color: var(--text-faint);",
                            div { class: "label-caps", style: "font-size: 24px; opacity: 0.1;", "01" }
                            p { style: "font-size: 14px; margin-top: 12px;", "Choose a workflow release from the left to inspect its artifacts." }
                        }
                    }
                }
            }
        }
    }
}
