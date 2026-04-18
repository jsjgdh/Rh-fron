use dioxus::prelude::*;
use crate::components::workflow_graph::WorkflowGraph;

#[derive(PartialEq, Props, Clone)]
pub struct ViewEditProps {
    workflow_name: String,
    version: String,
    #[props(default)]
    on_back: EventHandler<()>,
    #[props(default)]
    on_recompiled: EventHandler<(String, String)>,
}

#[component]
pub fn ViewEdit(props: ViewEditProps) -> Element {
    let workflow_name = props.workflow_name.clone();
    let version = props.version.clone();
    let on_back = props.on_back.clone();
    let on_recompiled = props.on_recompiled.clone();
    
    let mut source_code = use_signal(|| String::new());
    let mut ast_payload = use_signal(|| Option::<serde_json::Value>::None);
    let mut compiling = use_signal(|| false);
    let mut compile_error = use_signal(|| String::new());
    let mut selected_step = use_signal(|| Option::<String>::None);

    use_effect({
        let workflow_name = workflow_name.clone();
        let version = version.clone();
        move || {
            let workflow_name = workflow_name.clone();
            let version = version.clone();
            spawn(async move {
                if let Ok(detail) = crate::api::get_workflow_detail(&workflow_name, &version).await {
                    if let Some(source) = detail.get("source").and_then(|v| v.as_str()) {
                        source_code.set(source.to_string());
                    }
                    if let Some(ast_str) = detail.get("ast_json").and_then(|v| v.as_str()) {
                        if let Ok(ast) = serde_json::from_str(ast_str) {
                            ast_payload.set(Some(ast));
                        }
                    }
                }
            });
        }
    });

    let steps = ast_payload.read().as_ref()
        .and_then(|ast| ast.get("steps").and_then(|s| s.as_array()))
        .cloned()
        .unwrap_or_default();

    rsx! {
        div { class: "builder-container",
            // ── Left Pane: Step Navigator ────────────────────────────
            aside { class: "builder-pane",
                div { class: "builder-pane-header", "Steps" }
                div { class: "sidebar-nav",
                    for step in steps.iter() {
                        {
                            let name = step.get("name").and_then(|v| v.as_str()).unwrap_or("?").to_string();
                            let is_active = selected_step.read().as_ref() == Some(&name);
                            rsx! {
                                div { 
                                    class: if is_active { "nav-item active" } else { "nav-item" },
                                    onclick: {
                                        let name = name.clone();
                                        move |_| selected_step.set(Some(name.clone()))
                                    },
                                    span { class: "nav-icon", "•" }
                                    span { "{name}" }
                                }
                            }
                        }
                    }
                }
            }

            // ── Center Pane: Visual Flow ─────────────────────────────
            main { class: "builder-pane",
                div { class: "builder-pane-header", 
                    "Visual Flow"
                    span { style: "margin-left: auto; color: var(--text-faint); font-weight: 400;", 
                        "v{version}"
                    }
                }
                div { class: "vg-canvas",
                    if let Some(ast) = ast_payload.read().clone() {
                        WorkflowGraph { 
                            injected_ast: ast,
                            active_steps: selected_step.read().as_ref().map(|s| vec![s.clone()])
                        }
                    } else {
                        div { class: "empty-state", "Loading graph..." }
                    }
                }
            }

            // ── Right Pane: Inspector & Logic ────────────────────────
            aside { class: "builder-pane",
                div { class: "builder-pane-header", "Properties" }
                div { class: "inspector-content",
                    style: "padding: 16px; display: flex; flex-direction: column; gap: 20px;",
                    
                    div { class: "form-group",
                        label { class: "form-label", "RheLang Source" }
                        textarea {
                            class: "form-input code-editor",
                            style: "height: 400px; font-size: 11px;",
                            value: "{source_code}",
                            oninput: move |e| source_code.set(e.value()),
                            spellcheck: false,
                        }
                    }

                    if !compile_error.read().is_empty() {
                        div { class: "badge badge-danger", style: "width: 100%; justify-content: flex-start;", 
                            "{compile_error}" 
                        }
                    }

                    button {
                        class: "btn btn-primary",
                        style: "width: 100%",
                        disabled: *compiling.read(),
                        onclick: move |_| {
                            let source = source_code.read().clone();
                            compiling.set(true);
                            spawn(async move {
                                match crate::api::compile_workflow(&source).await {
                                    Ok(res) if res.success => {
                                        on_recompiled.call((res.workflow_name, res.version));
                                    }
                                    Ok(res) => compile_error.set(res.error.unwrap_or_default()),
                                    Err(e) => compile_error.set(e),
                                }
                                compiling.set(false);
                            });
                        },
                        if *compiling.read() { "Compiling..." } else { "Compile & Release" }
                    }

                    button {
                        class: "btn btn-secondary",
                        style: "width: 100%",
                        onclick: move |_| on_back.call(()),
                        "Close Builder"
                    }
                }
            }
        }
    }
}
