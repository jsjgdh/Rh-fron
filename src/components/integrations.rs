use dioxus::prelude::*;
use crate::api;

#[component]
pub fn Integrations() -> Element {
    let integrations = use_resource(api::get_integrations);
    let mut selected_service = use_signal(|| String::new());
    let mut api_key = use_signal(|| String::new());
    let mut status_msg = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    // Email Specific Configuration States
    let mut smtp_host = use_signal(|| "smtp.gmail.com".to_string());
    let mut smtp_port = use_signal(|| "465".to_string());
    let mut imap_host = use_signal(|| "imap.gmail.com".to_string());
    let mut imap_port = use_signal(|| "993".to_string());
    let mut imap_user = use_signal(|| String::new());
    
    // Multi-mapping state: Vec<(subject, workflow)>
    let mut email_mappings = use_signal(|| vec![
        ("leave approval".to_string(), "LeavePolicy".to_string())
    ]);

    // Email Specific Configuration States
    let mut smtp_host = use_signal(|| "smtp.gmail.com".to_string());
    let mut smtp_port = use_signal(|| "465".to_string());
    let mut imap_host = use_signal(|| "imap.gmail.com".to_string());
    let mut imap_port = use_signal(|| "993".to_string());
    let mut imap_user = use_signal(|| String::new());
    
    // Multi-mapping state: Vec<(subject, workflow)>
    let mut email_mappings = use_signal(|| vec![
        ("leave approval".to_string(), "LeavePolicy".to_string())
    ]);

    let mut webhooks = use_resource(api::list_webhooks);
    let mut new_webhook_name = use_signal(|| String::new());
    let mut new_webhook_workflow = use_signal(|| String::new());
    let mut new_webhook_version = use_signal(|| "v1.0".to_string());

    rsx! {
        div { class: "dashboard-stack control-integrations",
            section { class: "industrial-card glass detail-hero",
                div { style: "display: flex; justify-content: space-between; align-items: center;",
                    div {
                        div { class: "label-caps", style: "color: var(--accent);", "External Connections" }
                        h2 { class: "app-title", style: "font-size: 24px; margin-top: 8px;", "The Rhexiom Integration Pool" }
                        p { class: "panel-copy", style: "margin-top: 12px; color: var(--text-secondary);", "Securely link your policy engine to real-time data providers and operational services." }
                    }
                    span { class: "status-pill", "connect" }
                }
            }

            // Webhook Registry Section
            section { class: "industrial-card glass",
                div { class: "label-caps", style: "color: var(--accent);", "Dynamic Webhook Registry" }
                p { class: "panel-copy", style: "margin-top: 8px; color: var(--text-secondary);", "Link external system triggers (Stripe, GitHub, PagerDuty) directly to governed workflows." }
                
                div { style: "margin-top: 32px;",
                    div { class: "grid-metrics",
                        {
                            let hooks_data = webhooks.read();
                            match hooks_data.as_ref() {
                                Some(Ok(hooks)) => {
                                    let hooks_owned = hooks.clone();
                                    rsx! {
                                        for hook in hooks_owned {
                                            {
                                                let hook_id_outer = hook["webhook_id"].as_str().unwrap_or_default().to_string();
                                                let hook_name = hook["name"].as_str().unwrap_or_default().to_string();
                                                let wf_name = hook["workflow_name"].as_str().unwrap_or_default().to_string();
                                                let wf_ver = hook["version"].as_str().unwrap_or_default().to_string();
                                                
                                                let mut webhooks_del = webhooks.clone();
                                                let hid_del = hook_id_outer.clone();

                                                rsx! {
                                                    div { class: "industrial-card glass", style: "background: var(--bg);",
                                                        div { style: "display: flex; justify-content: space-between; align-items: flex-start;",
                                                            div {
                                                                div { class: "label-caps", style: "font-size: 10px;", "Webhook ID: {hook_id_outer}" }
                                                                div { class: "brand-name", style: "color: var(--text-primary); margin: 4px 0;", "{hook_name}" }
                                                                div { style: "display: flex; align-items: center; gap: 8px;",
                                                                    span { class: "status-pill", style: "padding: 2px 6px; font-size: 9px;", "Active" }
                                                                    span { style: "font-size: 12px; color: var(--text-faint);", "Triggers {wf_name} ({wf_ver})" }
                                                                }
                                                            }
                                                            button { 
                                                                class: "btn btn-secondary", 
                                                                style: "color: var(--accent); padding: 4px; border-color: transparent;",
                                                                onclick: move |_| {
                                                                    let mut webhooks = webhooks_del.clone();
                                                                    let hid = hid_del.clone();
                                                                    spawn(async move {
                                                                        let _ = api::delete_webhook(&hid).await;
                                                                        webhooks.restart();
                                                                    });
                                                                },
                                                                "✕"
                                                            }
                                                        }
                                                        div { class: "form-group", style: "margin-top: 16px;",
                                                            label { class: "form-label", style: "font-size: 10px;", "Ingress Endpoint" }
                                                            input { 
                                                                class: "form-input", 
                                                                style: "font-family: var(--font-mono); font-size: 11px; background: rgba(0,0,0,0.3);",
                                                                readonly: true, 
                                                                value: "http://localhost:3001/api/v1/webhooks/{hook_id_outer}" 
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                _ => rsx! { div { class: "panel-copy", "Polling webhook registrations..." } }
                            }
                        }
                        
                        // Create New Webhook Ingress
                        div { class: "industrial-card glass", style: "border: 1px dashed var(--panel-lighter);",
                            div { class: "label-caps", style: "font-size: 10px;", "New Ingress" }
                            div { class: "form-group", style: "margin-top: 12px;",
                                input { 
                                    class: "form-input", placeholder: "Webhook Name (e.g. Sales Alert)",
                                    value: "{new_webhook_name}",
                                    oninput: move |e| new_webhook_name.set(e.value())
                                }
                                input { 
                                    class: "form-input", style: "margin-top: 8px;", placeholder: "Workflow Name",
                                    value: "{new_webhook_workflow}",
                                    oninput: move |e| new_webhook_workflow.set(e.value())
                                }
                            }
                            button { 
                                class: "btn btn-primary", style: "width: 100%; margin-top: 12px;",
                                disabled: new_webhook_name.read().is_empty() || new_webhook_workflow.read().is_empty(),
                                onclick: move |_| {
                                    let name = new_webhook_name.read().clone();
                                    let w_name = new_webhook_workflow.read().clone();
                                    let w_ver = new_webhook_version.read().clone();
                                    let mut webhooks = webhooks.clone();
                                    spawn(async move {
                                        let _ = api::create_webhook(&name, &w_name, &w_ver).await;
                                        new_webhook_name.set(String::new());
                                        new_webhook_workflow.set(String::new());
                                        webhooks.restart();
                                    });
                                },
                                "+ Register Webhook" 
                            }
                        }
                    }
                }
            }

             section { class: "industrial-card",
                div { class: "label-caps", style: "margin-bottom: 32px;", "Service Connectors" }
                
                match &*integrations.read() {
                    Some(Ok(services)) => rsx! {
                        div { class: "grid-metrics",
                            for service in services {
                                {
                                    let s = service.clone();
                                    let is_active = *selected_service.read() == *service;
                                    let icon = if s.to_lowercase().contains("salesforce") { "☁" } 
                                              else if s.to_lowercase().contains("hubspot") { "◎" }
                                              else if s.to_lowercase() == "email" { "✉" }
                                              else { "⇄" };
                                    
                                    rsx! {
                                        div { 
                                            class: if is_active { "industrial-card glass" } else { "industrial-card" },
                                            style: if is_active { "border-color: var(--accent); background: var(--bg); cursor: pointer;" } else { "cursor: pointer;" },
                                            onclick: move |_| {
                                                selected_service.set(s.clone());
                                                status_msg.set(String::new());
                                                api_key.set(String::new());
                                            },
                                            div { 
                                                style: "display: flex; align-items: center; gap: 16px;",
                                                div { class: "brand-mark", style: "background: var(--panel-lighter); color: var(--accent);", "{icon}" }
                                                div {
                                                    div { class: "label-caps", style: "margin: 0; font-size: 10px;", "Service Connector" }
                                                    div { class: "brand-name", style: "color: var(--text-primary); font-size: 16px;", "{service}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! { div { class: "auth-error", "Failed to load services: {e}" } },
                    None => rsx! { div { style: "padding: 80px 0; text-align: center; color: var(--text-faint);", "Polling registries..." } }
                }
            }

            if !selected_service.read().is_empty() {
                section { class: "industrial-card",
                    div { style: "margin-bottom: 32px;",
                        div { class: "label-caps", "Configuration Panel" }
                        h3 { class: "app-title", style: "font-size: 20px; margin-top: 8px;", "Configure {selected_service}" }
                        p { class: "panel-copy", style: "margin-top: 8px; font-size: 14px; color: var(--text-secondary);", "Provide credentials to enable this sink/source." }
                    }
                    
                    div { class: "form-group", style: "max-width: 480px;",
                        label { class: "form-label", "API Key / Infrastructure Token" }
                        input { 
                            r#type: "password",
                            placeholder: "Enter token for {selected_service}",
                            value: "{api_key}",
                            oninput: move |evt| api_key.set(evt.value()),
                            class: "form-input",
                        }
                    }

                    if *selected_service.read() == "email" {
                        div { style: "margin-top: 32px; border-top: 1px solid var(--panel-lighter); padding-top: 32px;",
                            div { class: "label-caps", "Email Server Settings" }
                            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px; margin-top: 24px;",
                                div { class: "form-group",
                                    label { class: "form-label", "SMTP Host" }
                                    input { 
                                        class: "form-input",
                                        value: "{smtp_host}",
                                        oninput: move |e| smtp_host.set(e.value())
                                    }
                                }
                                div { class: "form-group",
                                    label { class: "form-label", "SMTP Port" }
                                    input { 
                                        class: "form-input",
                                        value: "{smtp_port}",
                                        oninput: move |e| smtp_port.set(e.value())
                                    }
                                }
                                div { class: "form-group",
                                    label { class: "form-label", "IMAP Host" }
                                    input { 
                                        class: "form-input",
                                        value: "{imap_host}",
                                        oninput: move |e| imap_host.set(e.value())
                                    }
                                }
                                div { class: "form-group",
                                    label { class: "form-label", "IMAP Port" }
                                    input { 
                                        class: "form-input",
                                        value: "{imap_port}",
                                        oninput: move |e| imap_port.set(e.value())
                                    }
                                }
                            }
                            div { class: "form-group", style: "margin-top: 24px;",
                                label { class: "form-label", "User / Email Address" }
                                input { 
                                    class: "form-input",
                                    placeholder: "e.g. system@yourdomain.com",
                                    value: "{imap_user}",
                                    oninput: move |e| imap_user.set(e.value())
                                }
                            }

                            div { style: "margin-top: 32px; background: var(--bg); padding: 24px; border-radius: 8px;",
                                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",
                                    div { class: "label-caps", "Intent Mappings (Neural Triggers)" }
                                    button { 
                                        class: "btn btn-secondary", 
                                        style: "padding: 4px 12px; font-size: 12px;",
                                        onclick: move |_| {
                                            email_mappings.with_mut(|m| m.push(("new pattern".to_string(), "NewWorkflow".to_string())));
                                        },
                                        "+ Add Trigger"
                                    }
                                }
                                
                                for (idx, (subject, workflow)) in email_mappings.read().iter().enumerate() {
                                    div { 
                                        key: "{idx}",
                                        style: "display: grid; grid-template-columns: 1fr 1fr auto; gap: 16px; margin-bottom: 16px; align-items: flex-end;",
                                        div { class: "form-group", style: "margin: 0;",
                                            label { class: "form-label", style: "font-size: 11px;", "Subject Pattern" }
                                            input { 
                                                class: "form-input",
                                                value: "{subject}",
                                                oninput: move |e| {
                                                    email_mappings.with_mut(|m| m[idx].0 = e.value());
                                                }
                                            }
                                        }
                                        div { class: "form-group", style: "margin: 0;",
                                            label { class: "form-label", style: "font-size: 11px;", "Linked Workflow" }
                                            input { 
                                                class: "form-input",
                                                value: "{workflow}",
                                                oninput: move |e| {
                                                    email_mappings.with_mut(|m| m[idx].1 = e.value());
                                                }
                                            }
                                        }
                                        button {
                                            class: "btn btn-secondary",
                                            style: "padding: 8px; color: var(--accent); border-color: transparent;",
                                            onclick: move |_| {
                                                email_mappings.with_mut(|m| { m.remove(idx); });
                                            },
                                            "✕"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !status_msg.read().is_empty() {
                        div { 
                            class: "status-pill", 
                            style: "margin: 24px 0; background: var(--bg); color: var(--accent); display: inline-block;", 
                            "{status_msg}" 
                        }
                    }

                    div { 
                        style: "display: flex; gap: 16px; margin-top: 32px;",
                        button {
                            class: "btn btn-primary",
                            disabled: *is_loading.read() || api_key.read().is_empty(),
                            onclick: move |_| {
                                let service = selected_service.read().clone();
                                let key = api_key.read().clone();
                                
                                let config = if service == "email" {
                                    let current_mappings: Vec<serde_json::Value> = email_mappings.read().iter().map(|(s, w)| {
                                        serde_json::json!({ "subject": s, "workflow": w })
                                    }).collect();

                                    serde_json::json!({
                                        "smtp_host": *smtp_host.read(),
                                        "smtp_port": smtp_port.read().parse::<u16>().unwrap_or(465),
                                        "imap_host": *imap_host.read(),
                                        "imap_port": imap_port.read().parse::<u16>().unwrap_or(993),
                                        "user": *imap_user.read(),
                                        "mappings": current_mappings
                                    })
                                } else {
                                    serde_json::json!({})
                                };

                                is_loading.set(true);
                                
                                spawn(async move {
                                    match api::update_integration(&service, &key, config).await {
                                        Ok(_) => status_msg.set("Credential sync successful!".to_string()),
                                        Err(e) => status_msg.set(format!("Sync Error: {}", e))
                                    }
                                    is_loading.set(false);
                                });
                            },
                            if *is_loading.read() { "Synchronizing..." } else { "Vault Sync Credentials" }
                        }
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| selected_service.set(String::new()),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
