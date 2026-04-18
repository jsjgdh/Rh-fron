use dioxus::prelude::*;

#[component]
pub fn Upload(#[props(default)] on_generation_complete: EventHandler<(String, String)>) -> Element {
    let mut nl_prompt = use_signal(|| String::new());
    let mut generating = use_signal(|| false);
    let mut importing = use_signal(|| false);
    let mut error_msg = use_signal(|| String::new());

    rsx! {
        div { class: "page-stack",
            section { class: "grid-2 upload-layout",
                div { class: "card composer-card",
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Compose a policy brief" }
                            div { class: "card-description", "Use natural language, raw RheLang, or text extracted from a document." }
                        }
                        if *generating.read() {
                            span { class: "badge badge-warning", "generating" }
                        } else if *importing.read() {
                            span { class: "badge badge-neutral", "importing" }
                        } else {
                            span { class: "badge badge-success", "ready" }
                        }
                    }

                    if *generating.read() {
                        div { class: "progress-shell",
                            div { class: "progress-copy",
                                h3 { "Compiling your policy flow" }
                                p { "Rhexiom is turning the brief into RheLang, validating it, and preparing the first immutable version." }
                            }
                            div { class: "progress-bar",
                                div { class: "progress-bar-fill" }
                            }
                        }
                    } else {
                        div { class: "composer-surface",
                            label { class: "form-label", "Policy prompt" }
                            p { class: "form-hint", "Describe goals, thresholds, approval logic, and any branching behavior you want encoded." }
                            textarea {
                                class: "upload-textarea",
                                placeholder: "Example: Create an expense approval workflow where requests above 5,000 go to a manager, missing receipts are rejected, and approved requests return an audit action.",
                                value: "{nl_prompt}",
                                oninput: move |e| nl_prompt.set(e.value()),
                            }

                            div { class: "starter-row",
                                button {
                                    class: "starter-chip",
                                    onclick: move |_| nl_prompt.set("Create a travel approval workflow where trips above 3000 require finance approval, international trips require legal review, and every accepted request emits an action to book travel.".to_string()),
                                    "Travel approvals"
                                }
                                button {
                                    class: "starter-chip",
                                    onclick: move |_| nl_prompt.set("Model an employee onboarding workflow where background checks gate account creation, managers approve equipment budgets, and failure states return the missing prerequisite.".to_string()),
                                    "Employee onboarding"
                                }
                                button {
                                    class: "starter-chip",
                                    onclick: move |_| nl_prompt.set("Write a refund policy that approves refunds under 100 instantly, escalates orders above 1000, and rejects requests marked as fraudulent.".to_string()),
                                    "Refund routing"
                                }
                            }

                            if !error_msg.read().is_empty() {
                                div { class: "status-message status-message-error", "{error_msg}" }
                            }

                            div { class: "upload-toolbar",
                                button {
                                    class: "btn btn-primary",
                                    disabled: nl_prompt.read().trim().is_empty(),
                                    onclick: move |_| {
                                        let prompt = nl_prompt.read().clone();
                                        generating.set(true);
                                        error_msg.set(String::new());

                                        spawn(async move {
                                            let source = match crate::api::generate_workflow(&prompt).await {
                                                Ok(res) => {
                                                    if let Some(err) = res.error {
                                                        error_msg.set(err);
                                                        generating.set(false);
                                                        return;
                                                    }
                                                    res.source_code
                                                }
                                                Err(err) => {
                                                    error_msg.set(err);
                                                    generating.set(false);
                                                    return;
                                                }
                                            };

                                            match crate::api::compile_workflow(&source).await {
                                                Ok(compile_res) => {
                                                    if compile_res.success {
                                                        on_generation_complete.call((
                                                            compile_res.workflow_name,
                                                            compile_res.version,
                                                        ));
                                                    } else {
                                                        error_msg.set(
                                                            compile_res
                                                                .error
                                                                .unwrap_or_else(|| {
                                                                    "Compilation failed. Adjust the prompt and try again."
                                                                        .to_string()
                                                                }),
                                                        );
                                                    }
                                                }
                                                Err(err) => error_msg.set(err),
                                            }

                                            generating.set(false);
                                        });
                                    },
                                    "Generate workflow"
                                }

                                input {
                                    id: "pdf-upload-input",
                                    r#type: "file",
                                    accept: ".pdf",
                                    style: "display: none;",
                                    onchange: move |evt| {
                                        spawn(async move {
                                            if let Some(engine) = evt.files() {
                                                let files = engine.files();
                                                if !files.is_empty() {
                                                    importing.set(true);
                                                    error_msg.set(String::new());

                                                    if let Some(contents) = engine.read_file(&files[0]).await {
                                                        match crate::api::extract_pdf(contents).await {
                                                            Ok(text) => nl_prompt.set(text),
                                                            Err(err) => error_msg.set(err),
                                                        }
                                                    }

                                                    importing.set(false);
                                                }
                                            }
                                        });
                                    }
                                }

                                label {
                                    class: if *importing.read() {
                                        "btn btn-secondary disabled"
                                    } else {
                                        "btn btn-secondary"
                                    },
                                    r#for: "pdf-upload-input",
                                    if *importing.read() { "Extracting PDF" } else { "Import PDF" }
                                }
                            }
                        }
                    }
                }

                div { class: "stack-column",
                    div { class: "card" ,
                        div { class: "card-header",
                            div {
                                div { class: "card-title", "What happens next" }
                                div { class: "card-description", "A quick view of the authoring pipeline behind this screen." }
                            }
                        }
                        div { class: "pipeline-list compact",
                            div { class: "pipeline-step",
                                div { class: "pipeline-index", "01" }
                                div {
                                    div { class: "pipeline-title", "Generate" }
                                    p { class: "pipeline-copy", "The prompt is converted into RheLang source that matches the requested policy intent." }
                                }
                            }
                            div { class: "pipeline-step",
                                div { class: "pipeline-index", "02" }
                                div {
                                    div { class: "pipeline-title", "Validate" }
                                    p { class: "pipeline-copy", "The compiler checks graph shape, typing, and execution semantics before storage." }
                                }
                            }
                            div { class: "pipeline-step",
                                div { class: "pipeline-index", "03" }
                                div {
                                    div { class: "pipeline-title", "Store" }
                                    p { class: "pipeline-copy", "Successful compiles are captured as immutable versions with inspectable artifacts." }
                                }
                            }
                        }
                    }

                    div { class: "card",
                        div { class: "card-header",
                            div {
                                div { class: "card-title", "Prompt tips" }
                                div { class: "card-description", "Small details improve generation quality." }
                            }
                        }
                        ul { class: "bullet-list",
                            li { "Name the inputs you expect, especially booleans and numeric thresholds." }
                            li { "Call out approval and rejection outcomes so the graph can branch clearly." }
                            li { "Mention external actions if the runtime should emit them during execution." }
                        }
                    }
                }
            }
        }
    }
}
