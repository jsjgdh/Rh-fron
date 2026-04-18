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
        div { class: "page-stack",
            section { class: "card detail-hero",
                div { class: "detail-hero-head",
                    div {
                        div { class: "section-kicker", "Artifact ledger" }
                        h2 { class: "section-title", "Inspect every stored workflow release." }
                        p { class: "section-copy", "Browse deployed versions and switch between source, AST, and IR payloads without leaving the workspace." }
                    }
                    span { class: "badge badge-neutral", "archive" }
                }
            }

            div { class: "grid-2 version-layout",
                section { class: "card",
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Stored workflows" }
                            div { class: "card-description", "Select a version to inspect its release artifacts." }
                        }
                    }

                    div { class: "version-list",
                        if let Some(workflows) = workflows_res.read().as_ref() {
                            if workflows.is_empty() {
                                div { class: "empty-state subtle",
                                    div { class: "empty-state-icon", "00" }
                                    div { class: "empty-state-text", "No workflow versions have been stored yet." }
                                }
                            } else {
                                for workflow in workflows {
                                    for version in &workflow.versions {
                                        div {
                                            class: if selected_workflow.read().as_ref() == Some(&workflow.name)
                                                && selected_version.read().as_ref() == Some(version) {
                                                "version-item selected"
                                            } else {
                                                "version-item"
                                            },
                                            onclick: {
                                                let workflow_name = workflow.name.clone();
                                                let version_name = version.clone();
                                                move |_| {
                                                    selected_workflow.set(Some(workflow_name.clone()));
                                                    selected_version.set(Some(version_name.clone()));
                                                }
                                            },
                                            div {
                                                div { class: "version-id", "{workflow.name}" }
                                                div { class: "version-meta", "{version}" }
                                            }
                                            span { class: "badge badge-success", "stored" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                section { class: "card",
                    div { class: "card-header",
                        div {
                            if let (Some(workflow), Some(version)) =
                                (selected_workflow.read().clone(), selected_version.read().clone())
                            {
                                div { class: "card-title", "{workflow} · {version}" }
                            } else {
                                div { class: "card-title", "Select a version" }
                            }
                            div { class: "card-description", "Switch between release artifacts and inspect the stored payloads." }
                        }
                    }

                    if selected_workflow.read().is_some() {
                        div { class: "artifact-stack",
                            div { class: "tab-bar",
                                div {
                                    class: if *active_tab.read() == "source" { "tab active" } else { "tab" },
                                    onclick: move |_| active_tab.set("source".to_string()),
                                    "Source"
                                }
                                div {
                                    class: if *active_tab.read() == "ast" { "tab active" } else { "tab" },
                                    onclick: move |_| active_tab.set("ast".to_string()),
                                    "AST"
                                }
                                div {
                                    class: if *active_tab.read() == "ir" { "tab active" } else { "tab" },
                                    onclick: move |_| active_tab.set("ir".to_string()),
                                    "IR"
                                }
                            }

                            div { class: "code-block artifact-block",
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
                                    "Loading artifacts..."
                                }
                            }
                        }
                    } else {
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "01" }
                            div { class: "empty-state-text", "Choose a workflow release from the left to inspect its artifacts." }
                        }
                    }
                }
            }
        }
    }
}
