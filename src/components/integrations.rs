use dioxus::prelude::*;
use crate::api;

#[component]
pub fn Integrations() -> Element {
    let integrations = use_resource(api::get_integrations);
    let mut selected_service = use_signal(|| String::new());
    let mut api_key = use_signal(|| String::new());
    let mut status_msg = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    rsx! {
        div { class: "dashboard-stack",
            section { class: "card",
                div { class: "card-header",
                    div {
                        div { class: "card-title", "External Connections" }
                        div { class: "card-description", "Securely link your policy engine to real-time data providers and operational services." }
                    }
                }

                match &*integrations.read() {
                    Some(Ok(services)) => rsx! {
                        div { class: "grid-3",
                            for service in services {
                                {
                                    let s = service.clone();
                                    let is_active = *selected_service.read() == *service;
                                    let icon = if s.to_lowercase().contains("salesforce") { "☁" } 
                                              else if s.to_lowercase().contains("hubspot") { "◎" }
                                              else { "⇄" };
                                    
                                    rsx! {
                                        div { 
                                            class: if is_active { "stat-card active" } else { "stat-card" },
                                            style: if is_active { "border-color: var(--accent);" } else { "" },
                                            onclick: move |_| {
                                                selected_service.set(s.clone());
                                                status_msg.set(String::new());
                                                api_key.set(String::new());
                                            },
                                            div { 
                                                style: "display: flex; align-items: center; gap: 12px;",
                                                div { class: "brand-mark", style: "background-color: var(--panel-lighter);", "{icon}" }
                                                div {
                                                    div { class: "stat-label", style: "margin: 0;", "Service" }
                                                    div { class: "stat-value", style: "font-size: 16px;", "{service}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! { div { class: "badge badge-danger", "Failed to load services: {e}" } },
                    None => rsx! { div { class: "status-message", "Connecting to integrations pool..." } }
                }
            }

            if !selected_service.read().is_empty() {
                section { class: "card",
                    div { class: "card-header",
                        div {
                            div { class: "card-title", "Configure {selected_service}" }
                            div { class: "card-description", "Provide credentials to enable this sink/source." }
                        }
                    }
                    
                    div { class: "form-group",
                        label { class: "form-label", "API Key / Infrastructure Token" }
                        input { 
                            r#type: "password",
                            placeholder: "Enter token for {selected_service}",
                            value: "{api_key}",
                            oninput: move |evt| api_key.set(evt.value()),
                            class: "form-input",
                            style: "max-width: 400px;"
                        }
                    }

                    if !status_msg.read().is_empty() {
                        div { class: "badge badge-info", style: "margin-bottom: 20px;", "{status_msg}" }
                    }

                    div { 
                        style: "display: flex; gap: 12px;",
                        button {
                            class: "btn btn-primary",
                            disabled: *is_loading.read() || api_key.read().is_empty(),
                            onclick: move |_| {
                                let service = selected_service.read().clone();
                                let key = api_key.read().clone();
                                is_loading.set(true);
                                
                                spawn(async move {
                                    match api::update_integration(&service, &key).await {
                                        Ok(_) => status_msg.set("Successfully updated!".to_string()),
                                        Err(e) => status_msg.set(format!("Error: {}", e))
                                    }
                                    is_loading.set(false);
                                });
                            },
                            if *is_loading.read() { "Verifying..." } else { "Update Credentials" }
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
