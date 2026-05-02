use dioxus::prelude::*;
use crate::api;

#[component]
pub fn TemplateMarketplace() -> Element {
    let templates_res = use_resource(api::list_templates);
    let nav = use_navigator();
    let mut selected_template = use_signal(|| None::<api::TemplateSummary>);
    let mut workflow_name = use_signal(|| "".to_string());
    let mut is_submitting = use_signal(|| false);

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "TEMPLATE MARKETPLACE" }
            p { style: "font-size: 1.2rem; color: var(--text-secondary); margin-bottom: 48px;",
                "Accelerate policy deployment with pre-verified industry standards and compliance frameworks."
            }

            match templates_res.read().as_ref() {
                Some(Ok(list)) => {
                    rsx! {
                        div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 24px;",
                            for template in list {
                                {
                                    let t = template.clone();
                                    rsx! {
                                        div { 
                                            class: "card template-card",
                                            style: "display: flex; flex-direction: column; cursor: pointer; transition: transform 0.2s;",
                                            onclick: move |_| selected_template.set(Some(t.clone())),
                                            div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 16px;",
                                                span { 
                                                    style: "font-size: 10px; font-weight: 800; text-transform: uppercase; letter-spacing: 0.1em; color: var(--accent-primary); background: rgba(var(--accent-primary-rgb), 0.1); padding: 4px 8px; border-radius: 4px;",
                                                    "{template.category}"
                                                }
                                                span { style: "font-size: 12px; color: var(--text-faint);", "Used {template.usage_count} times" }
                                            }
                                            h3 { style: "margin: 0 0 12px 0; font-size: 1.5rem;", "{template.name}" }
                                            p { style: "font-size: 14px; color: var(--text-secondary); line-height: 1.5; flex-grow: 1; margin-bottom: 24px;", "{template.description}" }
                                            
                                            div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                                                if let Some(frameworks) = &template.compliance_frameworks {
                                                    for fw in frameworks {
                                                        span { 
                                                            style: "font-size: 10px; font-weight: 700; color: var(--status-info); border: 1px solid var(--status-info); padding: 2px 6px; border-radius: 3px;",
                                                            "{fw}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                _ => rsx! {
                    div { style: "padding: 80px; text-align: center;",
                        div { class: "spinner", style: "margin: 0 auto 16px;" }
                        div { style: "color: var(--text-faint);", "Accessing global marketplace..." }
                    }
                }
            }

            // Modal for instantiation
            if let Some(template) = selected_template.read().clone() {
                div { 
                    class: "modal-overlay",
                    style: "display: flex; align-items: center; justify-content: center;",
                    div { 
                        class: "card", 
                        style: "width: 100%; max-width: 500px; padding: 40px;",
                        h2 { style: "margin-top: 0; font-size: 2rem;", "Instantiate Template" }
                        p { style: "color: var(--text-secondary); margin-bottom: 32px;", "Choose a unique name for your new policy based on '{template.name}'." }
                        
                        div { style: "margin-bottom: 24px;",
                            label { class: "form-label", "Workflow Name" }
                            input {
                                class: "search-input",
                                style: "width: 100%; background: #fff;",
                                placeholder: "e.g. MyCustomSecurityCheck",
                                autofocus: true,
                                value: "{workflow_name}",
                                oninput: move |evt| workflow_name.set(evt.value())
                            }
                        }

                        div { style: "display: flex; gap: 16px; justify-content: flex-end; margin-top: 40px;",
                            button { 
                                class: "btn", 
                                onclick: move |_| selected_template.set(None),
                                "Cancel"
                            }
                            button { 
                                class: "btn btn-primary",
                                disabled: workflow_name.read().is_empty() || *is_submitting.read(),
                                onclick: move |_| {
                                    let id = template.id.clone();
                                    let name = workflow_name.read().clone();
                                    is_submitting.set(true);
                                    spawn(async move {
                                        match api::instantiate_template(&id, &name).await {
                                            Ok(_) => {
                                                crate::app::show_toast(format!("Successfully created policy '{}'", name), crate::app::ToastType::Success);
                                                nav.push(crate::app::Route::Ledger {});
                                            },
                                            Err(e) => {
                                                crate::app::show_toast(format!("Failed to instantiate: {}", e), crate::app::ToastType::Error);
                                                is_submitting.set(false);
                                            }
                                        }
                                    });
                                },
                                if *is_submitting.read() { "Creating..." } else { "Create Policy" }
                            }
                        }
                    }
                }
            }
        }
    }
}
