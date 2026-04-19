//! Execution form component.
//!
//! Auto-generates a form from the workflow's input schema, allows the user
//! to submit values, and displays the execution result + trace.

use dioxus::prelude::*;

/// Execution form view — fill inputs, execute, and see results.
#[component]
pub fn ExecutionForm() -> Element {
    let mut selected_workflow = use_signal(|| Option::<String>::None);
    let mut selected_version = use_signal(|| Option::<String>::None);
    let mut form_inputs = use_signal(|| std::collections::HashMap::<String, String>::new());
    let mut execution_result = use_signal(|| Option::<ExecutionResultData>::None);

    let workflows_res =
        use_resource(|| async move { crate::api::list_workflows().await.unwrap_or_default() });

    let detail_res = use_resource(move || {
        let workflow = selected_workflow.read().clone();
        let version = selected_version.read().clone();
        async move {
            if let (Some(workflow), Some(version)) = (workflow, version) {
                crate::api::get_workflow_detail(&workflow, &version)
                    .await
                    .ok()
            } else {
                None
            }
        }
    });

    rsx! {
        div { class: "page-stack",
            section { class: "card detail-hero",
                div { class: "detail-hero-head",
                    div {
                        div { class: "section-kicker", "Execution console" }
                        h2 { class: "section-title", "Run a stored workflow version with typed inputs." }
                        p { class: "section-copy", "Select a workflow deployment, provide schema-aware values, and inspect the exact trace returned by the runtime." }
                    }
                    span { class: "badge badge-neutral", "runtime" }
                }
            }

            div { class: "grid-2 execution-layout",
                section { class: "card" ,
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Input schema" }
                            div { class: "card-description",
                                if let (Some(workflow), Some(version)) = (
                                    selected_workflow.read().clone(),
                                    selected_version.read().clone(),
                                ) {
                                    "{workflow} · {version}"
                                } else {
                                    "Choose a workflow version to populate the form."
                                }
                            }
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Workflow version" }
                        select {
                            class: "form-input",
                            onchange: move |e| {
                                let value = e.value();
                                let mut parts = value.split(':');
                                if let (Some(workflow), Some(version)) = (parts.next(), parts.next()) {
                                    selected_workflow.set(Some(workflow.to_string()));
                                    selected_version.set(Some(version.to_string()));
                                    form_inputs.write().clear();
                                    execution_result.set(None);
                                }
                            },
                            option { value: "", "Select a workflow" }
                            if let Some(list) = workflows_res.read().as_ref() {
                                for workflow in list {
                                    for version in &workflow.versions {
                                        option {
                                            value: "{workflow.name}:{version}",
                                            selected: selected_workflow.read().as_ref() == Some(&workflow.name)
                                                && selected_version.read().as_ref() == Some(version),
                                            "{workflow.name} ({version})"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(Some(detail)) = detail_res.read().as_ref() {
                        if let Some(ast_str) = detail.get("ast_json").and_then(|value| value.as_str()) {
                            if let Ok(ast) = serde_json::from_str::<serde_json::Value>(ast_str) {
                                if let Some(inputs) = ast.get("inputs").and_then(|value| value.as_array()) {
                                    div { class: "dynamic-form",
                                        {
                                            inputs.iter().map(|input| {
                                                let name = input
                                                    .get("name")
                                                    .and_then(|value| value.as_str())
                                                    .unwrap_or_default()
                                                    .to_string();
                                                let field_type = input
                                                    .get("field_type")
                                                    .and_then(|value| value.as_str())
                                                    .unwrap_or_default()
                                                    .to_string();
                                                let current_value = form_inputs
                                                    .read()
                                                    .get(&name)
                                                    .cloned()
                                                    .unwrap_or_default();

                                                if field_type == "Boolean" {
                                                    rsx! {
                                                        div { class: "form-group",
                                                            label { class: "form-label",
                                                                "{name}"
                                                                span { class: "form-label-type", "boolean" }
                                                            }
                                                            div { class: "toggle-group",
                                                                button {
                                                                    class: if current_value == "true" {
                                                                        "toggle-option active"
                                                                    } else {
                                                                        "toggle-option"
                                                                    },
                                                                    onclick: {
                                                                        let name = name.clone();
                                                                        move |_| {
                                                                            form_inputs.write().insert(name.clone(), "true".to_string());
                                                                        }
                                                                    },
                                                                    "true"
                                                                }
                                                                button {
                                                                    class: if current_value == "false" || current_value.is_empty() {
                                                                        "toggle-option active"
                                                                    } else {
                                                                        "toggle-option"
                                                                    },
                                                                    onclick: {
                                                                        let name = name.clone();
                                                                        move |_| {
                                                                            form_inputs.write().insert(name.clone(), "false".to_string());
                                                                        }
                                                                    },
                                                                    "false"
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    rsx! {
                                                        div { class: "form-group",
                                                            label { class: "form-label",
                                                                "{name}"
                                                                span { class: "form-label-type", "{field_type.to_lowercase()}" }
                                                            }
                                                            input {
                                                                class: "form-input",
                                                                r#type: if field_type == "Number" { "number" } else { "text" },
                                                                value: "{current_value}",
                                                                placeholder: "Enter {name}",
                                                                oninput: {
                                                                    let name = name.clone();
                                                                    move |e: Event<FormData>| {
                                                                        form_inputs.write().insert(name.clone(), e.value());
                                                                    }
                                                                },
                                                            }
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "empty-state subtle",
                            div { class: "empty-state-icon", "01" }
                            div { class: "empty-state-text", "Pick a workflow version to generate the input form." }
                        }
                    }

                    div { class: "divider" }

                    button {
                        class: "btn btn-primary full-width",
                        disabled: selected_workflow.read().is_none(),
                        onclick: move |_| {
                            let workflow = selected_workflow.read().clone();
                            let version = selected_version.read().clone();
                            let detail = detail_res.read().clone();
                            let inputs_map = form_inputs.read().clone();

                            if let (Some(workflow), Some(version), Some(Some(detail))) = (workflow, version, detail) {
                                execution_result.set(None);

                                spawn(async move {
                                    let mut execution_input = std::collections::HashMap::new();

                                    if let Some(ast_str) = detail.get("ast_json").and_then(|value| value.as_str()) {
                                        if let Ok(ast) = serde_json::from_str::<serde_json::Value>(ast_str) {
                                            if let Some(inputs) = ast.get("inputs").and_then(|value| value.as_array()) {
                                                for input in inputs {
                                                    let name = input
                                                        .get("name")
                                                        .and_then(|value| value.as_str())
                                                        .unwrap_or_default();
                                                    let field_type = input
                                                        .get("field_type")
                                                        .and_then(|value| value.as_str())
                                                        .unwrap_or_default();
                                                    let value = inputs_map.get(name).cloned().unwrap_or_default();

                                                    let parsed_value = match field_type {
                                                        "Number" => serde_json::json!(value.parse::<f64>().unwrap_or(0.0)),
                                                        "Boolean" => serde_json::json!(value == "true"),
                                                        _ => serde_json::json!(value),
                                                    };

                                                    execution_input.insert(name.to_string(), parsed_value);
                                                }
                                            }
                                        }
                                    }

                                    let request = crate::api::RunRequest {
                                        workflow_name: workflow,
                                        version,
                                        input: execution_input,
                                        execution_mode: "Live".to_string(),
                                    };

                                    match crate::api::run_workflow(&request).await {
                                        Ok(response) => {
                                            let total_duration_us = response.trace.last().map(|s| s.timestamp_us).unwrap_or(0);
                                            let steps = response
                                                .trace
                                                .into_iter()
                                                .map(|step| (step.step_name, step.action, step.timestamp_us))
                                                .collect();

                                            execution_result.set(Some(ExecutionResultData {
                                                success: response.success,
                                                final_step: response.final_step,
                                                total_duration_us,
                                                steps,
                                            }));
                                        }
                                        Err(err) => {
                                            execution_result.set(Some(ExecutionResultData {
                                                success: false,
                                                final_step: "Network error".to_string(),
                                                total_duration_us: 0,
                                                steps: vec![("Error".to_string(), Some(err), 0)],
                                            }));
                                        }
                                    }
                                });
                            }
                        },
                        "Execute workflow"
                    }
                }

                section { class: "card",
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Execution trace" }
                            div { class: "card-description", "Every visited step is listed in order as the runtime reports it." }
                        }
                        if let Some(result) = execution_result.read().as_ref() {
                            div { class: "trace-header-stats",
                                if result.success {
                                    span { class: "badge badge-success", "pass" }
                                } else {
                                    span { class: "badge badge-danger", "fail" }
                                }
                                span { class: "telemetry-stat", "took {result.total_duration_us}µs" }
                            }
                        } else {
                            span { class: "badge badge-neutral", "waiting" }
                        }
                    }

                    if let Some(result) = execution_result.read().as_ref() {
                        div { class: "trace-summary",
                            span { class: "trace-summary-label", "Final step" }
                            span { class: "badge badge-neutral", "{result.final_step}" }
                        }
                        div { class: "trace-list",
                            for (index, (step_name, action, timestamp)) in result.steps.iter().enumerate() {
                                div { class: "trace-step",
                                    div { class: "trace-step-number", "{index + 1}" }
                                    div { class: "trace-step-body",
                                        div { class: "trace-step-main",
                                            div { class: "trace-step-name", "{step_name}" }
                                            span { class: "trace-step-time", "+{timestamp}µs" }
                                        }
                                        if let Some(action) = action {
                                            div { class: "trace-step-action", "{action}" }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "02" }
                            div { class: "empty-state-text", "Run a workflow to see the step trace and final state here." }
                        }
                    }
                }
            }
        }
    }
}

/// Local result data for display.
#[derive(Clone)]
struct ExecutionResultData {
    success: bool,
    final_step: String,
    total_duration_us: u64,
    steps: Vec<(String, Option<String>, u64)>,
}
