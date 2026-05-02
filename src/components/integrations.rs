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
    
    // Multi-mapping state: Vec<(subject, workflow, version)>
    let mut email_mappings = use_signal(|| vec![
        ("leave approval".to_string(), "LeavePolicy".to_string(), "v1.0".to_string())
    ]);

    let webhooks = use_resource(api::list_webhooks);
    let mut new_webhook_name = use_signal(|| String::new());
    let mut new_webhook_workflow = use_signal(|| String::new());
    let new_webhook_version = use_signal(|| "v1.0".to_string());

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "Integrations" }
            p { "Securely link your policy engine to real-time data providers and operational services." }

            // Webhook Registry Section
            div { class: "section-title", "Webhook Ingress" }
            div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 20px;",
                {
                    let hooks_data = webhooks.read();
                    match hooks_data.as_ref() {
                        Some(Ok(hooks)) => {
                            let hooks_owned = hooks.clone();
                            rsx! {
                                for hook in hooks_owned {
                                    {
                                        let hook_id = hook["webhook_id"].as_str().unwrap_or_default().to_string();
                                        let hook_name = hook["name"].as_str().unwrap_or_default().to_string();
                                        let wf_name = hook["workflow_name"].as_str().unwrap_or_default().to_string();
                                        let wf_ver = hook["version"].as_str().unwrap_or_default().to_string();
                                        
                                        rsx! {
                                            div { class: "card",
                                                div { style: "display: flex; justify-content: space-between; align-items: flex-start;",
                                                    div {
                                                        div { style: "font-weight: 600; font-size: 15px;", "{hook_name}" }
                                                        div { style: "font-size: 11px; color: var(--text-faint); margin-top: 4px;", "Triggers {wf_name} ({wf_ver})" }
                                                    }
                                                     button {
                                                        class: "btn btn-ghost",
                                                        style: "padding: 4px 8px; border: none; color: var(--status-error);",
                                                        aria_label: "Delete webhook {hook_name}",
                                                        onclick: move |_| {
                                                            let hid = hook_id.clone();
                                                            let hname = hook_name.clone();
                                                            let mut webhooks = webhooks.clone();
                                                            // Show confirmation dialog
                                                            if web_sys::window()
                                                                .unwrap()
                                                                .confirm_with_message(&format!("Are you sure you want to delete the webhook '{}'?

This action cannot be undone.", hname))
                                                                .unwrap_or(false)
                                                            {
                                                                spawn(async move {
                                                                    match api::delete_webhook(&hid).await {
                                                                        Ok(()) => {
                                                                            crate::app::show_toast(format!("Webhook '{}' deleted successfully", hname), crate::app::ToastType::Success);
                                                                        }
                                                                        Err(e) => {
                                                                            crate::app::show_toast(format!("Failed to delete webhook: {}", e), crate::app::ToastType::Error);
                                                                        }
                                                                    }
                                                                    webhooks.restart();
                                                                });
                                                            }
                                                        },
                                                        span { aria_hidden: "true", "✕" }
                                                    }
                                                }
                                                div { style: "margin-top: 16px; padding: 8px; background: var(--bg-secondary); border-radius: 4px;",
                                                    div { style: "font-size: 10px; font-weight: 600; color: var(--text-faint); margin-bottom: 4px;", "ENDPOINT" }
                                                    div { class: "mono", style: "font-size: 11px; overflow-x: auto;", "/api/v1/webhooks/{hook_id}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => rsx! { div { style: "padding: 20px; color: var(--text-faint); font-size: 13px;", "Loading webhooks..." } }
                    }
                }
                
                // Create New Webhook
                div { class: "card", style: "border: 1px dashed var(--border-strong);",
                    div { style: "font-weight: 600; font-size: 15px; margin-bottom: 12px;", "Register New" }
                    input { 
                        placeholder: "Webhook Name",
                        value: "{new_webhook_name}",
                        oninput: move |e| new_webhook_name.set(e.value())
                    }
                    input { 
                        placeholder: "Linked Workflow",
                        value: "{new_webhook_workflow}",
                        oninput: move |e| new_webhook_workflow.set(e.value())
                    }
                    button { 
                        class: "btn btn-primary", style: "width: 100%; height: 32px; font-size: 12px;",
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
                        "Add Webhook" 
                    }
                }
            }

            div { class: "section-title", "Service Connectors" }
            match &*integrations.read() {
                Some(Ok(services)) => rsx! {
                    div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 20px;",
                        for service in services {
                            {
                                let s = service.clone();
                                let service_name = s.name.clone();
                                let is_active = selected_service.read().as_str() == service_name.as_str();
                                rsx! {
                                    div {
                                        class: if is_active { "card active" } else { "card" },
                                        style: if is_active { "border-color: var(--accent-primary); background: #F0F7FF;" } else { "" },
                                        onclick: move |_| {
                                            selected_service.set(s.name.clone());
                                            status_msg.set(String::new());
                                            api_key.set(String::new());
                                        },
                                        div { style: "display: flex; align-items: center; gap: 12px;",
                                            div { style: "width: 32px; height: 32px; background: var(--bg-hover); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 18px;",
                                                if service_name.to_lowercase().contains("salesforce") { "☁" }
                                                else if service_name.to_lowercase().contains("hubspot") { "◎" }
                                                else if service_name.to_lowercase() == "email" { "✉" }
                                                else if service_name.to_lowercase() == "slack" { "💬" }
                                                else { "⇄" }
                                            }
                                            div { style: "font-weight: 500;", "{service_name}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                    _ => rsx! {
                        div { class: "card", style: "padding: 40px; text-align: center;",
                            div { class: "spinner", style: "margin: 0 auto 16px;" }
                            div { style: "color: var(--text-faint); font-size: 14px;", "Loading services..." }
                        }
                    }
            }

            if !selected_service.read().is_empty() {
                div { class: "section-title", "Configuration: {selected_service}" }
                div { class: "card", style: "max-width: 600px;",
                    div { class: "section-title", style: "border: none; padding: 0; margin-top: 0;", "Credentials" }
                    input { 
                        r#type: "password",
                        placeholder: "API Key / Token for {selected_service}",
                        value: "{api_key}",
                        oninput: move |evt| api_key.set(evt.value()),
                    }

                    if *selected_service.read() == "email" {
                        div { style: "margin-top: 24px; border-top: 1px solid var(--border-subtle); padding-top: 24px;",
                            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                                div { label { style: "font-size: 11px; font-weight: 600; color: var(--text-faint);", "SMTP Host" } input { value: "{smtp_host}", oninput: move |e| smtp_host.set(e.value()) } }
                                div { label { style: "font-size: 11px; font-weight: 600; color: var(--text-faint);", "SMTP Port" } input { value: "{smtp_port}", oninput: move |e| smtp_port.set(e.value()) } }
                                div { label { style: "font-size: 11px; font-weight: 600; color: var(--text-faint);", "IMAP Host" } input { value: "{imap_host}", oninput: move |e| imap_host.set(e.value()) } }
                                div { label { style: "font-size: 11px; font-weight: 600; color: var(--text-faint);", "IMAP Port" } input { value: "{imap_port}", oninput: move |e| imap_port.set(e.value()) } }
                                div { style: "grid-column: 1 / span 2;",
                                    label { style: "font-size: 11px; font-weight: 600; color: var(--text-faint);", "Mailbox User" }
                                    input { value: "{imap_user}", oninput: move |e| imap_user.set(e.value()) }
                                }
                            }
                            
                            // Email Mappings Configuration
                            div { style: "margin-top: 24px; border-top: 1px solid var(--border-subtle); padding-top: 24px;",
                                div { style: "font-size: 13px; font-weight: 600; margin-bottom: 12px;", "Email-to-Workflow Mappings" }
                                div { style: "display: flex; flex-direction: column; gap: 8px;",
                                    {
                                        let mappings = email_mappings.read().clone();
                                        rsx! {
                                            for (idx, (subject, workflow, version)) in mappings.iter().enumerate() {
                                                div { style: "display: grid; grid-template-columns: 1fr 1fr auto auto; gap: 8px; align-items: center;",
                                                    input { 
                                                        placeholder: "Subject pattern (e.g., 'leave approval')",
                                                        value: "{subject}",
                                                        oninput: move |e| {
                                                            let mut mappings = email_mappings.write();
                                                            if let Some((ref mut s, _, _)) = mappings.get_mut(idx) {
                                                                *s = e.value();
                                                            }
                                                        }
                                                    }
                                                    input { 
                                                        placeholder: "Workflow name",
                                                        value: "{workflow}",
                                                        oninput: move |e| {
                                                            let mut mappings = email_mappings.write();
                                                            if let Some((_, ref mut w, _)) = mappings.get_mut(idx) {
                                                                *w = e.value();
                                                            }
                                                        }
                                                    }
                                                    input { 
                                                        placeholder: "Version",
                                                        value: "{version}",
                                                        style: "width: 80px;",
                                                        oninput: move |e| {
                                                            let mut mappings = email_mappings.write();
                                                            if let Some((_, _, ref mut v)) = mappings.get_mut(idx) {
                                                                *v = e.value();
                                                            }
                                                        }
                                                    }
                                                    button {
                                                        class: "btn",
                                                        style: "padding: 4px 8px;",
                                                        onclick: move |_| {
                                                            let mut mappings = email_mappings.write();
                                                            if mappings.len() > 1 {
                                                                mappings.remove(idx);
                                                            }
                                                        },
                                                        "✕"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn",
                                        style: "margin-top: 8px;",
                                        onclick: move |_| {
                                            email_mappings.write().push((
                                                String::new(),
                                                String::new(),
                                                "v1.0".to_string()
                                            ));
                                        },
                                        "+ Add Mapping"
                                    }
                                }
                            }
                        }
                    }

                    if !status_msg.read().is_empty() {
                        div {
                            style: "margin-bottom: 16px; padding: 12px 16px; border-radius: 8px; font-size: 13px; font-weight: 500;",
                            class: if status_msg.read().contains("Error") { "badge badge-error" } else { "badge badge-success" },
                            "{status_msg}"
                        }
                    }

                    div { style: "display: flex; gap: 12px; margin-top: 24px;",
                        button {
                            class: "btn btn-primary",
                            style: "flex: 1;",
                            disabled: *is_loading.read() || api_key.read().is_empty(),
                            onclick: move |_| {
                                let service = selected_service.read().clone();
                                let key = api_key.read().clone();
                                let config = if service == "email" {
                                    serde_json::json!({
                                        "smtp_host": smtp_host.read().clone(),
                                        "smtp_port": smtp_port.read().parse::<u16>().unwrap_or(465),
                                        "imap_host": imap_host.read().clone(),
                                        "imap_port": imap_port.read().parse::<u16>().unwrap_or(993),
                                        "user": imap_user.read().clone(),
                                        "mappings": email_mappings.read().iter().map(|(subject, workflow, version)| {
                                            serde_json::json!({
                                                "subject": subject,
                                                "workflow": workflow,
                                                "version": version,
                                            })
                                        }).collect::<Vec<_>>(),
                                    })
                                } else {
                                    serde_json::json!({})
                                };
                                is_loading.set(true);
                                spawn(async move {
                                    match api::update_integration(&service, &key, config).await {
                                        Ok(_) => {
                                            status_msg.set("Configuration saved successfully.".into());
                                            crate::app::show_toast("Configuration saved successfully", crate::app::ToastType::Success);
                                        }
                                        Err(e) => {
                                            let err_msg = format!("Error: {}", e);
                                            status_msg.set(err_msg.clone());
                                            crate::app::show_toast(err_msg, crate::app::ToastType::Error);
                                        }
                                    }
                                    is_loading.set(false);
                                });
                            },
                            if *is_loading.read() { "Saving..." } else { "Save Configuration" }
                        }
                        button { class: "btn", onclick: move |_| selected_service.set(String::new()), "Cancel" }
                    }
                }
            }
        }
    }
}
