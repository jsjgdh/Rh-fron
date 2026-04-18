use dioxus::prelude::*;
use crate::api::{login, signup, LoginRequest, SignupRequest, AuthResponse};

#[component]
pub fn Home(#[props(default)] on_login: EventHandler<AuthResponse>) -> Element {
    let mut show_auth = use_signal(|| false);
    let mut is_signup = use_signal(|| false);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let handle_auth = move |_| {
        let e = email.read().clone();
        let p = password.read().clone();
        let nav_login = on_login.clone();

        spawn(async move {
            loading.set(true);
            error.set(None);

            if is_signup.read().clone() {
                match signup(&SignupRequest { email: e.clone(), password: p }).await {
                    Ok(res) if res.success => {
                        if res.token.is_some() {
                            nav_login.call(res);
                        } else {
                            is_signup.set(false);
                            error.set(Some("Account created! Please login.".to_string()));
                        }
                    }
                    Ok(res) => error.set(res.error),
                    Err(e) => error.set(Some(e)),
                }
            } else {
                match login(&LoginRequest { email: e, password: p }).await {
                    Ok(res) if res.success => {
                        nav_login.call(res);
                    }
                    Ok(res) => error.set(res.error),
                    Err(e) => error.set(Some(e)),
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "landing-page",
            // ── Navigation ──────────────────────────────────────────
            nav { class: "landing-nav",
                div { class: "brand-lockup",
                    div { class: "brand-mark", "R" }
                    div {
                        div { class: "brand-name", "Rhexiom" }
                        div { class: "brand-caption", "Policy OS" }
                    }
                }

                div { class: "landing-links",
                    a { href: "#capabilities", "Capabilities" }
                    a { href: "#pipeline", "Pipeline" }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            is_signup.set(false);
                            show_auth.set(true);
                        },
                        "Enter workspace"
                    }
                }
            }

            // ── Main Content ────────────────────────────────────────
            main { class: "landing-main",
                section { class: "landing-hero",
                    div { class: "landing-copy",
                        div { class: "section-kicker", "Text to Deterministic Logic" }
                        h1 { class: "landing-title",
                            "Model business policies with precision."
                        }
                        p { class: "landing-subtitle",
                            "Transform natural language into structured RheLang workflows. Compile, execute, and trace every decision in one continuous pipeline."
                        }
                        div { class: "landing-chip-row",
                            span { class: "header-badge", "AI-Assisted" }
                            span { class: "header-badge", "Immutable" }
                            span { class: "header-badge", "Traceable" }
                        }
                        div { class: "hero-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    is_signup.set(true);
                                    show_auth.set(true);
                                },
                                "Start modeling"
                            }
                            a { class: "btn btn-secondary", href: "#capabilities", "Learn more" }
                        }
                    }

                    div { class: "landing-panel",
                        if *show_auth.read() {
                            div { class: "auth-overlay card glassmorphic",
                                div { 
                                    style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 32px;",
                                    div {
                                        div { class: "card-title", if *is_signup.read() { "Create Account" } else { "Welcome Back" } }
                                        div { class: "card-description", if *is_signup.read() { "Join the Rhexiom network." } else { "Sign in to your workspace." } }
                                    }
                                    button { class: "btn-close", onclick: move |_| show_auth.set(false), "×" }
                                }
                                
                                div { class: "auth-form",
                                    div { class: "form-group",
                                        label { class: "form-label", "Email" }
                                        input {
                                            class: "form-input",
                                            r#type: "email",
                                            placeholder: "you@company.com",
                                            value: "{email}",
                                            oninput: move |e| email.set(e.value())
                                        }
                                    }
                                    div { class: "form-group",
                                        label { class: "form-label", "Password" }
                                        input {
                                            class: "form-input",
                                            r#type: "password",
                                            placeholder: "••••••••",
                                            value: "{password}",
                                            oninput: move |e| password.set(e.value())
                                        }
                                    }

                                    if let Some(err) = error.read().as_ref() {
                                        div { class: "auth-error", "{err}" }
                                    }

                                    button {
                                        class: "btn btn-primary btn-full",
                                        style: "margin-top: 12px;",
                                        disabled: *loading.read(),
                                        onclick: handle_auth,
                                        if *loading.read() { "Processing..." } else { if *is_signup.read() { "Create account" } else { "Sign in" } }
                                    }

                                    div { class: "auth-switch",
                                        if *is_signup.read() {
                                            "Already have an account? "
                                            button { class: "btn-link", onclick: move |_| is_signup.set(false), "Sign in" }
                                        } else {
                                            "New to Rhexiom? "
                                            button { class: "btn-link", onclick: move |_| is_signup.set(true), "Create account" }
                                        }
                                    }
                                }
                            }
                        } else {
                            div { class: "landing-viz",
                                div { class: "signal-card signal-card-accent" ,
                                    div { class: "signal-label", "Architectural Posture" }
                                    div { class: "signal-value", "Logic-Transparent" }
                                    p { class: "signal-copy",
                                        "Every step is traceable. Every decision is grounded in the compiled AST."
                                    }
                                }

                                div { class: "signal-card-grid",
                                    div { class: "signal-card",
                                        div { class: "signal-label", "Mode" }
                                        div { class: "signal-value signal-value-small", "Deterministic" }
                                    }
                                    div { class: "signal-card",
                                        div { class: "signal-label", "Storage" }
                                        div { class: "signal-value signal-value-small", "Content-Addressed" }
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Capabilities ────────────────────────────────────────
                section { class: "feature-grid", id: "capabilities",
                    article { class: "feature-card" ,
                        div { class: "feature-index", "01" }
                        h2 { "High-Density Modeling" }
                        p { "Author complex policies using a technical workspace designed for logic transparency and forensic auditability." }
                    }

                    article { class: "feature-card" ,
                        div { class: "feature-index", "02" }
                        h2 { "Validated Pipelines" }
                        p { "Compiler-checked workflows that ensure every graph path is reachable and every input is schema-valid." }
                    }

                    article { class: "feature-card" ,
                        div { class: "feature-index", "03" }
                        h2 { "Forensic Trace" }
                        p { "Replay the exact path of any execution to understand why a decision was reached by the runtime." }
                    }
                }

                // ── Pipeline Strip ──────────────────────────────────────
                section { class: "process-strip", id: "pipeline",
                    div { class: "process-item",
                        div { class: "process-step", "01 / Author" }
                        h3 { "Draft policies" }
                        p { "Natural language or RheLang source." }
                    }
                    div { class: "process-item",
                        div { class: "process-step", "02 / Compile" }
                        h3 { "Build artifacts" }
                        p { "Validated AST and IR generation." }
                    }
                    div { class: "process-item",
                        div { class: "process-step", "03 / Execute" }
                        h3 { "Run logic" }
                        p { "Deterministic sandboxed execution." }
                    }
                    div { class: "process-item",
                        div { class: "process-step", "04 / Audit" }
                        h3 { "Verify path" }
                        p { "Full forensic trace replay." }
                    }
                }
            }
        }
    }
}
