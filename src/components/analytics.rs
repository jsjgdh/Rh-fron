use dioxus::prelude::*;
use crate::api;

#[component]
pub fn WorkflowAnalytics() -> Element {
    let analytics_res = use_resource(api::get_analytics);

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "ANALYTICS" }
            p { style: "font-size: 1.2rem; color: var(--text-secondary); margin-bottom: 48px;",
                "Performance telemetry and execution fidelity across the active policy fleet."
            }

            match analytics_res.read().as_ref() {
                Some(Ok(list)) => {
                    if list.is_empty() {
                        rsx! { 
                            div { class: "card", style: "padding: 48px; text-align: center; color: var(--text-faint);",
                                "No execution data available yet for analytics."
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "grid-3", style: "margin-bottom: 48px;",
                                StatCard { 
                                    label: "Avg Success Rate", 
                                    value: format!("{:.1}%", list.iter().map(|a| a.success_rate).sum::<f64>() / list.len() as f64 * 100.0),
                                    trend: "Stable"
                                }
                                StatCard { 
                                    label: "Avg Latency", 
                                    value: format!("{:.1}ms", list.iter().map(|a| a.avg_latency_ms).sum::<f64>() / list.len() as f64),
                                    trend: "Optimal"
                                }
                                StatCard { 
                                    label: "High Traffic Workflow", 
                                    value: list.first().map(|a| a.name.clone()).unwrap_or_default(),
                                    trend: "Active"
                                }
                            }

                            div { class: "section-title", "WORKFLOW PERFORMANCE" }
                            div { class: "card", style: "padding: 0; overflow: hidden;",
                                table { style: "width: 100%; border-collapse: collapse;",
                                    thead {
                                        tr { style: "background: var(--bg-card); border-bottom: 1px solid var(--border);",
                                            th { style: "padding: 16px 24px; text-align: left; font-size: 12px; font-weight: 700; color: var(--text-faint);", "WORKFLOW" }
                                            th { style: "padding: 16px 24px; text-align: left; font-size: 12px; font-weight: 700; color: var(--text-faint);", "EXECUTIONS" }
                                            th { style: "padding: 16px 24px; text-align: left; font-size: 12px; font-weight: 700; color: var(--text-faint);", "SUCCESS RATE" }
                                            th { style: "padding: 16px 24px; text-align: left; font-size: 12px; font-weight: 700; color: var(--text-faint);", "LATENCY" }
                                        }
                                    }
                                    tbody {
                                        for entry in list {
                                            tr { style: "border-bottom: 1px solid var(--border);",
                                                td { style: "padding: 20px 24px;",
                                                    div { style: "font-weight: 600; color: var(--text-primary);", "{entry.name}" }
                                                }
                                                td { style: "padding: 20px 24px;",
                                                    div { class: "mono", "{entry.execution_count}" }
                                                }
                                                td { style: "padding: 20px 24px;",
                                                    div { style: "display: flex; align-items: center; gap: 12px;",
                                                        div { 
                                                            style: "flex: 1; height: 6px; background: var(--border); border-radius: 3px; max-width: 100px; overflow: hidden;",
                                                            div { 
                                                                style: "height: 100%; background: var(--status-success); width: {entry.success_rate * 100.0}%"
                                                            }
                                                        }
                                                        span { style: "font-size: 13px; font-weight: 500;", "{entry.success_rate * 100.0:.1}%" }
                                                    }
                                                }
                                                td { style: "padding: 20px 24px;",
                                                    div { class: "mono", style: "color: var(--text-secondary);", "{entry.avg_latency_ms:.1}ms" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                _ => rsx! {
                    div { style: "padding: 80px; text-align: center;",
                        div { class: "spinner", style: "margin: 0 auto 16px;" }
                        div { style: "color: var(--text-faint);", "Aggregating telemetry..." }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: String, value: String, trend: String) -> Element {
    rsx! {
        div { class: "card stat-card",
            div { class: "stat-label", "{label}" }
            div { class: "stat-value", "{value}" }
            div { 
                style: "margin-top: 12px; font-size: 11px; font-weight: 700; color: var(--status-success); text-transform: uppercase; letter-spacing: 0.05em;",
                "● {trend}"
            }
        }
    }
}
