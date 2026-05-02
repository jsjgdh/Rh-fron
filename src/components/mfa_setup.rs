use dioxus::prelude::*;
use crate::api::{mfa_setup, mfa_verify_setup, mfa_status, mfa_disable, MfaSetupResponse, MfaVerifySetupResponse};

#[component]
pub fn MfaSetup() -> Element {
    let mut setup_response = use_signal(|| Option::<MfaSetupResponse>::None);
    let mut verify_response = use_signal(|| Option::<MfaVerifySetupResponse>::None);
    let mut verification_code = use_signal(String::new);
    let error = use_signal(|| Option::<String>::None);
    let loading = use_signal(|| false);
    let mut mfa_enabled = use_signal(|| false);
    let mut show_disable = use_signal(|| false);
    let mut disable_code = use_signal(String::new);

    // Load MFA status on mount
    use_effect(move || {
        spawn(async move {
            match mfa_status().await {
                Ok(status) => {
                    mfa_enabled.set(status.enabled);
                }
                Err(_) => {}
            }
        });
    });

    let handle_start_setup = move |_| {
        let mut resp_signal = setup_response.clone();
        let mut err_signal = error.clone();
        let mut loading_signal = loading.clone();
        
        spawn(async move {
            loading_signal.set(true);
            err_signal.set(None);
            
            match mfa_setup().await {
                Ok(response) => {
                    if response.success {
                        resp_signal.set(Some(response));
                    } else {
                        err_signal.set(response.error);
                    }
                }
                Err(e) => {
                    err_signal.set(Some(e));
                }
            }
            loading_signal.set(false);
        });
    };

    let handle_verify = move |_| {
        let code = verification_code.read().clone();
        let mut verify_signal = verify_response.clone();
        let mut err_signal = error.clone();
        let mut loading_signal = loading.clone();
        let mut enabled_signal = mfa_enabled.clone();
        
        spawn(async move {
            loading_signal.set(true);
            err_signal.set(None);
            
            match mfa_verify_setup(&code).await {
                Ok(response) => {
                    if response.success {
                        verify_signal.set(Some(response));
                        enabled_signal.set(true);
                    } else {
                        err_signal.set(response.error);
                    }
                }
                Err(e) => {
                    err_signal.set(Some(e));
                }
            }
            loading_signal.set(false);
        });
    };

    let handle_disable = move |_| {
        let code = disable_code.read().clone();
        let mut enabled_signal = mfa_enabled.clone();
        let mut err_signal = error.clone();
        let mut loading_signal = loading.clone();
        let mut show_disable_signal = show_disable.clone();
        let mut setup_resp_signal = setup_response.clone();
        let mut verify_resp_signal = verify_response.clone();
        
        spawn(async move {
            loading_signal.set(true);
            err_signal.set(None);
            
            match mfa_disable(&code).await {
                Ok(response) => {
                    if response.success {
                        enabled_signal.set(false);
                        setup_resp_signal.set(None);
                        verify_resp_signal.set(None);
                        show_disable_signal.set(false);
                    } else {
                        err_signal.set(response.error);
                    }
                }
                Err(e) => {
                    err_signal.set(Some(e));
                }
            }
            loading_signal.set(false);
        });
    };

    rsx! {
        div { class: "fade-in",
            h3 { style: "font-size: 1.5rem; margin-bottom: 16px; font-weight: 800;", "Two-Factor Authentication" }
            
            div { class: "card",
                if *mfa_enabled.read() {
                    // MFA is enabled - show status and disable option
                    div {
                        div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 24px;",
                            div { style: "width: 12px; height: 12px; background: var(--status-success); border-radius: 50%;" }
                            div {
                                div { style: "font-weight: 700; font-size: 1.1rem;", "MFA is Enabled" }
                                div { style: "font-size: 13px; color: var(--text-faint);", "Your account is protected with TOTP authentication." }
                            }
                        }
                        
                        if !*show_disable.read() {
                            button {
                                class: "btn btn-danger",
                                style: "width: 100%; height: 48px;",
                                onclick: move |_| show_disable.set(true),
                                "Disable MFA"
                            }
                        } else {
                            div { style: "border: 1px solid var(--border); border-radius: 8px; padding: 16px; background: rgba(239, 68, 68, 0.05);",
                                p { style: "font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;",
                                    "To disable MFA, please enter your current TOTP code to confirm."
                                }
                                
                                div { class: "auth-input-group",
                                    input {
                                        r#type: "text",
                                        placeholder: "Enter TOTP code",
                                        maxlength: 6,
                                        value: "{disable_code}",
                                        oninput: move |e| disable_code.set(e.value())
                                    }
                                }
                                
                                if let Some(err) = error.read().as_ref() {
                                    div { style: "color: var(--status-error); font-size: 13px; margin-top: 8px;", "{err}" }
                                }
                                
                                div { style: "display: flex; gap: 12px; margin-top: 16px;",
                                    button {
                                        class: "btn",
                                        style: "flex: 1;",
                                        onclick: move |_| show_disable.set(false),
                                        "Cancel"
                                    }
                                    button {
                                        class: "btn btn-danger",
                                        style: "flex: 1;",
                                        disabled: *loading.read(),
                                        onclick: handle_disable,
                                        if *loading.read() { "DISABLING..." } else { "CONFIRM DISABLE" }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // MFA is not enabled
                    div {
                        if verify_response.read().is_some() {
                            // Show backup codes after successful setup
                            div {
                                div { style: "background: var(--status-success-bg); border: 1px solid var(--status-success); border-radius: 8px; padding: 16px; margin-bottom: 24px;",
                                    div { style: "font-weight: 700; color: var(--status-success); margin-bottom: 8px;", "MFA Enabled Successfully!" }
                                    div { style: "font-size: 14px; color: var(--text-secondary);",
                                        "Two-factor authentication is now active on your account."
                                    }
                                }
                                
                                if let Some(ref resp) = *verify_response.read() {
                                    if let Some(ref codes) = resp.backup_codes {
                                        div { style: "margin-bottom: 16px;",
                                            div { style: "font-weight: 700; margin-bottom: 12px;", "Backup Codes" }
                                            div { style: "font-size: 13px; color: var(--status-warning); margin-bottom: 12px;",
                                                "⚠️ Save these codes in a secure location. They can be used to recover access if you lose your authenticator device."
                                            }
                                            div { style: "background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 16px; display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 8px;",
                                                for code in codes {
                                                    div { style: "font-family: monospace; font-size: 14px; background: var(--bg-secondary); padding: 8px; border-radius: 4px; text-align: center;", "{code}" }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                button {
                                    class: "btn btn-primary",
                                    style: "width: 100%; height: 48px; margin-top: 16px;",
                                    onclick: move |_| {
                                        setup_response.set(None);
                                        verify_response.set(None);
                                    },
                                    "Done"
                                }
                            }
                        } else if let Some(ref setup) = *setup_response.read() {
                            // Show QR code and verification input
                            div {
                                div { style: "margin-bottom: 24px;",
                                    div { style: "font-weight: 700; margin-bottom: 12px;", "Step 1: Scan QR Code" }
                                    div { style: "font-size: 13px; color: var(--text-secondary); margin-bottom: 16px;",
                                        "Scan this QR code with your authenticator app (Google Authenticator, Authy, etc.)"
                                    }
                                    
                                    if let Some(ref svg) = setup.qr_code_svg {
                                        div { style: "background: white; padding: 16px; border-radius: 8px; display: inline-block; margin-bottom: 16px;",
                                            // Display SVG QR code
                                            div { 
                                                dangerous_inner_html: "{svg}",
                                                style: "width: 200px; height: 200px;"
                                            }
                                        }
                                    }
                                    
                                    if let Some(ref secret) = setup.secret {
                                        div { style: "margin-top: 16px; padding: 12px; background: var(--bg-secondary); border-radius: 8px;",
                                            div { style: "font-size: 12px; color: var(--text-faint); margin-bottom: 4px;", "Manual Entry Code" }
                                            div { style: "font-family: monospace; font-size: 14px; word-break: break-all;", "{secret}" }
                                        }
                                    }
                                }
                                
                                div { style: "margin-bottom: 24px;",
                                    div { style: "font-weight: 700; margin-bottom: 12px;", "Step 2: Verify Setup" }
                                    div { style: "font-size: 13px; color: var(--text-secondary); margin-bottom: 16px;",
                                        "Enter the 6-digit code from your authenticator app to complete setup."
                                    }
                                    
                                    div { class: "auth-input-group",
                                        input {
                                            r#type: "text",
                                            placeholder: "000000",
                                            maxlength: 6,
                                            value: "{verification_code}",
                                            oninput: move |e| verification_code.set(e.value())
                                        }
                                    }
                                    
                                    if let Some(err) = error.read().as_ref() {
                                        div { style: "color: var(--status-error); font-size: 13px; margin-top: 8px;", "{err}" }
                                    }
                                    
                                    div { style: "display: flex; gap: 12px; margin-top: 16px;",
                                        button {
                                            class: "btn",
                                            style: "flex: 1;",
                                            onclick: move |_| setup_response.set(None),
                                            "Cancel"
                                        }
                                        button {
                                            class: "btn btn-primary",
                                            style: "flex: 1;",
                                            disabled: *loading.read() || verification_code.read().len() != 6,
                                            onclick: handle_verify,
                                            if *loading.read() { "VERIFYING..." } else { "ENABLE MFA" }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Initial state - show enable button
                            div {
                                div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 24px;",
                                    div { style: "width: 12px; height: 12px; background: var(--status-warning); border-radius: 50%;" }
                                    div {
                                        div { style: "font-weight: 700; font-size: 1.1rem;", "MFA is Not Enabled" }
                                        div { style: "font-size: 13px; color: var(--text-faint);", "Enable two-factor authentication for enhanced security." }
                                    }
                                }
                                
                                div { style: "background: var(--bg-secondary); border: 1px solid var(--border); border-radius: 8px; padding: 16px; margin-bottom: 24px;",
                                    div { style: "font-weight: 700; margin-bottom: 12px;", "Why Enable MFA?" }
                                    ul { style: "font-size: 14px; color: var(--text-secondary); margin: 0; padding-left: 20px;",
                                        li { "Protects against password theft" }
                                        li { "Adds an extra layer of security" }
                                        li { "Required for sensitive operations" }
                                    }
                                }
                                
                                if let Some(err) = error.read().as_ref() {
                                    div { style: "color: var(--status-error); font-size: 13px; margin-bottom: 16px;", "{err}" }
                                }
                                
                                button {
                                    class: "btn btn-primary",
                                    style: "width: 100%; height: 48px;",
                                    disabled: *loading.read(),
                                    onclick: handle_start_setup,
                                    if *loading.read() { "GENERATING..." } else { "ENABLE MFA" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
