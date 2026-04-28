//! Workflow creation component.
//!
//! Provides a text area for entering RheLang source (or describing a policy),
//! previewing the generated code, and compiling it. Designed as a split-pane
//! editor with source on the left and output on the right.

use dioxus::prelude::*;
use crate::i18n::messages::{workflow, labels, placeholders, format_message};

/// Workflow creator view — input RheLang, preview, and compile.
#[component]
pub fn WorkflowCreator() -> Element {
    let mut source_code = use_signal(|| {
        r#"workflow ExpenseApproval v1.0 {
    input {
        amount: number
        receipt: boolean
    }

    start check_amount

    step check_amount {
        if amount > 5000 {
            goto manager_approval
        } else {
            goto auto_approve
        }
    }

    step manager_approval {
        action request_manager_approval
        goto done
    }

    step auto_approve {
        action approve
        goto done
    }

    step done {
        action complete
    }
}"#
        .to_string()
    });

    let mut compile_status = use_signal(|| String::new());
    let mut compiled_output = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "source".to_string());

    // Natural Language Hooks
    let mut nl_prompt = use_signal(|| String::new());
    let mut generating = use_signal(|| false);

    rsx! {
        div {
            div { class: "page-header",
                h1 { class: "page-title", "Create Workflow" }
                p { class: "page-subtitle", "Write RheLang or describe your policy in natural language" }
            }

            div { class: "grid-2",
                // Editor panel
                div { class: "card",
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Source Editor" }
                            div { class: "card-description", "RheLang workflow definition" }
                        }
                        div { class: "action-group", style: "display: flex; gap: 8px; align-items: center;",
                            span { class: "badge badge-info", "EDIT" }
                        }
                    }
                    div { class: "form-group", style: "display: flex; gap: 8px; margin-bottom: 12px;",
                        input {
                            class: "form-input",
                            style: "flex: 1;",
                            value: "{nl_prompt}",
                            placeholder: placeholders::DESCRIPTION,
                            oninput: move |e| nl_prompt.set(e.value()),
                            disabled: *generating.read(),
                        }
                        button {
                            class: "btn btn-primary",
                            style: "white-space: nowrap;",
                            disabled: *generating.read(),
                            onclick: move |_| {
                                generating.set(true);
                                compile_status.set("Thinking...".to_string());
                                let p = nl_prompt.read().clone();

                                spawn(async move {
                                    match crate::api::generate_workflow(&p).await {
                                        Ok(res) => {
                                            if let Some(err) = res.error {
                                                compile_status.set(format_message(&workflow::GENERATION_FAILED));
                                                compiled_output.set(err);
                                            } else {
                                                source_code.set(res.source_code);
                                                compile_status.set(format_message(&workflow::GENERATED_SUCCESS));
                                            }
                                        },
                                        Err(e) => {
                                            compile_status.set(format_message(&crate::i18n::messages::system::NETWORK_ERROR));
                                            compiled_output.set(e);
                                        }
                                    }
                                    generating.set(false);
                                });
                            },
                            if *generating.read() { "..." } else { "{labels::GENERATE}" }
                        }
                    }
                    div { class: "form-group",
                        textarea {
                            class: "form-textarea",
                            style: "min-height: 420px;",
                            value: "{source_code}",
                            oninput: move |e| source_code.set(e.value()),
                            spellcheck: false,
                        }
                    }
                    div { class: "action-bar",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                let source = source_code.read().clone();
                                compile_status.set("Compiling...".to_string());
                                compiled_output.set("".to_string());

                                spawn(async move {
                                    match crate::api::compile_workflow(&source).await {
                                        Ok(res) => {
                                            if !res.success {
                                                compile_status.set(format_message(&workflow::COMPILE_FAILED));
                                                let mut trace = String::new();
                                                if let Some(err) = &res.error { trace.push_str(&format!("ERROR:\n{}\n\n", err)); }
                                                if let Some(code) = &res.generated_rust { trace.push_str(&format!("Generated Rust (partial):\n{}", code)); }
                                                compiled_output.set(trace);
                                            } else {
                                                compile_status.set(format!("{} ({} v{})", workflow::COMPILE_SUCCESS.text, res.workflow_name, res.version));
                                                if let Some(code) = &res.generated_rust { compiled_output.set(code.clone()); }
                                            }
                                        },
                                        Err(e) => {
                                            compile_status.set(format_message(&crate::i18n::messages::system::NETWORK_ERROR));
                                            compiled_output.set(e);
                                        }
                                    }
                                });
                            },
                            "{labels::COMPILE}"
                        }
                        label {
                            class: "btn btn-secondary",
                            style: "cursor: pointer; margin: 0; padding: 8px 16px;",
                            "{labels::UPLOAD}"
                            input {
                                type: "file",
                                accept: ".rhe,.txt",
                                style: "display: none;",
                                onchange: move |evt| {
                                    if let Some(file_engine) = evt.files() {
                                        spawn(async move {
                                            if let Some(filename) = file_engine.files().first() {
                                                if let Some(contents) = file_engine.read_file_to_string(filename).await {
                                                    source_code.set(contents);
                                                    compile_status.set("Loaded file".to_string());
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                        }
                        div { class: "action-bar-spacer" }
                        button { class: "btn btn-ghost",
                            "{labels::REFRESH}"
                        }
                    }
                }

                // Preview panel
                div { class: "card",
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Output" }
                            div { class: "card-description", "Compilation artifacts" }
                        }
                        if !compile_status.read().is_empty() {
                            span { class: "badge badge-success", "{compile_status}" }
                        }
                    }

                    // Tab bar
                    div { class: "tab-bar",
                        div {
                            class: if *active_tab.read() == "source" { "tab active" } else { "tab" },
                            onclick: move |_| active_tab.set("source".to_string()),
                            "Generated Rust"
                        }
                        div {
                            class: if *active_tab.read() == "ast" { "tab active" } else { "tab" },
                            onclick: move |_| active_tab.set("ast".to_string()),
                            "AST"
                        }
                        div {
                            class: if *active_tab.read() == "ir" { "tab active" } else { "tab" },
                            onclick: move |_| active_tab.set("ir".to_string()),
                            "IR"
                        }
                    }

                    div { class: "code-block", style: "min-height: 380px;",
                        if compiled_output.read().is_empty() {
                            span { style: "color: var(--text-dim);",
                                "Compile to see generated output"
                            }
                        } else {
                            "{compiled_output}"
                        }
                    }

                    div { class: "action-bar", style: "margin-top: 12px;",
                        button { class: "btn btn-success",
                            "{labels::DEPLOY}"
                        }
                        button { class: "btn btn-secondary",
                            "{labels::COPY}"
                        }
                    }
                }
            }
        }
    }
}
