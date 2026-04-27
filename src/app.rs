//! Root application component and routing.

use dioxus::prelude::*;

use crate::components::documentation::Documentation;
use crate::components::execution_detail::ExecutionDetail;
use crate::components::home::{Home, AuthForm};
use crate::components::upload::Upload;
use crate::components::version_list::VersionList;
use crate::components::view_edit::ViewEdit;
use crate::components::view_gen_workflow::ViewGenWorkflow;
use crate::components::visualize::Visualize;
use crate::components::navbar::Navbar;

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(PublicLayout)]
    #[route("/")]
    Home {},
    #[route("/login")]
    AuthForm {},
    #[route("/about-us")]
    About {},

    #[layout(WorkspaceLayout)]
    #[route("/dashboard")]
    Dashboard {},
    #[route("/upload")]
    Upload {},
    #[route("/visualize")]
    Visualize {},
    #[route("/history")]
    History {},
    #[route("/ledger")]
    Ledger {},
    #[route("/docs")]
    Documentation {},
    #[route("/settings")]
    Settings {},
    #[route("/execution/:id")]
    ExecutionDetail { id: String },
    #[route("/workflow/:name/:version")]
    ViewGenWorkflow { name: String, version: String },
    #[route("/workflow/:name/:version/edit")]
    ViewEdit { name: String, version: String },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}

