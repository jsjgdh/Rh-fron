use dioxus::prelude::*;
use crate::api;
use crate::app::{Route, show_toast, ToastType};

#[component]
pub fn Upload() -> Element {
    let mut file_content = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut is_shadow_mode = use_signal(|| false);
    let mut progress = use_signal(|| 0);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let nav = use_navigator();

    let handle_upload = move |_| {
        let content = file_content.read().clone();
        let mut n = nav.clone();
        
        spawn(async move {
            is_loading.set(true);
            error_msg.set(None);
            progress.set(10);
            
            // Intelligence detection: Is this RheLang or Natural Language?
            let is_rhelang = content.contains("workflow") && content.contains("{");
            
            if !is_rhelang {
                progress.set(30);
                // Deconstruct natural language policy into workflow intents first
                match api::deconstruct_policy(&content).await {
                    Ok(deconstruct_res) => {
                        if let Some(err) = deconstruct_res.error {
                            error_msg.set(Some(format!("Policy Deconstruction failed: {}", err)));
                            is_loading.set(false);
                            return;
                        }
                        
                        // Store deconstructed workflows for reference
                        let deconstructed_workflows = deconstruct_res.deconstructed_workflows;
                        progress.set(50);
                        
                        // Now generate RheLang from the deconstructed intents
                        match api::generate_workflow(&deconstructed_workflows).await {
                            Ok(res) => {
                                if let Some(err) = res.error {
                                    error_msg.set(Some(format!("AI Architech failed: {}", err)));
                                } else {
                                    file_content.set(res.source_code);
                                    progress.set(70);
                                    // Now try to compile the generated code
                                    let final_content = file_content.read().clone();
                                    match api::compile_workflow(&final_content).await {
                                        Ok(comp_res) => {
                                            if let Some(err) = comp_res.error {
                                                let err_msg = format!("Auto-generated code has issues: {}", err);
                                                error_msg.set(Some(err_msg.clone()));
                                                show_toast(err_msg, ToastType::Error);
                                            } else {
                                                progress.set(100);
                                                show_toast(format!("Workflow '{}' compiled successfully", comp_res.workflow_name), ToastType::Success);
                                                n.push(Route::ViewGenWorkflow {
                                                    name: comp_res.workflow_name,
                                                    version: comp_res.version
                                                });
                                            }
                                        }
                                        Err(e) => {
                                            error_msg.set(Some(e.clone()));
                                            show_toast(e, ToastType::Error);
                                        }
                                    }
                                }
                            }
                            Err(e) => { error_msg.set(Some(format!("AI Architech Offline: {}", e))); }
                        }
                    }
                    Err(e) => { error_msg.set(Some(format!("Policy Deconstruction failed: {}", e))); }
                }
            } else {
                progress.set(40);
                match api::compile_workflow(&content).await {
                    Ok(res) => {
                        if let Some(err) = res.error {
                            error_msg.set(Some(err.clone()));
                            show_toast(err, ToastType::Error);
                        } else {
                            progress.set(100);
                            show_toast(format!("Workflow '{}' compiled successfully", res.workflow_name), ToastType::Success);
                            n.push(Route::ViewGenWorkflow {
                                name: res.workflow_name,
                                version: res.version
                            });
                        }
                    }
                    Err(e) => {
                        error_msg.set(Some(e.clone()));
                        show_toast(e, ToastType::Error);
                    }
                }
            }
            is_loading.set(false);
        });
    };

    let handle_repair = move |_| {
        let content = file_content.read().clone();
        spawn(async move {
            is_loading.set(true);
            error_msg.set(None);
            progress.set(20);
            // Repair should use generate_workflow for RheLang code fixes
            match api::generate_workflow(&format!("FIX THE FOLLOWING RHELANG CODE WHICH HAS ERRORS:\n\n{}", content)).await {
                Ok(res) => {
                    if let Some(err) = res.error {
                        error_msg.set(Some(format!("Repair failed: {}", err)));
                    } else {
                        file_content.set(res.source_code);
                        progress.set(100);
                    }
                }
                Err(e) => { error_msg.set(Some(e)); }
            }
            is_loading.set(false);
        });
    };

    rsx! {
        div { class: "fade-in",
            div { style: "margin-bottom: 48px;",
                h1 { class: "page-title", "LOGIC STUDIO" }
                p { style: "font-size: 1.1rem; color: var(--text-secondary); max-width: 600px;",
                    "Architect deterministic RheLang policies using high-fidelity AI deconstruction or direct source input."
                }
            }

            div { style: "display: grid; grid-template-columns: 320px 1fr; gap: 40px; align-items: start;",
                // ── Control Panel ───────────────────────────────────
                aside { style: "display: flex; flex-direction: column; gap: 24px;",
                    div { class: "card",
                        div { class: "section-title", style: "margin-top: 0;", "ORCHESTRATION" }
                        
                        div { style: "margin-bottom: 24px;",
                            label { class: "form-label", style: "display: block; margin-bottom: 12px;", "Execution Mode" }
                            div { 
                                style: "display: flex; gap: 8px; background: var(--bg); padding: 4px; border-radius: 8px; border: 1px solid var(--border);",
                                button { 
                                    class: if !*is_shadow_mode.read() { "btn-primary" } else { "" },
                                    style: "flex: 1; padding: 8px; font-size: 11px; border-radius: 6px; border: none;",
                                    onclick: move |_| is_shadow_mode.set(false),
                                    "LIVE" 
                                }
                                button { 
                                    class: if *is_shadow_mode.read() { "btn-primary" } else { "" },
                                    style: "flex: 1; padding: 8px; font-size: 11px; border-radius: 6px; border: none;",
                                    onclick: move |_| is_shadow_mode.set(true),
                                    "SHADOW" 
                                }
                            }
                            p { style: "font-size: 11px; color: var(--text-faint); margin-top: 8px;", 
                                if *is_shadow_mode.read() { "Shadow mode captures trace data without triggering external side-effects." } else { "Live mode triggers all connected service hooks." }
                            }
                        }

                        button { 
                            class: "btn btn-primary", 
                            style: "width: 100%; height: 52px; font-size: 1rem;",
                            disabled: *is_loading.read() || file_content.read().is_empty(),
                            onclick: handle_upload,
                            if *is_loading.read() { "DECONSTRUCTING..." } else { "START COMPILATION" }
                        }

                        if let Some(err) = error_msg.read().as_ref() {
                            div { 
                                style: "margin-top: 20px; padding: 12px; background: rgba(239, 68, 68, 0.1); color: var(--status-error); border: 1px solid rgba(239, 68, 68, 0.2); border-radius: 8px; font-size: 13px;",
                                div { style: "margin-bottom: 12px;", "Pipeline Error: {err}" }
                                button { 
                                    class: "btn", 
                                    style: "width: 100%; font-size: 11px; background: var(--status-error); color: white; border: none;",
                                    onclick: handle_repair,
                                    "REPAIR WITH AI"
                                }
                            }
                        }
                    }

                    div { class: "card",
                        div { class: "section-title", style: "margin-top: 0;", "AI CO-PILOT" }
                        div { style: "display: flex; flex-direction: column; gap: 12px;",
                            div { style: "font-size: 13px; color: var(--text-secondary);", "Suggested Intent:" }
                            div { class: "badge badge-success", style: "width: fit-content;", "Approval Gate Detected" }
                            p { style: "font-size: 12px; color: var(--text-faint);", "Adding a 'timeout' block to the next step is recommended for SLA compliance." }
                        }
                    }
                }

                // ── Main Canvas ─────────────────────────────────────
                div {
                    div { 
                        style: "display: flex; justify-content: space-between; align-items: flex-end; margin-bottom: 16px;",
                        div { class: "section-title", style: "margin: 0;", "SOURCE LOGIC" }
                        div { style: "display: flex; gap: 12px;",
                             label { 
                                class: "btn btn-secondary", 
                                style: "height: 32px; font-size: 11px; cursor: pointer; display: flex; align-items: center;",
                                r#for: "pdf-upload",
                                "PDF IMPORT" 
                            }
                            button { class: "btn", style: "height: 32px; font-size: 11px;", "LOAD TEMPLATE" }
                        }
                    }
                    
                    input {
                        id: "pdf-upload",
                        r#type: "file",
                        accept: ".pdf",
                        style: "display: none;",
                        onchange: move |evt| {
                            spawn(async move {
                                if let Some(file_engine) = evt.files() {
                                    let files = file_engine.files();
                                    if !files.is_empty() {
                                        if let Some(content) = file_engine.read_file(&files[0]).await {
                                            match api::extract_pdf(content).await {
                                                Ok(extract_res) => {
                                                    // Set the deconstructed workflows to the file content
                                                    file_content.set(extract_res.deconstructed_workflows);
                                                }
                                                Err(e) => {
                                                    error_msg.set(Some(format!("PDF extraction failed: {}", e)));
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }

                    div { 
                        class: "card",
                        style: "padding: 0; overflow: hidden; border-color: var(--border-strong);",
                        textarea { 
                            class: "mono",
                            style: "width: 100%; min-height: 500px; border: none; padding: 32px; margin: 0; font-size: 14px; background: transparent; line-height: 1.6;",
                            placeholder: "Paste natural language policy or RheLang source...",
                            value: "{file_content}",
                            oninput: move |e| file_content.set(e.value())
                        }
                    }
                    
                    if *is_loading.read() {
                        div { style: "margin-top: 24px; padding: 24px; background: var(--accent-bg); border-radius: 12px;",
                            div { style: "display: flex; justify-content: space-between; margin-bottom: 12px;",
                                span { style: "font-weight: 800; font-size: 11px; letter-spacing: 0.1em;", "AI DECONSTRUCTION PIPELINE" }
                                span { style: "font-weight: 900; font-size: 12px; font-family: var(--font-mono);", "{progress}%" }
                            }
                            div { style: "height: 6px; background: rgba(0,0,0,0.1); border-radius: 3px; overflow: hidden;",
                                div { 
                                    style: "height: 100%; background: var(--accent-primary); width: {progress}%; transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1);" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
