use dioxus::prelude::*;
use crate::components::workflow_graph::WorkflowGraph;
use crate::api::{WorkflowSummary, get_workflow_detail};

#[component]
pub fn Visualize() -> Element {
    let mut workflows = use_signal(Vec::<WorkflowSummary>::new);
    let mut selected_workflow = use_signal(|| Option::<String>::None);
    let mut selected_version = use_signal(|| Option::<String>::None);
    let mut workflow_detail = use_signal(|| Option::<serde_json::Value>::None);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    // Initial fetch of workflows
    let _ = use_resource(move || async move {
        if let Ok(list) = crate::api::list_workflows(None).await {
            workflows.set(list);
        }
    });

    // Fetch details when selection changes
    let _ = use_resource(move || {
        let name = selected_workflow.read().clone();
        let version = selected_version.read().clone();
        async move {
            if let (Some(n), Some(v)) = (name, version) {
                loading.set(true);
                match get_workflow_detail(&n, &v).await {
                    Ok(detail) => {
                        workflow_detail.set(Some(detail));
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        workflow_detail.set(None);
                    }
                }
                loading.set(false);
            }
        }
    });

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "DIAGNOSTIC VISUALIZER" }
            p { style: "font-size: 1.1rem; color: var(--text-secondary); margin-bottom: 40px;",
                "Interactive trace of policy logic compiled into deterministic execution graphs."
            }

            section { class: "card", style: "margin-bottom: 32px;",
                div { class: "section-title", style: "margin-top: 0;", "SELECTION" }
                
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                    div { class: "form-group",
                        label { class: "form-label", "TARGET WORKFLOW" }
                        select {
                            onchange: move |evt| {
                                let val = evt.value();
                                selected_workflow.set(Some(val.clone()));
                                if let Some(w) = workflows.read().iter().find(|w| w.name == val) {
                                    selected_version.set(w.versions.first().cloned());
                                }
                            },
                            option { value: "", "Choose workflow..." }
                            for w in workflows.read().iter() {
                                option { value: "{w.name}", "{w.name}" }
                            }
                        }
                    }

                    if let Some(w_name) = selected_workflow.read().as_ref() {
                        div { class: "form-group",
                            label { class: "form-label", "ARTIFACT VERSION" }
                            select {
                                value: selected_version.read().clone().unwrap_or_default(),
                                onchange: move |evt| selected_version.set(Some(evt.value())),
                                for v in workflows.read().iter().find(|w| &w.name == w_name).map(|w| &w.versions).unwrap_or(&vec![]) {
                                    option { value: "{v}", "{v}" }
                                }
                            }
                        }
                    }
                }
            }

            if *loading.read() {
                div { style: "padding: 40px; text-align: center;",
                    div { class: "spinner spinner-lg", style: "margin: 0 auto 16px;" }
                    div { style: "color: var(--text-faint);", "Synthesizing graph symbols..." }
                }
            } else if let Some(err) = error.read().as_ref() {
                div { class: "status-pill status-pill-danger", style: "display: block; width: fit-content; margin: 0 auto;", "{err}" }
            } else if let Some(detail) = workflow_detail.read().as_ref() {
                if let Some(ast_str) = detail.get("ast_json").and_then(|v| v.as_str()) {
                    if let Ok(ast) = serde_json::from_str::<serde_json::Value>(ast_str) {
                        section { class: "card", style: "padding: 0; overflow: hidden; background: #fff; min-height: 500px;",
                            div { style: "padding: 24px; border-bottom: 1px solid var(--border); background: var(--bg); display: flex; justify-content: space-between; align-items: center;",
                                h3 { style: "margin: 0; font-size: 1.25rem;", "LOGIC TOPOLOGY" }
                                span { class: "status-pill", "Deterministic View" }
                            }
                            WorkflowGraph { injected_ast: ast }
                        }
                    }
                }
            } else {
                div { style: "height: 300px; display: flex; flex-direction: column; align-items: center; justify-content: center; border: 1px dashed var(--border-strong); border-radius: 12px; color: var(--text-faint);",
                    span { style: "font-size: 2rem; margin-bottom: 12px;", "✧" }
                    div { "Select an artifact to begin visualization" }
                }
            }
        }
    }
}
