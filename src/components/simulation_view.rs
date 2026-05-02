use dioxus::prelude::*;
use crate::api::{SimulationResult, SimulationStep, DecisionPath, SimulatedAction, ExecutionTiming, SimulationMetadata};

/// Component to display detailed simulation results with real execution data.
#[component]
pub fn SimulationView(
    simulation: SimulationResult,
    on_close: EventHandler<()>,
) -> Element {
    let mut selected_tab = use_signal(|| Tab::Trace);
    let expanded_steps = use_signal::<Vec<usize>>(|| Vec::new());
    
    rsx! {
        div { class: "simulation-overlay",
            div { class: "simulation-modal",
                // Header
                div { class: "simulation-header",
                    div { class: "header-left",
                        h2 { "Simulation Results" }
                        span { class: "simulation-subtitle",
                            "{simulation.workflow_name} v{simulation.version}"
                        }
                    }
                    div { class: "header-right",
                        StatusBadge { status: simulation.status.clone() }
                        button { class: "close-btn", onclick: move |_| on_close.call(()), "✕" }
                    }
                }
                
// Tab Navigation
        div { class: "simulation-tabs",
        TabButton { label: "Execution Trace", tab: Tab::Trace, selected: *selected_tab.read(), on_select: move |t| selected_tab.set(t) }
        TabButton { label: "Decision Paths", tab: Tab::Decisions, selected: *selected_tab.read(), on_select: move |t| selected_tab.set(t) }
        TabButton { label: "Actions", tab: Tab::Actions, selected: *selected_tab.read(), on_select: move |t| selected_tab.set(t) }
        TabButton { label: "Timing & Memory", tab: Tab::Metrics, selected: *selected_tab.read(), on_select: move |t| selected_tab.set(t) }
        }
                
                // Tab Content
                div { class: "simulation-content",
                    match *selected_tab.read() {
                        Tab::Trace => rsx! { 
                            TraceTab { 
                                steps: simulation.trace.steps.clone(),
                                expanded_steps: expanded_steps
                            } 
                        },
                        Tab::Decisions => rsx! { 
                            DecisionsTab { paths: simulation.trace.decision_paths.clone() } 
                        },
                        Tab::Actions => rsx! { 
                            ActionsTab { actions: simulation.actions.clone() } 
                        },
                        Tab::Metrics => rsx! { 
                            MetricsTab { 
                                timing: simulation.timing.clone(),
                                metadata: simulation.metadata.clone()
                            } 
                        },
                    }
                }
                
                // Footer with summary
                div { class: "simulation-footer",
                    div { class: "footer-stats",
                        span { "Steps: {simulation.metadata.steps_executed}" }
                        span { "Branches: {simulation.metadata.branches_explored}" }
                        span { "Duration: {format_duration(simulation.timing.total_duration_us)}" }
                        span { "Memory: {format_bytes(simulation.timing.memory_usage_bytes)}" }
                    }
                    div { class: "footer-meta",
                        span { "Mode: {simulation.metadata.mode}" }
                        {
                            let status = if simulation.metadata.external_calls_enabled { "Enabled" } else { "Disabled" };
                            rsx! { span { "External calls: {status}" } }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Trace,
    Decisions,
    Actions,
    Metrics,
}

#[component]
fn TabButton(label: String, tab: Tab, selected: Tab, on_select: EventHandler<Tab>) -> Element {
    let is_selected = tab == selected;
    rsx! {
        button {
            class: if is_selected { "tab-btn active" } else { "tab-btn" },
            onclick: move |_| on_select.call(tab),
            "{label}"
        }
    }
}

#[component]
fn StatusBadge(status: String) -> Element {
    let status_class = match status.as_str() {
        "Completed" => "status-completed",
        "Failed" => "status-failed",
        "Suspended" => "status-suspended",
        "Running" => "status-running",
        _ => "status-pending",
    };
    rsx! {
        span { class: "status-badge {status_class}", "{status}" }
    }
}

#[component]
fn TraceTab(steps: Vec<SimulationStep>, expanded_steps: Signal<Vec<usize>>) -> Element {
    rsx! {
        div { class: "trace-container",
            if steps.is_empty() {
                div { class: "empty-state", "No execution trace available" }
            } else {
                div { class: "trace-timeline",
                    for (i, step) in steps.iter().enumerate() {
                        TraceStepView {
                            index: i,
                            step: step.clone(),
                            is_expanded: expanded_steps.read().contains(&i),
                            on_toggle: move |_| {
                                let mut expanded = expanded_steps.read().clone();
                                if expanded.contains(&i) {
                                    expanded.retain(|&x| x != i);
                                } else {
                                    expanded.push(i);
                                }
                                expanded_steps.set(expanded);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceStepView(
    index: usize,
    step: SimulationStep,
    is_expanded: bool,
    on_toggle: EventHandler<()>,
) -> Element {
    let action_text = step.action.clone().unwrap_or_else(|| "No action".to_string());
    let shadow_badge = if step.shadowed {
        rsx! { span { class: "shadow-badge", "Shadow" } }
    } else {
        rsx! { }
    };
    
    rsx! {
        div { class: "trace-step",
            div { class: "step-header", onclick: move |_| on_toggle.call(()),
                div { class: "step-number", "{index + 1}" }
                div { class: "step-info",
                    div { class: "step-name", "{step.step_name}" }
                    div { class: "step-action", "{action_text}" }
                }
                div { class: "step-meta",
                    span { class: "step-duration", "{format_duration(step.duration_us)}" }
                    {shadow_badge}
                    span { class: "expand-icon", if is_expanded { "▼" } else { "▶" } }
                }
            }
            if is_expanded {
                div { class: "step-details",
                    h4 { "Variables at this step:" }
                    div { class: "variables-grid",
                        for (name, value) in step.variables.iter() {
                            VariableRow { name: name.clone(), value: value.clone() }
                        }
                    }
                    div { class: "step-timing",
                        span { "Timestamp: {step.timestamp_us}µs" }
                    }
                }
            }
        }
    }
}

#[component]
fn VariableRow(name: String, value: serde_json::Value) -> Element {
    let formatted_value = match &value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        _ => value.to_string(),
    };
    
    let value_class = match &value {
        serde_json::Value::String(_) => "value-string",
        serde_json::Value::Number(_) => "value-number",
        serde_json::Value::Bool(_) => "value-bool",
        _ => "value-other",
    };
    
    rsx! {
        div { class: "variable-row",
            span { class: "var-name", "{name}:" }
            span { class: "var-value {value_class}", "{formatted_value}" }
        }
    }
}

#[component]
fn DecisionsTab(paths: Vec<DecisionPath>) -> Element {
    rsx! {
        div { class: "decisions-container",
            if paths.is_empty() {
                div { class: "empty-state", 
                    "No decision branches in this workflow"
                }
            } else {
                for path in paths.iter() {
                    div { class: "decision-card",
                        div { class: "decision-header",
                            span { class: "decision-step", "{path.step_name}" }
                        }
                        if let Some(condition) = &path.condition {
                            div { class: "decision-condition",
                                span { class: "label", "Condition: " }
                                code { "{condition}" }
                            }
                        }
                        div { class: "decision-branch",
                            span { class: "label", "Branch taken: " }
                            span { class: "branch-taken", "{path.branch_taken}" }
                        }
                        if !path.alternative_branches.is_empty() {
                            div { class: "alternative-branches",
                                span { class: "label", "Alternative branches: " }
                                for alt in path.alternative_branches.iter() {
                                    span { class: "alt-branch", "{alt}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ActionsTab(actions: Vec<SimulatedAction>) -> Element {
    rsx! {
        div { class: "actions-container",
            if actions.is_empty() {
                div { class: "empty-state", "No actions would be executed" }
            } else {
                div { class: "actions-list",
                    for action in actions.iter() {
                        div { class: "action-card",
                            div { class: "action-header",
                                div { class: "action-name", "{action.name}" }
                                div { class: "action-meta",
                                    span { class: "action-step", "Step: {action.step_name}" }
                                    if action.executed {
                                        span { class: "executed-badge", "Executed" }
                                    } else {
                                        span { class: "simulated-badge", "Simulated" }
                                    }
                                }
                            }
                            if let Some(result) = &action.result {
                                div { class: "action-result",
                                    h4 { "Result:" }
                                    pre { "{result}" }
                                }
                            }
                            if let Some(error) = &action.error {
                                div { class: "action-error",
                                    span { "Error: {error}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MetricsTab(timing: ExecutionTiming, metadata: SimulationMetadata) -> Element {
    rsx! {
        div { class: "metrics-container",
            div { class: "metrics-grid",
                div { class: "metric-card",
                    h3 { "Timing" }
                    div { class: "metric-row",
                        span { "Total duration:" }
                        span { class: "metric-value", "{format_duration(timing.total_duration_us)}" }
                    }
                    div { class: "metric-row",
                        span { "Execution time:" }
                        span { class: "metric-value", "{format_duration(timing.execution_time_us)}" }
                    }
                    div { class: "metric-row",
                        span { "External calls:" }
                        span { class: "metric-value", "{format_duration(timing.external_call_time_us)}" }
                    }
                }
                
                div { class: "metric-card",
                    h3 { "Memory" }
                    div { class: "metric-row large",
                        span { "Estimated usage:" }
                        span { class: "metric-value highlight", "{format_bytes(timing.memory_usage_bytes)}" }
                    }
                }
                
                div { class: "metric-card wide",
                    h3 { "Step Timings" }
                    if timing.step_timings.is_empty() {
                        span { class: "empty-text", "No step timings available" }
                    } else {
                        div { class: "step-timings-list",
                            for (step_name, duration) in timing.step_timings.iter() {
                                div { class: "step-timing-row",
                                    span { class: "step-name", "{step_name}" }
                                    span { class: "step-duration", "{format_duration(*duration)}" }
                                }
                            }
                        }
                    }
                }
                
                div { class: "metric-card wide",
                    h3 { "Simulation Metadata" }
                    div { class: "metadata-grid",
                        div { class: "meta-item",
                            span { class: "meta-label", "Mode:" }
                            span { class: "meta-value", "{metadata.mode}" }
                        }
                        div { class: "meta-item",
                            span { class: "meta-label", "External calls:" }
                            {
                                let status = if metadata.external_calls_enabled { "Enabled" } else { "Disabled" };
                                rsx! { span { class: "meta-value", "{status}" } }
                            }
                        }
                        div { class: "meta-item",
                            span { class: "meta-label", "Steps executed:" }
                            span { class: "meta-value", "{metadata.steps_executed}" }
                        }
                        div { class: "meta-item",
                            span { class: "meta-label", "Branches explored:" }
                            span { class: "meta-value", "{metadata.branches_explored}" }
                        }
                        div { class: "meta-item full",
                            span { class: "meta-label", "Started at:" }
                            span { class: "meta-value", "{metadata.started_at}" }
                        }
                    }
                }
            }
        }
    }
}

fn format_duration(micros: u64) -> String {
    if micros < 1000 {
        format!("{}µs", micros)
    } else if micros < 1_000_000 {
        format!("{:.2}ms", micros as f64 / 1000.0)
    } else {
        format!("{:.2}s", micros as f64 / 1_000_000.0)
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
