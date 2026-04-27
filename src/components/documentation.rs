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
                p { "RheLang is a deterministic domain-specific language for modeling business logic as directed acyclic graphs. Every policy in Rhexiom is compiled from RheLang into a sandboxed WASM artifact." }
                pre { 
                    class: "mono",
                    style: "margin-top: 24px; padding: 20px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "policy \"TaxRate\" {{\n  input income: decimal\n  step calc = income * 0.15\n  output result = calc\n}}"
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
