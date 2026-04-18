use dioxus::prelude::*;
use crate::components::workflow_graph::WorkflowGraph;
use crate::api::{WorkflowSummary, get_workflow_detail};

#[component]
pub fn Visualize() -> Element {
    let mut workflows = use_signal(Vec::<WorkflowSummary>::new);
    let mut selected_workflow = use_signal(|| Option::<String>::None);
    let mut selected_version = use_signal(|| Option::<String>::None);
    let mut workflow_detail = use_signal(|| Option::<serde_json::Value>::None);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    // Initial fetch of workflows
    let _ = use_resource(move || async move {
        if let Ok(list) = crate::api::list_workflows().await {
            workflows.set(list);
        }
    });

    // Fetch details when selection changes
    let _ = use_resource(move || {
        let name = selected_workflow.read().clone();
        let version = selected_version.read().clone();
        async move {
            if let (Some(n), Some(v)) = (name, version) {
                loading.set(true);
                match get_workflow_detail(&n, &v).await {
                    Ok(detail) => {
                        workflow_detail.set(Some(detail));
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        workflow_detail.set(None);
                    }
                }
                loading.set(false);
            }
        }
    });

    rsx! {
        div { class: "page-stack",
            section { class: "card",
                div { class: "card-header",
                    div {
                        div { class: "card-title", "Selection" }
                        div { class: "card-description", "Choose a workflow and version to visualize the logic." }
                    }
                }

                div { class: "grid-2",
                    div { class: "form-group",
                        label { class: "form-label", "Workflow" }
                        select {
                            class: "form-input",
                            onchange: move |evt| {
                                let val = evt.value();
                                selected_workflow.set(Some(val.clone()));
                                // Reset version when workflow changes
                                if let Some(w) = workflows.read().iter().find(|w| w.name == val) {
                                    selected_version.set(w.versions.first().cloned());
                                }
                            },
                            option { value: "", "Select a workflow..." }
                            for w in workflows.read().iter() {
                                option { value: "{w.name}", "{w.name}" }
                            }
                        }
                    }

                    if let Some(w_name) = selected_workflow.read().as_ref() {
                        div { class: "form-group",
                            label { class: "form-label", "Version" }
                            select {
                                class: "form-input",
                                value: selected_version.read().clone().unwrap_or_default(),
                                onchange: move |evt| selected_version.set(Some(evt.value())),
                                for v in workflows.read().iter().find(|w| &w.name == w_name).map(|w| &w.versions).unwrap_or(&vec![]) {
                                    option { value: "{v}", "{v}" }
                                }
                            }
                        }
                    }
                }
            }

            if *loading.read() {
                div { class: "status-message", "Fetching workflow graph symbols..." }
            } else if let Some(err) = error.read().as_ref() {
                div { class: "status-message status-message-error", "{err}" }
            } else if let Some(detail) = workflow_detail.read().as_ref() {
                if let Some(ast_str) = detail.get("ast_json").and_then(|v| v.as_str()) {
                    if let Ok(ast) = serde_json::from_str::<serde_json::Value>(ast_str) {
                        section { class: "card graph-card",
                            div { class: "card-header",
                                div {
                                    div { class: "card-title", "Process Logic" }
                                    div { class: "card-description", "Interactive trace of the compiled policy logic." }
                                }
                            }
                            WorkflowGraph { injected_ast: ast }
                        }
                    }
                }
            } else {
                div { class: "empty-state",
                    div { class: "empty-state-icon", "✧" }
                    div { class: "empty-state-text", "Select a workflow to begin visualization" }
                }
            }
        }
    }
}
