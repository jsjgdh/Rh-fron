use dioxus::prelude::*;
use crate::api::{login, signup, LoginRequest, SignupRequest, AuthResponse};
use crate::components::navbar::Navbar;
use crate::app::Route;

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    rsx! {
        div { class: "fade-in", style: "background: var(--bg);",
            Navbar {}
            
            main { 
                div { class: "section-hero",
                    span { class: "category-label", "AI POLICY ORCHESTRATION" }
                    
                    h1 { 
                        class: "page-title", 
                        style: "font-size: 80px; margin-bottom: 24px; max-width: 900px; margin-left: auto; margin-right: auto;", 
                        "THE PLATFORM FOR POLICY GOVERNANCE" 
                    }
                    
                    p { 
                        style: "font-size: 1.25rem; color: var(--text-secondary); margin: 0 auto 48px; line-height: 1.6; max-width: 800px; text-align: center;", 
                        "Orchestrate an army of policy agents from one dashboard. Execute entire workflows with deterministic precision and forensic safety."
                    }

                    div { style: "max-width: 500px; margin: 0 auto; display: flex; gap: 8px; align-items: center;",
                        div { 
                            style: "flex: 1; background: var(--surface); border: 1px solid var(--border-strong); border-radius: 12px; display: flex; align-items: center; padding: 4px 6px;",
                            input { 
                                style: "border: none; margin: 0; padding: 12px 16px; font-size: 15px; background: transparent;",
                                placeholder: "Your work email",
                            }
                            button { 
                                class: "btn btn-primary", 
                                style: "height: 44px; padding: 0 20px; font-size: 14px;",
                                onclick: move |_| { nav.push(Route::AuthForm {}); },
                                "Get started"
                            }
                        }
                    }

                    div { style: "display: flex; gap: 16px; justify-content: center; margin-top: 16px;",
                        Link { 
                            class: "nav-link", 
                            style: "font-size: 12px; color: var(--text-faint); text-decoration: underline;",
                            to: Route::About {},
                            "See how it works" 
                        }
                    }

                    // Product Visualization Mockup
                    div { class: "mockup-frame", style: "max-width: 1000px; margin-left: auto; margin-right: auto;",
                        div { class: "mockup-header",
                            div { class: "mockup-dot" }
                            div { class: "mockup-dot" }
                            div { class: "mockup-dot" }
                        }
                        img { 
                            src: asset!("/assets/mockup.png"),
                            style: "width: 100%; height: auto; display: block;"
                        }
                    }
                }

                // Trust Section
                div { style: "padding: 80px 40px; text-align: center; border-top: 1px solid var(--border); background: var(--surface);",
                    div { style: "color: var(--text-faint); font-size: 11px; font-weight: 800; text-transform: uppercase; letter-spacing: 0.15em; margin-bottom: 40px;",
                        "Powering deterministic logic for high-growth engineering teams"
                    }
                    div { style: "display: flex; gap: 64px; justify-content: center; align-items: center; opacity: 0.4; filter: grayscale(1);",
                        TrustLogo { name: "trigger.dev".to_string() }
                        TrustLogo { name: "perplexity".to_string() }
                        TrustLogo { name: "runpod".to_string() }
                        TrustLogo { name: "exa".to_string() }
                        TrustLogo { name: "Vercel".to_string() }
                        TrustLogo { name: "OpenAI".to_string() }
                    }
                }

                // How it Works Section
                div { style: "padding: 100px 40px; max-width: 1200px; margin: 0 auto;",
                    div { class: "section-title", style: "text-align: center; border: none;", "HOW RHEXIOM WORKS" }
                    h2 { style: "text-align: center; font-size: 3rem; margin-bottom: 64px;", "FROM INTENT TO EXECUTION" }
                    
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
                div { style: "padding: 100px 40px; background: #000; color: #fff;",
                    div { style: "max-width: 800px; margin: 0 auto; text-align: center;",
                        span { class: "category-label", style: "color: var(--accent-primary);", "THE RHEXIOM PROMISE" }
                        h2 { style: "font-size: 3.5rem; margin-bottom: 32px; color: #fff;", "YOUR DATA IS FORENSICALLY SAFE" }
                        p { style: "font-size: 1.25rem; line-height: 1.75; opacity: 0.8; margin-bottom: 48px;",
                            "We believe in preserving the sovereignty of your business logic. Rhexiom saves thousands of engineering hours by automating policy audits while ensuring your underlying IP and sensitive data remain completely invisible to third parties. Deterministic execution isn't just about speed—it's about absolute trust."
                        }
                        button { 
                            class: "btn btn-primary", 
                            style: "height: 56px; padding: 0 40px; font-size: 1.1rem;",
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
            h3 { style: "font-size: 1.5rem; margin-bottom: 12px;", "{title}" }
            p { style: "font-size: 14px; color: var(--text-secondary); line-height: 1.6;", "{desc}" }
        }
    }
}

#[component]
pub fn AuthForm() -> Element {
    let mut is_signup = use_signal(|| false);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut role = use_signal(|| "Policy Architect".to_string());
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);
    let nav = use_navigator();

    let handle_auth = move |_| {
        let e = email.read().clone();
        let p = password.read().clone();
        let mut n = nav.clone();
        spawn(async move {
            loading.set(true);
            error.set(None);
            if is_signup.read().clone() {
                match signup(&SignupRequest { email: e.clone(), password: p, role: Some(role.read().clone()) }).await {
                    Ok(res) if res.success => {
                        if let Some(t) = res.token { 
                            crate::api::set_token(&t);
                            if let Some(e) = res.email { crate::api::set_user_email(&e); }
                            if let Some(r) = res.role { crate::api::set_user_role(&r); }
                            n.push(Route::Dashboard {}); 
                        } 
                        else { is_signup.set(false); error.set(Some("Account created! Please login.".to_string())); }
                    }
                    Ok(res) => error.set(res.error),
                    Err(e) => error.set(Some(e)),
                }
            } else {
                match login(&LoginRequest { email: e, password: p }).await {
                    Ok(res) if res.success => { 
                        if let Some(t) = res.token { crate::api::set_token(&t); }
                        if let Some(e) = res.email { crate::api::set_user_email(&e); }
                        if let Some(r) = res.role { crate::api::set_user_role(&r); }
                        n.push(Route::Dashboard {}); 
                    }
                    Ok(res) => error.set(res.error),
                    Err(e) => error.set(Some(e)),
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "fade-in",
            Navbar {}
            
            div { style: "max-width: 420px; margin: 80px auto;",
                div { class: "card", style: "padding: 40px; border-radius: 16px; background: #fff;",
                    h2 { style: "font-size: 2rem; margin-bottom: 32px;", if *is_signup.read() { "CREATE ACCOUNT" } else { "SIGN IN" } }
                    
                    div { style: "display: flex; flex-direction: column; gap: 4px;",
                        label { class: "section-title", style: "border: none; padding: 0; font-size: 11px; margin: 0 0 8px;", "Work Email" }
                        input { 
                            r#type: "email", 
                            placeholder: "name@company.com", 
                            value: "{email}", 
                            oninput: move |e| email.set(e.value()) 
                        }
                    }

                    div { style: "display: flex; flex-direction: column; gap: 4px;",
                        label { class: "section-title", style: "border: none; padding: 0; font-size: 11px; margin: 0 0 8px;", "Password" }
                        input { 
                            r#type: "password", 
                            placeholder: "••••••••", 
                            value: "{password}", 
                            oninput: move |e| password.set(e.value()) 
                        }
                    }

                    if *is_signup.read() {
                        div { style: "display: flex; flex-direction: column; gap: 4px;",
                            label { class: "section-title", style: "border: none; padding: 0; font-size: 11px; margin: 0 0 8px;", "Select Role" }
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
                        div { style: "color: #ef4444; font-size: 12px; margin-bottom: 16px; font-weight: 500;", "Error: {err}" }
                    }

                    button { 
                        class: "btn btn-primary", 
                        style: "width: 100%; height: 44px; margin-top: 12px; font-weight: 700; font-size: 15px;",
                        disabled: *loading.read(),
                        onclick: handle_auth,
                        if *loading.read() { "PROCESSING..." } else { if *is_signup.read() { "START FREE TRIAL" } else { "ENTER WORKSPACE" } }
                    }

                    div { style: "margin-top: 24px; text-align: center; font-size: 13px; color: var(--text-faint); font-weight: 600;",
                        if *is_signup.read() {
                            "Already have an account? "
                            span { 
                                style: "color: var(--accent-primary); cursor: pointer; text-decoration: underline;", 
                                onclick: move |_| is_signup.set(false), 
                                "Sign In" 
                            }
                        } else {
                            "New to Rhexiom? "
                            span { 
                                style: "color: var(--accent-primary); cursor: pointer; text-decoration: underline;", 
                                onclick: move |_| is_signup.set(true), 
                                "Create an account" 
                            }
                        }
                    }
                }
            }
        }
    }
}
