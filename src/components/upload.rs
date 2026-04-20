use dioxus::prelude::*;

#[component]
pub fn Upload(#[props(default)] on_generation_complete: EventHandler<(String, String)>) -> Element {
    let mut nl_prompts = use_signal(|| String::new());
    let mut current_blueprint = use_signal(|| String::new());
    let mut bulk_status = use_signal(|| String::new());
    let mut bulk_progress = use_signal(|| 0.0);
    
    let mut generating = use_signal(|| false); // For the preview synthesis
    let mut bulk_creating = use_signal(|| false); 
    let mut importing = use_signal(|| false);
    let mut error_msg = use_signal(|| String::new());

    let opacity = if *importing.read() { "0.5" } else { "1.0" };
    let progress_width = format!("{}%", *bulk_progress.read());

    rsx! {
        div { class: "dashboard-stack studio-theme", style: "height: calc(100vh - 80px); padding: 24px;",
            
            section { class: "grid-2", style: "height: 100%; gap: 32px;",
                
                // ── Pane 1: Policy Deconstructor ────────────────────────
                div { class: "studio-glass-card", style: "display: flex; flex-direction: column;",
                    div { class: "builder-pane-header", 
                        "Policy Deconstructor"
                        if *importing.read() {
                            span { class: "badge badge-info", style: "margin-left: auto;", "Analyzing PDF..." }
                        }
                    }
                    div { style: "padding: 24px; flex: 1; display: flex; flex-direction: column;",
                        p { class: "label-caps", style: "margin-bottom: 8px; color: var(--brand-emerald);", "Deconstructed Intents" }
                        p { class: "card-description", style: "margin-bottom: 24px;", 
                            "Edit the identified workflow targets below. The bulk orchestrator will generate a RheLang blueprint for each numbered entry." 
                        }
                        
                        textarea {
                            class: "ide-textarea",
                            style: "flex: 1; min-height: 400px; padding: 20px; background: rgba(0,0,0,0.4); border: 1px solid var(--border-subtle); border-radius: 8px;",
                            placeholder: "Import a Policy PDF to begin deconstruction...",
                            value: "{nl_prompts}",
                            oninput: move |e| nl_prompts.set(e.value()),
                            spellcheck: false,
                        }

                        div { style: "display: flex; gap: 12px; margin-top: 24px;",
                            input {
                                id: "pdf-upload", r#type: "file", accept: ".pdf", style: "display: none;",
                                onchange: move |evt| {
                                    spawn(async move {
                                        if let Some(engine) = evt.files() {
                                            let files = engine.files();
                                            if !files.is_empty() {
                                                importing.set(true);
                                                error_msg.set(String::new());
                                                if let Some(contents) = engine.read_file(&files[0]).await {
                                                    match crate::api::extract_pdf(contents).await {
                                                        Ok(text) => nl_prompts.set(text),
                                                        Err(err) => error_msg.set(format!("Deconstruction failed: {err}")),
                                                    }
                                                }
                                                importing.set(false);
                                            }
                                        }
                                    });
                                }
                            }
                            label {
                                class: "btn btn-secondary", r#for: "pdf-upload",
                                style: "flex: 1; height: 48px; border-style: dashed; opacity: {opacity};",
                                if *importing.read() { "Analyzing Policy Traces..." } else { "Import Policy PDF" }
                            }
                            
                            button {
                                class: "btn btn-primary",
                                style: "flex: 1.5; height: 48px; font-weight: 800;",
                                disabled: nl_prompts.read().trim().is_empty() || *bulk_creating.read(),
                                onclick: move |_| {
                                    let content = nl_prompts.read().clone();
                                    bulk_creating.set(true);
                                    error_msg.set(String::new());
                                    
                                    spawn(async move {
                                        // 1. Parse prompts by newline starting with digit + parenthesis
                                        let prompts: Vec<String> = content.lines()
                                            .filter(|l| !l.trim().is_empty())
                                            .filter(|l| l.contains(')') ) // Simple heuristic for "1) ..."
                                            .map(|l| l.to_string())
                                            .collect();
                                        
                                        let total = prompts.len();
                                        if total == 0 {
                                            error_msg.set("No valid numbered workflows identified. Ensure entries follow '1) name: intent' format.".into());
                                            bulk_creating.set(false);
                                            return;
                                        }

                                        for (i, prompt) in prompts.iter().enumerate() {
                                            let current_num = i + 1;
                                            bulk_status.set(format!("Synthesizing {} ({} of {})...", prompt, current_num, total));
                                            bulk_progress.set((current_num as f32 / total as f32) * 100.0);

                                            // A. Generate Logic
                                            match crate::api::generate_workflow(prompt).await {
                                                Ok(gen_res) => {
                                                    if let Some(err) = gen_res.error {
                                                        error_msg.set(format!("Synthesis {} failed: {}", current_num, err));
                                                        break;
                                                    }
                                                    
                                                    // B. Commit to Ledger
                                                    bulk_status.set(format!("Committing {} to Forensic Ledger...", prompt));
                                                    match crate::api::compile_workflow(&gen_res.source_code).await {
                                                        Ok(comp_res) => {
                                                            if !comp_res.success {
                                                                error_msg.set(format!("Commit {} failed: {}", current_num, comp_res.error.unwrap_or_default()));
                                                                break;
                                                            }
                                                            // Success on this specific one
                                                        }
                                                        Err(e) => { error_msg.set(e); break; }
                                                    }
                                                }
                                                Err(e) => { error_msg.set(e); break; }
                                            }
                                        }
                                        
                                        if error_msg.read().is_empty() {
                                            bulk_status.set("Bulk Extraction Complete.".into());
                                            // Signal completion - usually would trigger a nav to dashboard
                                            on_generation_complete.call(("Bulk Creation".into(), "v1.0.0".into()));
                                        }
                                        bulk_creating.set(false);
                                    });
                                },
                                if *bulk_creating.read() { "Orchestrating Bulk Creation..." } else { "Bulk Create Workflows" }
                            }
                        }

                        if !error_msg.read().is_empty() {
                            div { class: "forensic-error", style: "margin-top: 16px;", "{error_msg}" }
                        }
                    }
                }

                // ── Pane 2: Live Orchestration Telemetry ─────────────────
                div { class: "studio-glass-card", style: "background: rgba(0,0,0,0.2);",
                    div { class: "builder-pane-header", "Orchestration Telemetry" }
                    div { style: "padding: 32px; height: 100%; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center;",
                        
                        if *bulk_creating.read() || !bulk_status.read().is_empty() {
                            div { style: "width: 100%;",
                                div { class: "label-caps", style: "color: var(--brand-emerald); margin-bottom: 24px;", "Status: Performing Bulk Synthesis" }
                                
                                div { style: "font-size: 18px; margin-bottom: 32px; color: var(--text-primary); font-weight: 500;", "{bulk_status}" }
                                
                                // Progress Bar
                                div { style: "width: 100%; height: 8px; background: rgba(255,255,255,0.05); border-radius: 100px; overflow: hidden; margin-bottom: 12px;",
                                    div { 
                                        style: "width: {progress_width}; height: 100%; background: var(--brand-emerald); box-shadow: 0 0 20px var(--brand-emerald); transition: width 0.5s ease;",
                                    }
                                }
                                div { style: "font-size: 11px; color: var(--text-faint);", "{(*bulk_progress.read() as f32).round()}% Complete" }
                            }
                        } else {
                            div { 
                                style: "color: var(--text-faint);",
                                div { class: "pulse-ring", style: "margin: 0 auto 24px auto;" }
                                p { "Awaiting policy deconstruction data." }
                                p { style: "font-size: 12px; margin-top: 8px;", "Blueprints will appear here during bulk orchestration." }
                            }
                        }
                    }
                }
            }
        }
    }
}
