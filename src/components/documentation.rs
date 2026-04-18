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
        div { class: "page-stack",
            section { class: "card detail-hero",
                div { class: "detail-hero-head",
                    div {
                        div { class: "section-kicker", "Language guide" }
                        h2 { class: "section-title", "RheLang: The Policy Operating System Language." }
                        p { class: "section-copy", "RheLang is a deterministic DSL designed for modeling business policies as explicit, typed directed graphs. Every line is audit-ready and traceable." }
                    }
                    span { class: "badge badge-neutral", "v1.0 spec" }
                }
            }

            div { class: "grid-2 docs-layout",
                section { class: "card",
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Core Concepts" }
                            div { class: "card-description", "The mental model for RheLang execution." }
                        }
                    }
                    ul { class: "bullet-list",
                        li {
                            strong { "Deterministic Execution: " }
                            "Workflows run in a sandboxed WASM environment, ensuring identical behavior across all environments."
                        }
                        li {
                            strong { "Typed Context: " }
                            "All inputs and intermediate states are strictly typed (String, Number, Boolean)."
                        }
                        li {
                            strong { "Graph-First Design: " }
                            "Policies are not scripts; they are collections of named steps with explicit transitions."
                        }
                    }
                }

                section { class: "card",
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Grammar & Basic Syntax" }
                            div { class: "card-description", "The building blocks of a RheLang file." }
                        }
                    }
                    div { class: "syntax-ref",
                        div { class: "syntax-item",
                            code { "workflow [Name] v[Version] {{ ... }}" }
                            p { "Defines the entry point and semantic version of the policy." }
                        }
                        div { class: "syntax-item",
                            code { "input {{ [field]: [Type] }}" }
                            p { "Declares the required data schema to initiate an execution." }
                        }
                        div { class: "syntax-item",
                            code { "start [step_name]" }
                            p { "Specifies which step should be executed first." }
                        }
                    }
                }
            }

            section { class: "card",
                div { class: "card-header",
                    div {
                        div { class: "card-title", "Control Flow: Steps & Branching" }
                        div { class: "card-description", "Mapping out the decision logic." }
                    }
                }
                div { class: "grid-3",
                    div { class: "flow-card",
                        h4 { "Steps" }
                        p { "Logical containers for actions and transitions. Each step is a node in the graph." }
                        pre { class: "code-block-tiny", "step my_step {{ ... }}" }
                    }
                    div { class: "flow-card",
                        h4 { "Conditionals" }
                        p { "Standard if/else branching using input variables or action results." }
                        pre { class: "code-block-tiny", "if variable {{ goto a }} else {{ goto b }}" }
                    }
                    div { class: "flow-card",
                        h4 { "Returns" }
                        p { "Finalizes execution and returns a string status to the orchestrator." }
                        pre { class: "code-block-tiny", "return \"Completed\"" }
                    }
                }
            }

            section { class: "card",
                div { class: "card-header",
                    div {
                        div { class: "card-title", "Integrations Catalog" }
                        div { class: "card-description", "Standard actions available for connected services." }
                    }
                }
                div { class: "type-table",
                    div { class: "type-row type-row-3 type-row-head",
                        div { "Service" }
                        div { "Action Binding" }
                        div { "Arguments Schema" }
                    }
                    div { class: "type-row type-row-3",
                        div { "HubSpot" }
                        div { class: "type-cell-code", "hubspot_get_contact" }
                        div { class: "type-cell-code", "{{ email: String }}" }
                    }
                    div { class: "type-row type-row-3",
                        div { "HubSpot" }
                        div { class: "type-cell-code", "hubspot_create_deal" }
                        div { class: "type-cell-code", "{{ amount: Number, name: String }}" }
                    }
                    div { class: "type-row type-row-3",
                        div { "HubSpot" }
                        div { class: "type-cell-code", "hubspot_update_contact" }
                        div { class: "type-cell-code", "{{ email: String, property: String, value: String }}" }
                    }
                    div { class: "type-row type-row-3",
                        div { "Salesforce" }
                        div { class: "type-cell-code", "salesforce_query" }
                        div { class: "type-cell-code", "{{ query: String }}" }
                    }
                    div { class: "type-row type-row-3",
                        div { "Salesforce" }
                        div { class: "type-cell-code", "salesforce_upsert_lead" }
                        div { class: "type-cell-code", "{{ email: String, lastname: String }}" }
                    }
                    div { class: "type-row type-row-3",
                        div { "Salesforce" }
                        div { class: "type-cell-code", "salesforce_get_account" }
                        div { class: "type-cell-code", "{{ account_id: String }}" }
                    }
                }
            }

            section { class: "card",
                div { class: "card-header",
                    div {
                        div { class: "card-title", "Interactive Example" }
                        div { class: "card-description", "A complete triage flow illustrating inputs, actions, and branching." }
                    }
                }
                pre { class: "code-block", "{SYNC_CODE}" }
            }
        }
    }
}