#[component]
fn PublicLayout() -> Element {
    rsx! {
        div { class: "app-container fade-in", style: "display: block; overflow-y: auto;",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn WorkspaceLayout() -> Element {
    let mut token = use_signal(crate::api::get_token);
    let mut user_email = use_signal(crate::api::get_user_email);
    let mut user_role = use_signal(crate::api::get_user_role);
    let nav = use_navigator();

    rsx! {
        div { class: "app-container fade-in",
            aside { class: "sidebar",
                div { style: "margin-bottom: 48px; display: flex; align-items: center; gap: 12px;",
                    img { 
                        src: asset!("/assets/image.png"), 
                        style: "width: 24px; height: 24px; object-fit: contain;" 
                    }
                    span { class: "brand-name", style: "font-size: 1.5rem; font-weight: 900; letter-spacing: -0.02em;", "RHEXIOM" }
                }

                nav { class: "nav-group",
                    Link { class: "nav-item", to: Route::Dashboard {}, "Console" }
                    Link { class: "nav-item", to: Route::Upload {}, "Studio" }
                }

                div { class: "nav-label", "Forensics" }
                nav { class: "nav-group",
                    Link { class: "nav-item", to: Route::Visualize {}, "Visualizer" }
                    Link { class: "nav-item", to: Route::History {}, "History" }
                    Link { class: "nav-item", to: Route::Ledger {}, "Ledger" }
                }

                div { class: "nav-label", "Support" }
                nav { class: "nav-group",
                    Link { class: "nav-item", to: Route::Documentation {}, "Docs" }
                    Link { class: "nav-item", to: Route::About {}, "About" }
                    Link { class: "nav-item", to: Route::Settings {}, "Settings" }
                }

                if let Some(email) = user_email.read().as_ref() {
                    div { style: "margin-top: auto; padding-top: 24px; border-top: 1px solid var(--border);",
                        div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 16px;",
                            div { 
                                style: "width: 24px; height: 24px; background: var(--text-primary); border-radius: 4px; color: white; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: 700;",
                                "{email.chars().next().unwrap_or('U')}"
                            }
                            span { style: "font-size: 13px; font-weight: 600; color: var(--text-secondary);", "{email}" }
                        }
                        button { 
                            class: "btn", 
                            style: "width: 100%; border: 1px solid var(--border); font-size: 12px; font-weight: 700; text-transform: uppercase;",
                            onclick: move |_| {
                                crate::api::logout();
                                token.set(None);
                                user_email.set(None);
                                user_role.set(None);
                                nav.push(Route::Home {});
                            },
                            "Sign out"
                        }
                    }
                }
            }

            main { class: "main-content",
                header { class: "app-header",
                    div { class: "breadcrumb",
                        span { "RHEXIOM" }
                        span { class: "breadcrumb-sep", "/" }
                        span { class: "breadcrumb-current", "WORKSPACE" }
                    }
                }

                div { class: "content-area fade-in",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn Dashboard() -> Element {
    let nav = use_navigator();
    let stats_res = use_resource(|| async move { crate::api::get_stats().await.ok() });
    let (active_workflows, total_executions, system_status) =
        if let Some(Some(stats)) = stats_res.read().as_ref() {
            (stats.active_workflows.to_string(), stats.total_executions.to_string(), stats.system_status.clone())
        } else {
            ("...".into(), "...".into(), "Stabilizing".into())
        };

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "CONSOLE" }
            
            div { style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 24px; margin-bottom: 48px;",
                div { class: "card",
                    div { class: "section-title", style: "margin-top: 0;", "STATUS" }
                    div { style: "font-size: 2.5rem; font-family: var(--font-display); color: var(--accent-primary);", "{system_status}" }
                }
                div { class: "card",
                    div { class: "section-title", style: "margin-top: 0;", "ACTIVE POLICIES" }
                    div { style: "font-size: 2.5rem; font-family: var(--font-display);", "{active_workflows}" }
                }
                div { class: "card",
                    div { class: "section-title", style: "margin-top: 0;", "EXECUTIONS" }
                    div { style: "font-size: 2.5rem; font-family: var(--font-display);", "{total_executions}" }
                }
            }

            div { class: "section-title", "LIVE STREAM" }
            div { class: "card", style: "padding: 0; overflow: hidden;",
                crate::components::activity_list::ActivityList { 
                    on_select: move |id| { nav.push(Route::ExecutionDetail { id }); } 
                }
            }
        }
    }
}

#[component]
fn History() -> Element {
    let nav = use_navigator();
    rsx! {
        div {
            h1 { class: "page-title", "HISTORY" }
            crate::components::activity_list::ActivityList { 
                on_select: move |id| { nav.push(Route::ExecutionDetail { id }); }
            }
        }
    }
}

#[component]
fn Ledger() -> Element {
    rsx! { VersionList {} }
}

#[component]
fn About() -> Element {
    rsx! { AboutPage {} }
}

#[component]
fn Settings() -> Element {
    let email = crate::api::get_user_email();
    let role = crate::api::get_user_role();
    rsx! { SettingsPage { user_email: email, user_role: role } }
}

#[component]
fn AboutPage() -> Element {
    rsx! {
        div { class: "fade-in",
            Navbar {}
            
            div { style: "max-width: 1000px; margin: 64px auto; padding: 0 40px;",
                div { style: "margin-bottom: 40px;",
                    Link { 
                        class: "btn", 
                        style: "border: none; padding: 0; color: var(--text-faint); font-size: 12px; font-weight: 700; text-transform: uppercase;",
                        to: Route::Home {},
                        "← Back to home"
                    }
                }

                h1 { class: "page-title", style: "font-size: 4rem;", "ABOUT RHEXIOM" }
                p { style: "font-size: 1.25rem; line-height: 1.6; margin-bottom: 48px; color: var(--text-secondary);", 
                    "Rhexiom is a post-sovereign policy operating system designed for deterministic execution and forensic auditability." 
                }
                
                div { class: "section-title", "OUR MISSION" }
                p { style: "font-size: 1rem; line-height: 1.75; margin-bottom: 32px;", 
                    "We believe that in an age of automated decisions, logic must be transparent, verifiable, and immutable. Rhexiom provides the tools to bridge the gap between human intent and machine execution." 
                }
                
                div { style: "display: flex; gap: 16px; margin-top: 48px; margin-bottom: 64px;",
                    Link { 
                        class: "btn btn-primary", 
                        style: "height: 48px; padding: 0 32px; font-size: 16px; display: flex; align-items: center;",
                        to: Route::AuthForm {},
                        "Get Started Now"
                    }
                }

                div { class: "section-title", "ARCHITECTURE" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                    div { class: "card",
                        h3 { style: "margin-bottom: 12px; font-size: 1.5rem;", "RHELANG" }
                        p { style: "font-size: 14px; color: var(--text-faint);", "A custom DSL designed for modeling complex policy logic as deterministic graphs." }
                    }
                    div { class: "card",
                        h3 { style: "margin-bottom: 12px; font-size: 1.5rem;", "WASM RUNTIME" }
                        p { style: "font-size: 14px; color: var(--text-faint);", "Policies are compiled into sandboxed WebAssembly for isolated and high-performance execution." }
                    }
                }
            }
        }
    }
}

#[component]
fn SettingsPage(user_email: Option<String>, user_role: Option<String>) -> Element {
    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "SETTINGS" }
            
            div { class: "section-title", "PROFILE" }
            div { class: "card",
                div { style: "display: flex; flex-direction: column; gap: 24px;",
                    div {
                        label { class: "form-label", "Email Address" }
                        div { style: "font-size: 1.1rem; font-weight: 600;", "{user_email.clone().unwrap_or_else(|| \"Not signed in\".into())}" }
                    }
                    div {
                        label { class: "form-label", "Assigned Role" }
                        div { style: "font-size: 1.1rem; font-weight: 600;", "{user_role.clone().unwrap_or_else(|| \"N/A\".into())}" }
                    }
                }
            }

            div { class: "section-title", "PREFERENCES" }
            div { class: "card",
                div { style: "display: flex; align-items: center; justify-content: space-between;",
                    div {
                        div { style: "font-weight: 700; font-size: 1.1rem;", "High-Precision Mode" }
                        div { style: "font-size: 13px; color: var(--text-faint);", "Enable forensic tracing by default for all executions." }
                    }
                    button { class: "btn", "Enabled" }
                }
            }
        }
    }
}
