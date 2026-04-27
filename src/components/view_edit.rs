use dioxus::prelude::*;
use crate::components::workflow_graph::WorkflowGraph;
use crate::app::Route;

#[component]
pub fn ViewEdit(name: String, version: String) -> Element {
    let mut source_code = use_signal(|| String::new());
    let mut ast_payload = use_signal(|| Option::<serde_json::Value>::None);
    let mut compiling = use_signal(|| false);
    let mut compile_error = use_signal(|| String::new());
    let mut selected_step = use_signal(|| Option::<String>::None);
    let nav = use_navigator();

    use_effect({
        let name = name.clone();
        let version = version.clone();
        move || {
            let name = name.clone();
            let version = version.clone();
            spawn(async move {
                if let Ok(detail) = crate::api::get_workflow_detail(&name, &version).await {
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
        div { class: "studio-theme", style: "height: 100vh; overflow: hidden;",
            div { class: "studio-workbench",
                
                // ─── Studio Shell ────────────────────────────────────────
                div { class: "builder-container",
                
                    // 1. Left: Forensic Explorer
                    aside { class: "builder-pane",
                        div { class: "builder-pane-header", "Forensic Step Explorer" }
                        div { class: "sidebar-nav",
                            for step in steps.iter() {
                                {
                                    let step_name = step.get("name").and_then(|v| v.as_str()).unwrap_or("?").to_string();
                                    let is_active = selected_step.read().as_ref() == Some(&step_name);
                                    rsx! {
                                        div { 
                                            key: "{step_name}",
                                            class: if is_active { "nav-item active" } else { "nav-item" },
                                            onclick: {
                                                let step_name = step_name.clone();
                                                move |_| selected_step.set(Some(step_name.clone()))
                                            },
                                            span { class: "nav-icon", "◇" }
                                            span { "{step_name}" }
                                        }
                                    }
                                }
                            }
                        }
                        
                        div { style: "margin-top: auto; padding: 16px; border-top: 1px solid var(--border-subtle);",
                            button {
                                class: "btn btn-secondary",
                                style: "width: 100%; height: 36px; font-size: 11px;",
                                onclick: move |_| { nav.push(Route::ViewGenWorkflow { name: name.clone(), version: version.clone() }); },
                                "Close Studio"
                            }
                        }
                    }

                    // 2. Center: Forensic Visual Graph
                    main { class: "builder-pane",
                        div { class: "builder-pane-header", 
                            "Forensic Canvas"
                            span { style: "margin-left: auto; color: var(--text-faint); font-weight: 400; font-size: 10px;", 
                                "{name} • {version}"
                            }
                        }
                        div { class: "vg-canvas-svg",
                            if let Some(ast) = ast_payload.read().clone() {
                                WorkflowGraph { 
                                    injected_ast: ast,
                                    active_steps: selected_step.read().as_ref().map(|s| vec![s.clone()])
                                }
                            } else {
                                div { class: "empty-state", 
                                    div { class: "pulse-ring" }
                                    span { "Initializing visualization..." } 
                                }
                            }
                        }
                    }

                    // 3. Right: Logic Panel (IDE)
                    aside { class: "builder-pane",
                        div { class: "builder-pane-header", "Logic Inspector" }
                        div { class: "inspector-content",
                            style: "height: 100%; display: flex; flex-direction: column;",
                            
                            div { class: "ide-panel", style: "flex: 1; display: flex; flex-direction: column;",
                                div { class: "label-caps", style: "margin-bottom: 8px;", "Current RheLang Source" }
                                textarea {
                                    class: "ide-textarea",
                                    value: "{source_code}",
                                    oninput: move |e| source_code.set(e.value()),
                                    spellcheck: false,
                                }
                            }

                            if !compile_error.read().is_empty() {
                                div { class: "badge badge-danger", style: "margin: 12px; border-radius: 4px;", 
                                    "{compile_error}" 
                                }
                            }

                            div { style: "padding: 16px; background: rgba(0,0,0,0.2);",
                                button {
                                    class: "btn btn-primary",
                                    style: "width: 100%; height: 48px; font-weight: 700;",
                                    disabled: *compiling.read(),
                                    onclick: move |_| {
                                        let source = source_code.read().clone();
                                        let mut n = nav.clone();
                                        compiling.set(true);
                                        compile_error.set(String::new());
                                        spawn(async move {
                                            match crate::api::compile_workflow(&source).await {
                                                Ok(res) if res.success => {
                                                    n.push(Route::ViewGenWorkflow { name: res.workflow_name, version: res.version });
                                                }
                                                Ok(res) => compile_error.set(res.error.unwrap_or_else(|| "Logic audit failed.".into())),
                                                Err(e) => compile_error.set(e),
                                            }
                                            compiling.set(false);
                                        });
                                    },
                                    if *compiling.read() { "Auditing Logic..." } else { "Compile & Release" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
