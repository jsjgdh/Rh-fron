use dioxus::prelude::*;
use tracing;
use serde_json::Value;
use crate::api::{get_execution_detail, get_workflow_detail, simulate_execution, SimulationResult};
use crate::components::workflow_graph::WorkflowGraph;
use crate::components::simulation_view::SimulationView;

#[component]
pub fn ExecutionDetail(id: String) -> Element {
    let execution = use_resource({
        let id = id.clone();
        move || {
            let id = id.clone();
            async move { get_execution_detail(&id).await }
        }
    });

    let workflow = use_resource(move || {
        let exec_data = execution.read().as_ref().and_then(|r| r.as_ref().ok().cloned());
        async move {
            if let Some(data) = exec_data {
                let name = data["workflow_name"].as_str().unwrap_or_default();
                let version = data["version"].as_str().unwrap_or_default();
                get_workflow_detail(name, version).await
            } else {
                Err("Loading execution...".to_string())
            }
        }
    });
    
    let mut simulation = use_signal(|| None::<SimulationResult>);
    let mut simulation_loading = use_signal(|| false);

    let exec_state = execution.read();
    let wf_state = workflow.read();

    if let Some(Ok(data)) = &*exec_state {
        // Handle trace as either an object with steps or a direct array
        let trace_obj = data["trace"].as_object();
        let steps = trace_obj
            .and_then(|t| t.get("steps"))
            .and_then(|s| s.as_array())
            .or_else(|| data["trace"].as_array());
        let active_steps: Vec<String> = steps
            .clone()
            .map(|s| {
                s.iter()
                    .filter_map(|step| step["step_name"].as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let total_duration_us = trace_obj
            .and_then(|t| t.get("total_duration_us"))
            .and_then(|d| d.as_u64())
            .unwrap_or(0);

        let status = data["status"].as_str().unwrap_or_default();
        let status_lower = status.to_lowercase();
        let created_at = data["created_at"].as_str().unwrap_or_default();
        let wf_name = data["workflow_name"].as_str().unwrap_or("Unknown");

        rsx! {
            div { class: "dashboard-stack",
                // ── Header Summary ───────────────────────────────────────
                div { class: "card", style: "border-color: var(--accent-soft);",
                    div { class: "card-header",
                        div {
                            div { class: "app-eyebrow", "Forensic Audit Trail" }
                            h2 { class: "section-title", "Trace {id}" }
                            p { class: "section-copy", "Verified execution of {wf_name} recorded on {created_at}." }
                        }
                        div { style: "display: flex; align-items: center; gap: 12px;",
                            button { 
                                class: "btn btn-secondary", 
                                style: "background: var(--bg); border: 1px solid var(--accent); color: var(--accent);",
                                disabled: *simulation_loading.read(),
                                onclick: move |_| {
                                    let id = id.clone();
                                    simulation_loading.set(true);
                                    spawn(async move {
                                        match simulate_execution(&id).await {
                                            Ok(result) => {
                                                simulation.set(Some(result));
                                            }
                                            Err(e) => {
                                                // Error will be logged, UI will show empty state
                                                tracing::error!("Simulation failed: {}", e);
                                            }
                                        }
                                        simulation_loading.set(false);
                                    });
                                },
                                if *simulation_loading.read() {
                                    "Running simulation..."
                                } else {
                                    "Simulate what-if branch"
                                }
                            }
                            span { class: "badge badge-{status_lower}", "{status}" }
                        }
                    }
                }

                div { class: "grid-2",
                    style: "grid-template-columns: 1fr 340px;",
                    
                    // ── Visual Verification ────────────────────────────────
                    section { class: "card",
                        div { class: "card-header",
                            div {
                                div { class: "card-title", "Visual Path Verification" }
                                div { class: "card-description", "Highlighted spine showing the exact route taken during execution." }
                            }
                        }
                        div { class: "vg-canvas",
                            if let Some(Ok(wf_data)) = &*wf_state {
                                {
                                    let ast = wf_data.get("ast_json")
                                        .and_then(|s| s.as_str())
                                        .and_then(|s| serde_json::from_str::<Value>(s).ok())
                                        .unwrap_or(Value::Null);
                                    rsx! {
                                        WorkflowGraph { 
                                            injected_ast: ast,
                                            active_steps: Some(active_steps.clone())
                                        }
                                    }
                                }
                            } else {
                                div { class: "empty-state", "Loading graph verification..." }
                            }
                        }
                    }

                    // ── Audit Log ──────────────────────────────────────────
                    aside { class: "card",
                        style: "background-color: #08080A;",
                        div { class: "card-header",
                            div {
                                div { class: "card-title", "Step-by-Step Audit" }
                                div { class: "card-description", "Deterministic decision log. Total duration: {total_duration_us}µs" }
                            }
                        }
                        div { class: "pipeline-list",
                            if let Some(step_list) = steps {
                                for (i, step) in step_list.iter().enumerate() {
                                    {
                                        let idx = i + 1;
                                        let step_name = step["step_name"].as_str().unwrap_or("?");
                                        let duration = step["timestamp_us"].as_u64().unwrap_or(0);
                                        rsx! {
                                            div { class: "pipeline-step",
                                                div { class: "pipeline-index", "{idx}" }
                                                div {
                                                    div { class: "pipeline-title", "{step_name}" }
                                                    div { class: "stat-note", "Latency: {duration}µs" }
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
            
            // Simulation overlay
            if let Some(sim) = simulation.read().as_ref() {
                SimulationView {
                    simulation: sim.clone(),
                    on_close: move |_| simulation.set(None)
                }
            }
        }
    } else {
        rsx! {
            div { class: "empty-state",
                div { class: "spinner spinner-lg", style: "margin-bottom: 24px;" }
                div { "Retrieving audit data from ledger..." }
            }
        }
    }
}
