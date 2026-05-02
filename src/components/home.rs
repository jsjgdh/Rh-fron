use dioxus::prelude::*;
use crate::api::{login, signup, mfa_verify, mfa_verify_backup, LoginRequest, SignupRequest};
use crate::components::navbar::Navbar;
use crate::app::Route;
use crate::i18n::messages::{auth, labels, placeholders, format_message};

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    rsx! {
        div { class: "fade-in",
            Navbar {}
            
            main { 
                div { class: "section-hero",
                    span { class: "category-label", "AI POLICY ORCHESTRATION" }
                    
                    h1 { 
                        class: "page-title", 
                        style: "font-size: clamp(3rem, 8vw, 8rem); margin-bottom: 24px; max-width: 1000px; margin-left: auto; margin-right: auto;", 
                        "THE PLATFORM FOR POLICY GOVERNANCE" 
                    }
                    
                    p { 
                        style: "font-size: 1.5rem; color: var(--text-secondary); margin: 0 auto 64px; line-height: 1.5; max-width: 800px; text-align: center; font-weight: 500;", 
                        "Orchestrate an army of policy agents from one dashboard. Execute entire workflows with deterministic precision and forensic safety."
                    }

                    div { style: "max-width: 560px; margin: 0 auto; display: flex; gap: 12px; align-items: center;",
                        div { 
                            style: "flex: 1; background: var(--surface); border: 1px solid var(--border-strong); border-radius: 16px; display: flex; align-items: center; padding: 6px 8px; box-shadow: var(--shadow);",
                            input {
                                style: "border: none; margin: 0; padding: 14px 20px; font-size: 16px; background: transparent; width: 100%;",
                                placeholder: placeholders::EMAIL,
                            }
                            button { 
                                class: "btn btn-primary", 
                                style: "height: 48px; padding: 0 24px; font-size: 14px; border-radius: 12px;",
                                onclick: move |_| { nav.push(Route::AuthForm {}); },
                                "Get started"
                            }
                        }
                    }

                    // Product Visualization Mockup
                    div { class: "mockup-frame", style: "max-width: 1100px; margin-left: auto; margin-right: auto;",
                        div { class: "mockup-header",
                            div { class: "mockup-dot" }
                            div { class: "mockup-dot" }
                            div { class: "mockup-dot" }
                        }
                        img { 
                            src: asset!("/assets/image_copy.png"),
                            style: "width: 100%; height: auto; display: block;"
                        }
                    }
                }

                // How it Works Section
                div { style: "padding: 120px 40px; max-width: 1300px; margin: 0 auto;",
                    div { class: "section-title", style: "text-align: center; border: none; justify-content: center;", "HOW RHEXIOM WORKS" }
                    h2 { style: "text-align: center; font-size: 4rem; margin-bottom: 80px; font-weight: 900;", "FROM INTENT TO EXECUTION" }
                    
                    div { class: "how-it-works-strip",
                        StepItem { 
                            num: "01".to_string(), 
                            title: "MODEL".to_string(), 
                            desc: "Map your natural language policies into RheLang graphs. Define logic without the noise of underlying tech.".to_string() 
                        }
                        StepItem { 
                            num: "02".to_string(), 
                            title: "COMPILE".to_string(), 
                            desc: "Transform graphs into isolated, high-performance WASM artifacts. Built for forensic stability.".to_string() 
                        }
                        StepItem { 
                            num: "03".to_string(), 
                            title: "GOVERN".to_string(), 
                            desc: "Execute across any environment with character-level tracing. Full auditability with zero data exposure.".to_string() 
                        }
                    }
                }

                // Mission/Data Safety Section
                div { style: "padding: 120px 40px; background: var(--text-primary); color: var(--surface);",
                    div { style: "max-width: 900px; margin: 0 auto; text-align: center;",
                        span { class: "category-label", "THE RHEXIOM PROMISE" }
                        h2 { style: "font-size: 4rem; margin-bottom: 40px; color: var(--surface);", "YOUR DATA IS FORENSICALLY SAFE" }
                        p { style: "font-size: 1.4rem; line-height: 1.6; opacity: 0.9; margin-bottom: 56px; font-weight: 400;",
                            "We believe in preserving the sovereignty of your business logic. Rhexiom saves thousands of engineering hours by automating policy audits while ensuring your underlying IP and sensitive data remain completely invisible to third parties. Deterministic execution isn't just about speed—it's about absolute trust."
                        }
                        button { 
                            class: "btn btn-primary", 
                            style: "height: 64px; padding: 0 48px; font-size: 1.2rem; border-radius: 16px;",
                            onclick: move |_| { nav.push(Route::AuthForm {}); },
                            "Start securing your policies"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TrustLogo(name: String) -> Element {
    rsx! {
        span { class: "mono", style: "font-weight: 800; font-size: 16px; letter-spacing: -0.02em;", "{name}" }
    }
}

#[component]
fn StepItem(num: String, title: String, desc: String) -> Element {
    rsx! {
        div { class: "step-card",
            div { class: "step-number", "{num}" }
            h3 { style: "font-size: 1.8rem; margin-bottom: 16px; font-weight: 800;", "{title}" }
            p { style: "font-size: 15px; color: var(--text-secondary); line-height: 1.7; font-weight: 500;", "{desc}" }
        }
    }
}

#[component]
pub fn AuthForm() -> Element {
    let mut is_signup = use_signal(|| false);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut role = use_signal(|| "Policy Architect".to_string());
    let error = use_signal(|| Option::<String>::None);
    let loading = use_signal(|| false);
    let mut pending_mfa_user_id = use_signal(|| Option::<String>::None);
    let mut pending_mfa_email = use_signal(|| Option::<String>::None);
    let mut pending_mfa_role = use_signal(|| Option::<String>::None);
    let mut mfa_code = use_signal(String::new);
    let mut use_backup_code = use_signal(|| false);
    let nav = use_navigator();

    // Handle regular login/signup
    let handle_auth = move |_| {
        let e = email.read().clone();
        let p = password.read().clone();
        let n = nav.clone();
        let mut mfa_user_id = pending_mfa_user_id.clone();
        let mut mfa_email = pending_mfa_email.clone();
        let mut mfa_role = pending_mfa_role.clone();
        let mut err_signal = error.clone();
        let mut loading_signal = loading.clone();
        
        spawn(async move {
            loading_signal.set(true);
            err_signal.set(None);
            if *is_signup.read() {
                match signup(&SignupRequest { email: e.clone(), password: p, role: Some(role.read().clone()) }).await {
                    Ok(res) if res.success => {
                        if let Some(t) = res.token { 
                            crate::api::set_token(&t);
                            if let Some(e) = res.email { crate::api::set_user_email(&e); }
                            if let Some(r) = res.role { crate::api::set_user_role(&r); }
                            n.push(Route::Dashboard {}); 
                        } 
                        else { is_signup.set(false); err_signal.set(Some(format_message(&auth::SIGNUP_SUCCESS))); }
                    }
                    Ok(res) => err_signal.set(res.error),
                    Err(e) => err_signal.set(Some(e)),
                }
            } else {
                match login(&LoginRequest { email: e, password: p }).await {
                    Ok(res) if res.success && res.mfa_required == Some(true) => {
                        // MFA required - store user info and show MFA input
                        if let Some(uid) = res.user_id {
                            mfa_user_id.set(Some(uid));
                        }
                        if let Some(e) = res.email {
                            mfa_email.set(Some(e));
                        }
                        if let Some(r) = res.role {
                            mfa_role.set(Some(r));
                        }
                    }
                    Ok(res) if res.success => { 
                        if let Some(t) = res.token { crate::api::set_token(&t); }
                        if let Some(e) = res.email { crate::api::set_user_email(&e); }
                        if let Some(r) = res.role { crate::api::set_user_role(&r); }
                        n.push(Route::Dashboard {}); 
                    }
                    Ok(res) => err_signal.set(res.error),
                    Err(e) => err_signal.set(Some(e)),
                }
            }
            loading_signal.set(false);
        });
    };

    // Handle MFA verification
    let handle_mfa_verify = move |_| {
        let code = mfa_code.read().clone();
        let uid = pending_mfa_user_id.read().clone();
        let email = pending_mfa_email.read().clone();
        let role = pending_mfa_role.read().clone();
        let backup = *use_backup_code.read();
        let n = nav.clone();
        let mut err_signal = error.clone();
        let mut loading_signal = loading.clone();
        
        spawn(async move {
            loading_signal.set(true);
            err_signal.set(None);
            
            if let Some(user_id) = uid {
                let result = if backup {
                    mfa_verify_backup(&user_id, &code).await
                } else {
                    mfa_verify(&user_id, &code).await
                };
                
                match result {
                    Ok(res) if res.success => {
                        if let Some(t) = res.token {
                            crate::api::set_token(&t);
                            if let Some(e) = email { crate::api::set_user_email(&e); }
                            if let Some(r) = role { crate::api::set_user_role(&r); }
                            n.push(Route::Dashboard {});
                        }
                    }
                    Ok(res) => err_signal.set(res.error),
                    Err(e) => err_signal.set(Some(e)),
                }
            }
            loading_signal.set(false);
        });
    };

    rsx! {
        div { class: "fade-in",
            Navbar {}
            
            div { class: "auth-card",
                // Show MFA verification if pending
                if pending_mfa_user_id.read().is_some() {
                    h2 { style: "font-size: 2rem; margin-bottom: 24px; font-weight: 900;", "TWO-FACTOR AUTHENTICATION" }
                    
                    p { style: "color: var(--text-secondary); font-size: 14px; margin-bottom: 24px;",
                        "Enter the 6-digit code from your authenticator app."
                    }
                    
                    div { class: "auth-input-group",
                        label { class: "nav-label", style: "padding: 0; margin-bottom: 8px;",
                            if *use_backup_code.read() { "Backup Code" } else { "Authentication Code" }
                        }
                        input { 
                            r#type: "text",
                            placeholder: if *use_backup_code.read() { "xxxxxxxx" } else { "000000" },
                            value: "{mfa_code}",
                            maxlength: if *use_backup_code.read() { 8 } else { 6 },
                            oninput: move |e| mfa_code.set(e.value())
                        }
                    }
                    
                    if let Some(err) = error.read().as_ref() {
                        div { style: "color: var(--status-error); font-size: 13px; margin-bottom: 16px; font-weight: 600;", "{err}" }
                    }
                    
                    button {
                        class: "btn btn-primary",
                        style: "width: 100%; height: 48px; margin-top: 8px; font-size: 14px;",
                        disabled: *loading.read(),
                        onclick: handle_mfa_verify,
                        if *loading.read() {
                            span { class: "spinner spinner-sm", style: "margin-right: 8px;" }
                            "VERIFYING..."
                        } else { "VERIFY" }
                    }
                    
                            div { style: "margin-top: 16px; text-align: center;",
                                button {
                                    class: "btn btn-ghost",
                                    style: "font-size: 13px; color: var(--accent-primary); text-decoration: underline;",
                                    onclick: move |_| {
                                        let current = *use_backup_code.read();
                                        use_backup_code.set(!current);
                                        mfa_code.set(String::new());
                                    },
                                    if *use_backup_code.read() { "Use authenticator code" } else { "Use backup code" }
                                }
                            }
                    
                    div { style: "margin-top: 16px; text-align: center;",
                        button {
                            class: "btn btn-ghost",
                            style: "font-size: 13px; color: var(--text-faint);",
                            onclick: move |_| {
                                pending_mfa_user_id.set(None);
                                pending_mfa_email.set(None);
                                pending_mfa_role.set(None);
                                mfa_code.set(String::new());
                                use_backup_code.set(false);
                            },
                            "← Back to login"
                        }
                    }
                } else {
                    // Regular auth form
                    h2 { style: "font-size: 2.5rem; margin-bottom: 40px; font-weight: 900;", if *is_signup.read() { "CREATE ACCOUNT" } else { "SIGN IN" } }
                    
                    div { class: "auth-input-group",
                        label { class: "nav-label", style: "padding: 0; margin-bottom: 8px;", "Work Email" }
                        input {
                            r#type: "email",
                            placeholder: "{placeholders::EMAIL}",
                            value: "{email}",
                            aria_required: "true",
                            aria_invalid: "false",
                            aria_describedby: "email-help",
                            oninput: move |e| email.set(e.value())
                        }
                        span { id: "email-help", style: "font-size: 11px; color: var(--text-faint);", "Enter your work email address" }
                    }

                    div { class: "auth-input-group",
                        label { class: "nav-label", style: "padding: 0; margin-bottom: 8px;", "Password" }
                        input {
                            r#type: "password",
                            placeholder: "{placeholders::PASSWORD}",
                            value: "{password}",
                            aria_required: "true",
                            aria_invalid: "false",
                            aria_describedby: "password-help",
                            oninput: move |e| password.set(e.value())
                        }
                        span { id: "password-help", style: "font-size: 11px; color: var(--text-faint);", if *is_signup.read() { "Use 8+ chars with uppercase, lowercase, digit, and special character" } else { "Enter your password" } }
                    }

                    if *is_signup.read() {
                        div { class: "auth-input-group",
                            label { class: "nav-label", style: "padding: 0; margin-bottom: 8px;", "Select Role" }
                            select {
                                value: "{role}",
                                onchange: move |e| role.set(e.value()),
                                option { value: "Policy Architect", "Policy Architect" }
                                option { value: "Operator", "Operator" }
                                option { value: "Auditor", "Auditor" }
                                option { value: "System Administrator", "Administrator" }
                            }
                        }
                    }

                    if let Some(err) = error.read().as_ref() {
                        div { style: "color: var(--status-error); font-size: 13px; margin-bottom: 24px; font-weight: 600;", "Error: {err}" }
                    }

                        button {
                        class: "btn btn-primary",
                        style: "width: 100%; height: 52px; margin-top: 16px; font-size: 16px;",
                        disabled: *loading.read(),
                        onclick: handle_auth,
                        if *loading.read() {
                            span { class: "spinner spinner-sm", style: "margin-right: 8px;" }
                            "Processing..."
                        } else {
                            if *is_signup.read() { "{labels::CREATE_ACCOUNT}" } else { "{labels::SIGN_IN}" }
                        }
                    }

                    div { style: "margin-top: 32px; text-align: center; font-size: 14px; color: var(--text-faint); font-weight: 600;",
                        if *is_signup.read() {
                            "Already have an account? "
                            span {
                                style: "color: var(--accent-primary); cursor: pointer; text-decoration: underline;",
                                onclick: move |_| is_signup.set(false),
                                {labels::SIGN_IN}
                            }
                        } else {
                            "New to Rhexiom? "
                            span {
                                style: "color: var(--accent-primary); cursor: pointer; text-decoration: underline;",
                                onclick: move |_| is_signup.set(true),
                                {labels::CREATE_ACCOUNT}
                            }
                        }
                    }
                }
            }
        }
    }
}
