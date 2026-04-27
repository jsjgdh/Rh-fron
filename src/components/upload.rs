use dioxus::prelude::*;
use crate::api;
use crate::app::Route;

#[component]
pub fn Upload() -> Element {
    let mut file_content = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut progress = use_signal(|| 0);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let nav = use_navigator();

    let handle_upload = move |_| {
        let content = file_content.read().clone();
        let mut n = nav.clone();
        
        spawn(async move {
            is_loading.set(true);
            progress.set(10);
            
            progress.set(30);
            match api::compile_workflow(&content).await {
                Ok(res) => {
                    if let Some(err) = res.error {
                        error_msg.set(Some(err));
                    } else {
                        progress.set(100);
                        n.push(Route::ViewGenWorkflow { 
                            name: res.workflow_name, 
                            version: res.version 
                        });
                    }
                }
                Err(e) => {
                    error_msg.set(Some(e));
                }
            }
            is_loading.set(false);
        });
    };

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "LOGIC STUDIO" }
            p { style: "font-size: 1.2rem; color: var(--text-secondary); margin-bottom: 48px;",
                "Deconstruct natural language policies into deterministic RheLang graphs."
            }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 40px; align-items: start;",
                // Configuration Side
                div {
                    div { class: "section-title", "ORCHESTRATION" }
                    button { 
                        class: "btn btn-primary", 
                        style: "width: 100%; height: 56px; font-size: 1.25rem; font-family: var(--font-display);",
                        disabled: *is_loading.read() || file_content.read().is_empty(),
                        onclick: handle_upload,
                        if *is_loading.read() { "DECONSTRUCTING..." } else { "START COMPILATION" }
                    }

                    if let Some(err) = error_msg.read().as_ref() {
                        div { 
                            style: "margin-top: 24px; padding: 16px; background: #fee2e2; color: #991b1b; border: 1px solid #fecaca; border-radius: 8px; font-size: 14px;",
                            "Error: {err}" 
                        }
                    }
                }

                // Input Side
                div {
                    div { 
                        style: "display: flex; justify-content: space-between; align-items: baseline;",
                        div { class: "section-title", "SOURCE LOGIC" }
                        label { 
                            class: "btn btn-secondary", 
                            style: "height: 32px; font-size: 11px; cursor: pointer; display: flex; align-items: center;",
                            r#for: "pdf-upload",
                            "Import from PDF" 
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
                                            if let Ok(text) = api::extract_pdf(content).await {
                                                file_content.set(text);
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                    div { 
                        class: "card",
                        style: "padding: 0; overflow: hidden;",
                        textarea { 
                            class: "mono",
                            style: "width: 100%; min-height: 400px; border: none; padding: 24px; margin: 0; font-size: 14px; background: #fff;",
                            placeholder: "Paste your natural language policy or RheLang code here...",
                            value: "{file_content}",
                            oninput: move |e| file_content.set(e.value())
                        }
                    }
                    
                    if *is_loading.read() {
                        div { style: "margin-top: 24px;",
                            div { style: "display: flex; justify-content: space-between; margin-bottom: 8px;",
                                span { class: "form-label", "PROCESSING PROGRESS" }
                                span { style: "font-weight: 700; font-size: 12px;", "{progress}%" }
                            }
                            div { style: "height: 6px; background: var(--border); border-radius: 3px; overflow: hidden;",
                                div { 
                                    style: "height: 100%; background: var(--accent-primary); width: {progress}%; transition: width 0.3s ease;" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
