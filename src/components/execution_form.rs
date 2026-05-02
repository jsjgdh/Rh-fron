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
        use_resource(|| async move { crate::api::list_workflows(None).await.unwrap_or_default() });

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
        div { class: "dashboard-stack control-execution",
            section { class: "industrial-card glass detail-hero",
                div { style: "display: flex; justify-content: space-between; align-items: center;",
                    div {
                        div { class: "label-caps", style: "color: var(--accent);", "Execution console" }
                        h2 { class: "app-title", style: "font-size: 24px; margin-top: 8px;", "Run a stored workflow version with typed inputs." }
                        p { class: "panel-copy", style: "margin-top: 12px; color: var(--text-secondary);", "Select a workflow deployment, provide schema-aware values, and inspect the exact trace returned by the runtime." }
                    }
                    span { class: "status-pill", "runtime" }
                }
            }

            div { class: "grid-metrics", style: "grid-template-columns: 1fr 1fr;",
                section { class: "industrial-card" ,
                    div { class: "label-caps", "Input Deployment" }
                    
                    div { class: "form-group", style: "margin-top: 24px;",
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
                                    div { class: "dynamic-form", style: "margin-top: 32px;",
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
                                                                span { style: "color: var(--text-faint); font-weight: 500; margin-left: 8px;", "boolean" }
                                                            }
                                                            div { style: "display: flex; gap: 8px;",
                                                                button {
                                                                    class: if current_value == "true" {
                                                                        "btn btn-primary"
                                                                    } else {
                                                                        "btn btn-secondary"
                                                                    },
                                                                    style: "flex: 1;",
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
                                                                        "btn btn-primary"
                                                                    } else {
                                                                        "btn btn-secondary"
                                                                    },
                                                                    style: "flex: 1;",
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
                                                                span { style: "color: var(--text-faint); font-weight: 500; margin-left: 8px;", "{field_type.to_lowercase()}" }
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
                        div { style: "margin-top: 48px; text-align: center; color: var(--text-faint);",
                            div { class: "label-caps", style: "font-size: 24px; opacity: 0.1;", "01" }
                            p { style: "font-size: 14px; margin-top: 12px;", "Pick a workflow version to generate the input form." }
                        }
                    }

                    div { style: "border-top: 1px solid var(--border); margin: 32px 0;" }

                    button {
                        class: "btn btn-primary",
                        style: "width: 100%;",
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
                                            let total_duration_us = response.trace.total_duration_us;
                                            let steps = response
                                                .trace
                                                .steps
                                                .into_iter()
                                                .map(|step| (step.step_name, step.action, step.timestamp_us))
                                                .collect();

                                            execution_result.set(Some(ExecutionResultData {
                                                success: response.success,
                                                final_step: response.final_step,
                                                total_duration_us,
                                                steps,
                                                actions: response.actions,
                                            }));
                                        }
                                        Err(err) => {
                                            execution_result.set(Some(ExecutionResultData {
                                                success: false,
                                                final_step: "Network error".to_string(),
                                                total_duration_us: 0,
                                                steps: vec![("Error".to_string(), Some(err), 0)],
                                                actions: Vec::new(),
                                            }));
                                        }
                                    }
                                });
                            }
                        },
                        "Execute workflow"
                    }
                }

                section { class: "industrial-card",
                    div { style: "display: flex; justify-content: space-between; align-items: flex-start;",
                        div {
                            div { class: "label-caps", "Execution trace" }
                            p { class: "panel-copy", style: "font-size: 13px; color: var(--text-secondary);", "Every visited step is listed in order as the runtime reports it." }
                        }
                        if let Some(result) = execution_result.read().as_ref() {
                            div { style: "display: flex; gap: 8px; align-items: center;",
                                if result.success {
                                    span { class: "status-pill status-pill-success", "pass" }
                                } else {
                                    span { class: "status-pill status-pill-danger", "fail" }
                                }
                                span { style: "font-size: 11px; color: var(--text-faint); font-weight: 600;", "{result.total_duration_us}µs" }
                            }
                        } else {
                            span { class: "status-pill", "waiting" }
                        }
                    }

                    if let Some(result) = execution_result.read().as_ref() {
                        div { 
                            class: "industrial-card glass",
                            style: "margin-top: 32px; padding: 12px 16px; display: flex; justify-content: space-between; align-items: center;",
                            span { class: "label-caps", style: "margin: 0; font-size: 10px;", "Final decision" }
                            span { class: "brand-name", style: "color: var(--text-primary); font-size: 13px;", "{result.final_step}" }
                        }

                        // Actions display
                        if !result.actions.is_empty() {
                            div { 
                                class: "industrial-card",
                                style: "margin-top: 16px; padding: 12px 16px;",
                                div { class: "label-caps", style: "margin-bottom: 8px; font-size: 10px;", "Actions Taken" }
                                div { style: "display: flex; flex-direction: column; gap: 4px;",
                                    for action in result.actions.iter() {
                                        div { style: "font-size: 12px; color: var(--text-secondary); font-family: monospace;", "{action}" }
                                    }
                                }
                            }
                        }
                        
                        div { class: "trace-timeline", style: "margin-top: 32px; display: flex; flex-direction: column; gap: 12px;",
                            for (index, (step_name, action, timestamp)) in result.steps.iter().enumerate() {
                                div { 
                                    style: "display: flex; gap: 16px; padding: 12px; border-bottom: 1px solid var(--border); align-items: center;",
                                    div { class: "nav-icon", style: "font-size: 10px; flex-shrink: 0;", "{index + 1}" }
                                    div { style: "flex: 1;",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { style: "font-weight: 600; font-size: 14px;", "{step_name}" }
                                            span { style: "color: var(--text-faint); font-size: 11px;", "+{timestamp}µs" }
                                        }
                                        if let Some(action) = action {
                                            div { style: "font-size: 12px; color: var(--text-secondary); margin-top: 2px;", "{action}" }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { style: "margin-top: 80px; text-align: center; color: var(--text-faint);",
                            div { class: "label-caps", style: "font-size: 24px; opacity: 0.1;", "02" }
                            p { style: "font-size: 14px; margin-top: 12px;", "Run a workflow to see the step trace and final state here." }
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
    actions: Vec<String>,
}
