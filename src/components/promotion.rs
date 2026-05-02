use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct EnvStatus {
    pub workflow: String,
    pub dev: String,
    pub staging: String,
    pub production: String,
}

#[component]
pub fn Promotion() -> Element {
    let workflows = use_signal(|| vec![
        EnvStatus { workflow: "ExpenseGate".to_string(), dev: "v2.4.1".to_string(), staging: "v2.3.0".to_string(), production: "v2.1.0".to_string() },
        EnvStatus { workflow: "DataPurge".to_string(), dev: "v1.2.0".to_string(), staging: "v1.2.0".to_string(), production: "v1.1.5".to_string() },
        EnvStatus { workflow: "AccessControl".to_string(), dev: "v3.0.2".to_string(), staging: "v2.9.0".to_string(), production: "v2.8.4".to_string() },
    ]);

    let mut selected_wf = use_signal(|| Option::<String>::None);
    let mut show_modal = use_signal(|| false);

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "ENVIRONMENT PROMOTION" }
            p { style: "font-size: 1.1rem; color: var(--text-secondary); margin-bottom: 40px;",
                "Orchestrate policy rollout across isolated environments with cryptographic verification."
            }

            div { class: "industrial-card",
                div { class: "type-table",
                    div { class: "type-row type-row-head",
                        style: "grid-template-columns: 2fr 1fr 1fr 1fr 1fr;",
                        div { "Policy Workflow" }
                        div { "Development" }
                        div { "Staging" }
                        div { "Production" }
                        div { style: "text-align: right;", "Actions" }
                    }

                    for wf in workflows.read().iter() {
                        div { class: "type-row",
                            style: "grid-template-columns: 2fr 1fr 1fr 1fr 1fr;",
                            div { 
                                div { style: "font-weight: 700; font-size: 15px;", "{wf.workflow}" }
                                div { style: "font-size: 11px; color: var(--text-faint); margin-top: 4px;", "Checksum: 0x82f...a1e" }
                            }
                            div { 
                                span { class: "status-pill", style: "background: var(--bg); border: 1px solid var(--border-strong);", "{wf.dev}" }
                            }
                            div { 
                                span { class: "status-pill", style: "background: var(--bg); border: 1px solid var(--accent-primary); color: var(--accent-primary);", "{wf.staging}" }
                            }
                            div { 
                                span { class: "status-pill status-pill-success", "{wf.production}" }
                            }
                            div { style: "text-align: right;",
                                button { 
                                    class: "btn btn-primary",
                                    style: "padding: 6px 16px; font-size: 12px;",
                                    onclick: {
                                        let name = wf.workflow.clone();
                                        move |_| {
                                            selected_wf.set(Some(name.clone()));
                                            show_modal.set(true);
                                        }
                                    },
                                    "Promote"
                                }
                            }
                        }
                    }
                }
            }

            if *show_modal.read() {
                div { 
                    class: "modal-overlay fade-in",
                    style: "z-index: 100;",
                    div { 
                        class: "industrial-card",
                        style: "width: 500px; padding: 40px;",
                        h2 { style: "font-size: 2rem; margin-bottom: 8px;", "PROMOTE ARTIFACT" }
                        p { style: "color: var(--text-secondary); font-size: 14px; margin-bottom: 32px;", 
                            "Moving {selected_wf.read().clone().unwrap_or_default()} to the next stage." 
                        }

                        div { style: "display: grid; grid-template-columns: 1fr auto 1fr; align-items: center; gap: 24px; margin-bottom: 40px; padding: 24px; background: var(--bg); border-radius: 12px; border: 1px solid var(--border);",
                            div { style: "text-align: center;",
                                div { class: "label-caps", style: "font-size: 10px; margin-bottom: 8px;", "Source" }
                                div { style: "font-weight: 700; color: var(--accent-primary);", "Staging" }
                                div { style: "font-size: 12px; color: var(--text-faint);", "v2.3.0" }
                            }
                            div { style: "font-size: 24px; color: var(--text-faint);", "→" }
                            div { style: "text-align: center;",
                                div { class: "label-caps", style: "font-size: 10px; margin-bottom: 8px;", "Target" }
                                div { style: "font-weight: 700; color: var(--status-success);", "Production" }
                                div { style: "font-size: 12px; color: var(--text-faint);", "v2.3.0" }
                            }
                        }

                        div { class: "form-group",
                            label { class: "form-label", "Change Justification" }
                            textarea { 
                                class: "form-input", 
                                style: "min-height: 100px; resize: none;",
                                placeholder: "Describe the reason for this promotion..."
                            }
                        }

                        div { style: "margin-top: 32px; display: flex; gap: 12px;",
                            button { 
                                class: "btn btn-primary", 
                                style: "flex: 1;",
                                onclick: move |_| show_modal.set(false),
                                "Request Promotion" 
                            }
                            button { 
                                class: "btn btn-ghost", 
                                style: "flex: 1;",
                                onclick: move |_| show_modal.set(false),
                                "Cancel" 
                            }
                        }
                    }
                }
            }
        }
    }
}
