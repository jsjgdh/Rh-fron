use dioxus::prelude::*;

#[component]
pub fn Documentation() -> Element {
    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "DOCUMENTATION" }
            p { style: "font-size: 1.2rem; color: var(--text-secondary); margin-bottom: 48px; max-width: 800px;",
                "Technical specifications for integrating and extending the Rhexiom Policy Operating System."
            }

            div { class: "section-title", "GETTING STARTED" }
            div { class: "card",
                h3 { style: "margin-bottom: 16px; font-size: 1.5rem;", "RHELANG BASICS" }
                p { style: "line-height: 1.6;", "RheLang is a deterministic domain-specific language for modeling business logic as directed acyclic graphs. Every policy in Rhexiom is compiled from RheLang into a sandboxed WASM artifact." }
                
                h4 { style: "margin-top: 24px; margin-bottom: 12px; font-size: 1.1rem;", "Core Syntax" }
                pre { 
                    class: "mono",
                    style: "margin-top: 12px; padding: 20px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "policy \"TaxRate\" {{\n  input income: decimal\n  step calc = income * 0.15\n  output result = calc\n}}"
                }
                
                h4 { style: "margin-top: 24px; margin-bottom: 12px; font-size: 1.1rem;", "Key Concepts" }
                ul { style: "margin-left: 20px; line-height: 1.8;",
                    li { "policy - Defines a named policy module with inputs and outputs" }
                    li { "input - Declares external data that flows into the workflow" }
                    li { "step - Defines a computation node in the execution graph" }
                    li { "output - Marks the final result of policy evaluation" }
                    li { "action - Triggers side effects like notifications or API calls" }
                }
            }

            div { class: "section-title", "STEP-BY-STEP WORKFLOW CREATION" }
            div { class: "card",
                ol { style: "margin-left: 20px; line-height: 2;",
                    li { "Navigate to Studio from the sidebar" }
                    li { "Enter your policy in natural language or RheLang syntax" }
                    li { "Click 'Start Compilation' to generate the WASM artifact" }
                    li { "Review the generated workflow graph and node connections" }
                    li { "Test with sample inputs via the Run panel" }
                    li { "Deploy to production when validation passes" }
                }
            }

            div { class: "section-title", "API ENDPOINTS" }
            div { class: "card",
                table { style: "width: 100%; border-collapse: collapse;",
                    thead {
                        tr { style: "border-bottom: 1px solid var(--border);",
                            th { style: "text-align: left; padding: 12px; font-size: 12px;", "METHOD" }
                            th { style: "text-align: left; padding: 12px; font-size: 12px;", "ENDPOINT" }
                            th { style: "text-align: left; padding: 12px; font-size: 12px;", "DESCRIPTION" }
                        }
                    }
                    tbody {
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "POST" }
                            td { class: "mono", style: "padding: 10px;", "/api/workflows/compile" }
                            td { style: "padding: 10px; font-size: 14px;", "Compile RheLang source to WASM" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "POST" }
                            td { class: "mono", style: "padding: 10px;", "/api/workflows/run" }
                            td { style: "padding: 10px; font-size: 14px;", "Execute a compiled workflow" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "GET" }
                            td { class: "mono", style: "padding: 10px;", "/api/workflows" }
                            td { style: "padding: 10px; font-size: 14px;", "List all deployed workflows" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "GET" }
                            td { class: "mono", style: "padding: 10px;", "/api/executions/{{id}}" }
                            td { style: "padding: 10px; font-size: 14px;", "Retrieve execution details and trace" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "POST" }
                            td { class: "mono", style: "padding: 10px;", "/api/executions/{{id}}/resume" }
                            td { style: "padding: 10px; font-size: 14px;", "Resume a suspended execution" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "POST" }
                            td { class: "mono", style: "padding: 10px;", "/api/webhooks" }
                            td { style: "padding: 10px; font-size: 14px;", "Create a new webhook endpoint" }
                        }
                    }
                }
            }

            div { class: "section-title", "COMMON PATTERNS" }
            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                div { class: "card",
                    h3 { style: "margin-bottom: 12px; font-size: 1.25rem;", "Conditional Branching" }
                    pre { 
                        class: "mono",
                        style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 12px;",
                        "step check = income > threshold\nif check:\n  action approve()\nelse:\n  action reject()"
                    }
                }
                div { class: "card",
                    h3 { style: "margin-bottom: 12px; font-size: 1.25rem;", "External Data Fetch" }
                    pre { 
                        class: "mono",
                        style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 12px;",
                        "input user_id: string\nstep profile = fetch(\n  \"https://api.example.com/users/{{user_id}}\"\n)"
                    }
                }
            }

            div { class: "section-title", "INTEGRATION" }
            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                DocCard { 
                    title: "REST API".to_string(), 
                    desc: "Trigger policies and retrieve forensic traces via standard JSON/HTTPS.".to_string(),
                    tag: "v1.0 stable".to_string()
                }
                DocCard { 
                    title: "WEBHOOKS".to_string(), 
                    desc: "Async notifications for policy state transitions and step completions.".to_string(),
                    tag: "v1.0 stable".to_string()
                }
                DocCard { 
                    title: "WASM SDK".to_string(), 
                    desc: "Directly embed Rhexiom's deterministic engine into your own rust projects.".to_string(),
                    tag: "BETA".to_string()
                }
                DocCard { 
                    title: "GRPC CORE".to_string(), 
                    desc: "High-performance policy orchestration for microservices.".to_string(),
                    tag: "v1.1 nightly".to_string()
                }
            }
        }
    }
}

#[component]
fn DocCard(title: String, desc: String, tag: String) -> Element {
    rsx! {
        div { class: "card",
            div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 12px;",
                h3 { style: "font-size: 1.5rem; margin: 0;", "{title}" }
                span { class: "status-pill", "{tag}" }
            }
            p { style: "font-size: 14px; color: var(--text-secondary); line-height: 1.6;", "{desc}" }
        }
    }
}
