//! Documentation for RheLang & Rhexiom.

use dioxus::prelude::*;

const SYNC_CODE: &str = r#"workflow CustomerTriage v1.0.0 {
    input {
        customer_email: string,
        is_priority: boolean
    }

    start lookup_customer

    step lookup_customer {
        action hubspot_get_contact { email: customer_email };
        if is_priority { goto fast_track }
        else { goto standard_flow }
    }

    step fast_track {
        action salesforce_upsert_lead { 
            email: customer_email, 
            lastname: "Priority Lead" 
        };
        return "Notified Sales Force"
    }

    step standard_flow {
        return "Queued for Review"
    }
}"#;

/// RheLang syntax and engine documentation viewer.
#[component]
pub fn Documentation() -> Element {
    rsx! {
        div { class: "dashboard-stack control-docs",
            // Hero Section
            section { class: "industrial-card glass detail-hero",
                div { style: "display: flex; justify-content: space-between; align-items: flex-start;",
                    div {
                        div { class: "label-caps", style: "color: var(--accent);", "Language Specification" }
                        h2 { class: "app-title", style: "font-size: 28px; margin-top: 8px;", "RheLang: The Policy Operating System Language" }
                        p { class: "panel-copy", style: "margin-top: 12px; color: var(--text-secondary); max-width: 600px;", 
                            "RheLang is a deterministic DSL designed for modeling business policies as explicit, typed directed graphs. Every line is audit-ready, immutable, and forensic-grade." 
                        }
                    }
                    span { class: "status-pill", style: "background: var(--bg); color: var(--text-faint);", "v1.0.0-PRO" }
                }
            }

            // Core Concepts Grid
            section { class: "industrial-card",
                div { class: "label-caps", style: "margin-bottom: 24px;", "Core Infrastructure Concepts" }
                div { class: "grid-metrics",
                    div { class: "industrial-card glass",
                        div { class: "badge-neutral", style: "font-size: 10px; margin-bottom: 12px;", "Execution Mode" }
                        h4 { style: "margin: 0; color: var(--text-primary);", "Deterministic" }
                        p { style: "font-size: 13px; color: var(--text-secondary); margin-top: 8px;", "Workflows run in a sandboxed WASM environment, ensuring bit-identical behavior across all nodes." }
                    }
                    div { class: "industrial-card glass",
                        div { class: "badge-neutral", style: "font-size: 10px; margin-bottom: 12px;", "Type System" }
                        h4 { style: "margin: 0; color: var(--text-primary);", "Strict Context" }
                        p { style: "font-size: 13px; color: var(--text-secondary); margin-top: 8px;", "All inputs and intermediate states are strictly typed (String, Number, Boolean) to prevent drift." }
                    }
                    div { class: "industrial-card glass",
                        div { class: "badge-neutral", style: "font-size: 10px; margin-bottom: 12px;", "Modeling" }
                        h4 { style: "margin: 0; color: var(--text-primary);", "Graph-First" }
                        p { style: "font-size: 13px; color: var(--text-secondary); margin-top: 8px;", "Policies are modeled as explicit steps with immutable transitions, not sequential scripts." }
                    }
                }
            }

            div { class: "control-two-column", style: "display: grid; grid-template-columns: 2fr 1fr; gap: 24px;",
                // Grammar Reference
                section { class: "industrial-card",
                    div { class: "label-caps", style: "margin-bottom: 24px;", "Grammar & Syntax Ledger" }
                    
                    div { class: "type-table",
                        div { class: "type-row type-row-head",
                            div { "Instruction" }
                            div { "Operational Semantic" }
                        }
                        div { class: "type-row",
                            div { class: "type-cell-code", style: "color: var(--accent);", "workflow [Name] v[Version]" }
                            div { "Initializes the policy container with semantic versioning." }
                        }
                        div { class: "type-row",
                            div { class: "type-cell-code", style: "color: var(--accent);", "input {{ [field]: [Type] }}" }
                            div { "Declares the immutable data signature required for entry." }
                        }
                        div { class: "type-row",
                            div { class: "type-cell-code", style: "color: var(--accent);", "start [step_name]" }
                            div { "Identifies the root entry point for the graph traverser." }
                        }
                        div { class: "type-row",
                            div { class: "type-cell-code", style: "color: var(--accent);", "step [name] {{ ... }}" }
                            div { "Defines a logical node containing actions and transitions." }
                        }
                        div { class: "type-row",
                            div { class: "type-cell-code", style: "color: var(--accent);", "action [binding] {{ ... }}" }
                            div { "Executes an external service capability via standard registry." }
                        }
                    }
                }

                // Control Flow Cards
                div { style: "display: flex; flex-direction: column; gap: 24px;",
                    article { class: "industrial-card glass",
                        div { class: "label-caps", style: "font-size: 10px;", "Transition Control" }
                        h4 { style: "margin: 8px 0; color: var(--text-primary);", "Conditionals" }
                        code { style: "background: var(--bg); padding: 4px 8px; border-radius: 4px; font-size: 11px; color: var(--text-secondary);", "if [case] {{ goto A }} else {{ goto B }}" }
                        p { style: "font-size: 13px; color: var(--text-faint); margin-top: 12px;", "Implements deterministic branching using state or action outcomes." }
                    }
                    article { class: "industrial-card glass",
                        div { class: "label-caps", style: "font-size: 10px;", "Terminal Control" }
                        h4 { style: "margin: 8px 0; color: var(--text-primary);", "Returns" }
                        code { style: "background: var(--bg); padding: 4px 8px; border-radius: 4px; font-size: 11px; color: var(--text-secondary);", "return \"Status_String\"" }
                        p { style: "font-size: 13px; color: var(--text-faint); margin-top: 12px;", "Finalizes trace and transmits the final state to the orchestrator." }
                    }
                }
            }

            // Integrations Branded Grid
            section { class: "industrial-card",
                div { style: "margin-bottom: 32px;",
                    div { class: "label-caps", "External Service Registry" }
                    h3 { class: "app-title", style: "font-size: 20px; margin-top: 8px;", "Connected Capability Bindings" }
                    p { class: "panel-copy", style: "margin-top: 8px; font-size: 14px; color: var(--text-secondary);", "Standard actions enabled via integration pool synchronizers." }
                }

                div { class: "grid-metrics", style: "grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));",
                    div { class: "industrial-card glass",
                        div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 16px;",
                            div { class: "brand-mark", style: "background: rgba(255, 122, 0, 0.1); color: #FF7A00;", "◎" }
                            h4 { style: "margin: 0;", "HubSpot" }
                        }
                        ul { style: "list-style: none; padding: 0; margin: 0; font-size: 13px; color: var(--text-secondary); line-height: 1.8;",
                            li { code { class: "type-cell-code", "hubspot_get_contact" } }
                            li { code { class: "type-cell-code", "hubspot_create_deal" } }
                            li { code { class: "type-cell-code", "hubspot_update_contact" } }
                        }
                    }
                    div { class: "industrial-card glass",
                        div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 16px;",
                            div { class: "brand-mark", style: "background: rgba(0, 161, 224, 0.1); color: #00A1E0;", "☁" }
                            h4 { style: "margin: 0;", "Salesforce" }
                        }
                        ul { style: "list-style: none; padding: 0; margin: 0; font-size: 13px; color: var(--text-secondary); line-height: 1.8;",
                            li { code { class: "type-cell-code", "salesforce_query" } }
                            li { code { class: "type-cell-code", "salesforce_upsert_lead" } }
                            li { code { class: "type-cell-code", "salesforce_get_account" } }
                        }
                    }
                    div { class: "industrial-card glass",
                        div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 16px;",
                            div { class: "brand-mark", style: "background: rgba(0, 255, 157, 0.1); color: var(--accent);", "✉" }
                            h4 { style: "margin: 0;", "Email Gateway" }
                        }
                        ul { style: "list-style: none; padding: 0; margin: 0; font-size: 13px; color: var(--text-secondary); line-height: 1.8;",
                            li { code { class: "type-cell-code", "email_send_email" } }
                            li { code { class: "type-cell-code", "email_fetch_latest" } }
                        }
                    }
                }
            }

            // Interactive High-Fidelity Example
            section { class: "industrial-card",
                div { class: "label-caps", style: "margin-bottom: 24px;", "Policy Workbench Example" }
                div { 
                    style: "background: #0d1117; border-radius: 8px; padding: 32px; font-family: var(--font-mono); font-size: 13px; line-height: 1.6; border: 1px solid var(--panel-lighter); overflow-x: auto;",
                    pre { style: "margin: 0;", "{SYNC_CODE}" }
                }
            }
        }
    }
}
