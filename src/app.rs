//! Root application component and routing.

use dioxus::prelude::*;

use crate::auth::{can_access_route, has_permission, Permission};
use crate::components::documentation::Documentation;
use crate::components::execution_detail::ExecutionDetail;
use crate::components::home::{Home, AuthForm};
use crate::components::integrations::Integrations;
use crate::components::upload::Upload;
use crate::components::version_list::VersionList;
use crate::components::view_edit::ViewEdit;
use crate::components::view_gen_workflow::ViewGenWorkflow;
use crate::components::visualize::Visualize;
use crate::components::view_diff::ViewDiff;
use crate::components::navbar::Navbar;


#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(PublicLayout)]
    #[route("/")]
    Home {},
    #[route("/login")]
    AuthForm {},
    #[route("/about")]
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
    #[route("/integrations")]
    Integrations {},
    #[route("/execution/:id")]
    ExecutionDetail { id: String },
    #[route("/workflow/:name/:version")]
    ViewGenWorkflow { name: String, version: String },
    #[route("/workflow/:name/:version/edit")]
    ViewEdit { name: String, version: String },
    #[route("/workflow/:name/:v1/diff/:v2")]
    ViewDiff { name: String, v1: String, v2: String },
    #[route("/access-denied")]
    AccessDenied {},
}

/// Toast notification state - shared across app
#[derive(Clone)]
pub struct ToastMessage {
    pub id: u64,
    pub message: String,
    pub toast_type: ToastType,
}

#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

/// Global toast signal for app-wide notifications
pub static TOASTS: GlobalSignal<Vec<ToastMessage>> = Global::new(|| Vec::new());

/// Show a toast notification
pub fn show_toast(message: impl Into<String>, toast_type: ToastType) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let current = TOASTS.cloned();
    let mut writer = TOASTS.write();
    *writer = [current, vec![ToastMessage { id, message: message.into(), toast_type }]].concat();
}

/// Remove a toast by id
pub fn remove_toast(id: u64) {
    let mut writer = TOASTS.write();
    *writer = writer.clone().into_iter().filter(|t| t.id != id).collect();
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        crate::components::animated_bg::AnimatedBackground {}
        Router::<Route> {}
        ToastContainer {}
    }
}

/// Toast notification container
#[component]
fn ToastContainer() -> Element {
    let toasts = TOASTS.read();

    // Auto-dismiss toasts after 5 seconds
    use_future(move || async move {
        loop {
            #[cfg(target_arch = "wasm32")]
            {
                wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 5000)
                        .unwrap();
                }))
                .await
                .unwrap();
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
            let toasts = TOASTS.read().clone();
            if !toasts.is_empty() {
                // Remove the oldest toast
                let oldest = toasts.iter().min_by_key(|t| t.id);
                if let Some(oldest) = oldest {
                    remove_toast(oldest.id);
                }
            }
        }
    });

    rsx! {
        div {
            class: "toast-container",
            aria_live: "polite",
            aria_atomic: "true",
            for toast in toasts.iter() {
                Toast {
                    key: "{toast.id}",
                    id: toast.id,
                    message: toast.message.clone(),
                    toast_type: toast.toast_type.clone()
                }
            }
        }
    }
}

#[component]
fn Toast(id: u64, message: String, toast_type: ToastType) -> Element {
    let toast_class = match toast_type {
        ToastType::Success => "toast toast-success",
        ToastType::Error => "toast toast-error",
        ToastType::Warning => "toast toast-warning",
        ToastType::Info => "toast toast-info",
    };

    let icon = match toast_type {
        ToastType::Success => "✓",
        ToastType::Error => "✕",
        ToastType::Warning => "⚠",
        ToastType::Info => "ℹ",
    };

    rsx! {
        div {
            class: "{toast_class}",
            role: "alert",
            span { aria_hidden: "true", "{icon}" }
            span { "{message}" }
            button {
                class: "btn btn-ghost",
                style: "padding: 4px 8px; margin-left: auto; font-size: 14px;",
                aria_label: "Close notification",
                onclick: move |_| remove_toast(id),
                "✕"
            }
        }
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

/// Navigation item definition for sidebar
#[derive(Clone)]
struct SidebarNavItem {
    label: &'static str,
    route: Route,
    permission: Permission,
    icon: Option<&'static str>,
}

impl SidebarNavItem {
    fn new(label: &'static str, route: Route, permission: Permission) -> Self {
        Self {
            label,
            route,
            permission,
            icon: None,
        }
    }

    fn with_icon(label: &'static str, route: Route, permission: Permission, icon: &'static str) -> Self {
        Self {
            label,
            route,
            permission,
            icon: Some(icon),
        }
    }
}

/// Navigation group definition
#[derive(Clone)]
struct NavGroup {
    label: &'static str,
    items: Vec<SidebarNavItem>,
}

#[component]
fn WorkspaceLayout() -> Element {
    let mut token = use_signal(crate::api::get_token);
    let mut user_email = use_signal(crate::api::get_user_email);
    let mut user_role = use_signal(crate::api::get_user_role);
    let mut is_dark = use_signal(|| false);
    let mut sidebar_open = use_signal(|| false);
    let nav = use_navigator();
    let current_route = use_route::<Route>();

    // Check authentication
    if token.read().is_none() {
        nav.replace(Route::AuthForm {});
        return rsx! {
            div { class: "fade-in", style: "display: flex; align-items: center; justify-content: center; height: 100vh;",
                "Redirecting..."
            }
        };
    }

    let role_str = user_role.read().clone();
    let role_ref = role_str.as_deref();

    // Define navigation structure with permissions
    let nav_groups = vec![
        NavGroup {
            label: "Console",
            items: vec![
                SidebarNavItem::new("Console", Route::Dashboard {}, Permission::DashboardView),
            ],
        },
        NavGroup {
            label: "Studio",
            items: vec![
                SidebarNavItem::new("Studio", Route::Upload {}, Permission::WorkflowsCreate),
            ],
        },
        NavGroup {
            label: "Forensics",
            items: vec![
                SidebarNavItem::new("Visualizer", Route::Visualize {}, Permission::ExecutionsView),
                SidebarNavItem::new("History", Route::History {}, Permission::ExecutionsView),
                SidebarNavItem::new("Ledger", Route::Ledger {}, Permission::WorkflowsView),
            ],
        },
        NavGroup {
            label: "Management",
            items: vec![
                SidebarNavItem::new("Integrations", Route::Integrations {}, Permission::IntegrationsView),
                SidebarNavItem::new("Settings", Route::Settings {}, Permission::SettingsView),
            ],
        },
        NavGroup {
            label: "Support",
            items: vec![
                SidebarNavItem::new("Docs", Route::Documentation {}, Permission::DashboardView),
            ],
        },
    ];

    // Filter navigation based on permissions
    let filtered_groups: Vec<NavGroup> = nav_groups
        .into_iter()
        .map(|mut group| {
            group.items.retain(|item| has_permission(role_ref, &item.permission));
            group
        })
        .filter(|group| !group.items.is_empty())
        .collect();

    // Check if current route is accessible
    let current_path = match &current_route {
        Route::Dashboard {} => "/dashboard",
        Route::Upload {} => "/upload",
        Route::Visualize {} => "/visualize",
        Route::History {} => "/history",
        Route::Ledger {} => "/ledger",
        Route::Documentation {} => "/docs",
        Route::Settings {} => "/settings",
        Route::Integrations {} => "/integrations",
        Route::ExecutionDetail { .. } => "/execution",
        Route::ViewGenWorkflow { .. } => "/workflow",
        Route::ViewEdit { .. } => "/workflow/edit",
        Route::ViewDiff { .. } => "/workflow/diff",
        _ => "",
    };

    // If user can't access current route, redirect to dashboard or access denied
    if !current_path.is_empty() && !can_access_route(role_ref, current_path) {
        nav.replace(Route::AccessDenied {});
        return rsx! {
            div { class: "fade-in", style: "display: flex; align-items: center; justify-content: center; height: 100vh;",
                "Checking permissions..."
            }
        };
    }

    let sidebar_open_signal = *sidebar_open.read();

    rsx! {
        div {
            class: "app-container fade-in",
            "data-theme": if *is_dark.read() { "dark" } else { "light" },

            // Skip to content link for accessibility
            a {
                class: "skip-to-content",
                href: "#main-content",
                "Skip to main content"
            }

            // Mobile menu overlay
            if *sidebar_open.read() {
                div {
                    class: "modal-overlay",
                    style: "display: none;",
                    onclick: move |_| sidebar_open.set(false),
                }
            }

            aside {
                class: if *sidebar_open.read() { "sidebar open" } else { "sidebar" },
                div { style: "margin-bottom: 48px; display: flex; align-items: center; justify-content: space-between;",
                    div { style: "display: flex; align-items: center; gap: 12px;",
                        img { 
                            src: asset!("/assets/image.png"), 
                            style: "width: 28px; height: 28px; object-fit: contain;" 
                        }
                        span { class: "brand-name", style: "font-size: 1.6rem; font-weight: 900; letter-spacing: -0.02em;", "RHEXIOM" }
                    }
                    button { 
                        class: "btn btn-ghost", 
                        style: "padding: 6px; width: 32px; height: 32px; border-radius: 6px;",
                        onclick: move |_| {
                            let next = !*is_dark.read();
                            is_dark.set(next);
                        },
                        if *is_dark.read() { "☼" } else { "☾" }
                    }
                }

                // Render filtered navigation groups
                for group in filtered_groups.iter() {
                    div { class: "nav-label", "{group.label}" }
                    nav { class: "nav-group",
                        for item in group.items.iter() {
                            Link {
                                class: "nav-item",
                                to: item.route.clone(),
                                "{item.label}"
                            }
                        }
                    }
                }

                if let Some(email) = user_email.read().as_ref() {
                    div { style: "margin-top: auto; padding-top: 24px; border-top: 1px solid var(--border);",
                        div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 16px;",
                            div { 
                                style: "width: 32px; height: 32px; background: var(--accent-primary); border-radius: 8px; color: black; display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 800;",
                                "{email.chars().next().unwrap_or('U')}"
                            }
                            div {
                                div { style: "font-size: 13px; font-weight: 700; color: var(--text-primary);", "{email}" }
                                div { style: "font-size: 11px; font-weight: 600; color: var(--accent-primary);", "{role_ref.unwrap_or(\"GUEST\").to_uppercase()}" }
                            }
                        }
                        button { 
                            class: "btn btn-danger", 
                            style: "width: 100%; border-radius: 8px; font-size: 12px; font-weight: 700;",
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

            main {
                class: "main-content",
                id: "main-content",
                header {
                    class: "app-header",
                    // Mobile menu toggle button
                    button {
                        class: "btn btn-ghost mobile-menu-toggle",
                        style: "display: none; margin-right: 12px;",
                        aria_label: "Toggle navigation menu",
                        aria_expanded: "{sidebar_open_signal}",
                        onclick: move |_| sidebar_open.set(!sidebar_open_signal),
                        "☰"
                    }
                    div { class: "breadcrumb",
                        span { "RHEXIOM" }
                        span { class: "breadcrumb-sep", "→" }
                        span { class: "breadcrumb-current", "SECURE_WORKSPACE" }
                    }
                    div { style: "display: flex; gap: 16px;",
                        div {
                            class: "badge badge-success",
                            role: "status",
                            aria_live: "polite",
                            "System Operational"
                        }
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

    let user_role = crate::api::get_user_role();
    let role_ref = user_role.as_deref();
    let _can_run = has_permission(role_ref, &Permission::ExecutionsRun);

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "CONSOLE" }
            
            div { class: "grid-4", style: "margin-bottom: 48px;",
                div { class: "card stat-card",
                    div { class: "stat-label", "System Health" }
                    div { class: "stat-value", style: "color: var(--status-success);", "{system_status}" }
                }
                div { class: "card stat-card",
                    div { class: "stat-label", "Active Policies" }
                    div { class: "stat-value", "{active_workflows}" }
                }
                div { class: "card stat-card",
                    div { class: "stat-label", "Total Runs" }
                    div { class: "stat-value", "{total_executions}" }
                }
                div { class: "card stat-card",
                    div { class: "stat-label", "Risk Index" }
                    div { class: "stat-value", "0.02" }
                }
            }

            div { style: "display: grid; grid-template-columns: 1fr 340px; gap: 32px;",
                div {
                    div { class: "section-title", style: "margin-top: 0;", "LIVE EXECUTION STREAM" }
                    div { class: "card", style: "padding: 0; overflow: hidden;",
                        crate::components::activity_list::ActivityList { 
                            on_select: move |id| { nav.push(Route::ExecutionDetail { id }); } 
                        }
                    }
                }
                
                aside {
                    div { class: "section-title", style: "margin-top: 0;", "SYSTEM ALERTS" }
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        div { class: "card", style: "padding: 16px; border-left: 4px solid var(--status-warning);",
                            div { style: "font-weight: 700; font-size: 13px; margin-bottom: 4px;", "Rate Limit Warning" }
                            p { style: "font-size: 12px; color: var(--text-faint); margin: 0;", "HubSpot API is approaching 80% capacity." }
                        }
                        div { class: "card", style: "padding: 16px; border-left: 4px solid var(--status-info);",
                            div { style: "font-weight: 700; font-size: 13px; margin-bottom: 4px;", "Policy Update" }
                            p { style: "font-size: 12px; color: var(--text-faint); margin: 0;", "Global 'ExpenseGate' v2.1 deployed." }
                        }
                    }
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
fn ViewDiffPage(name: String, v1: String, v2: String) -> Element {
    rsx! { crate::components::view_diff::ViewDiff { name, v1, v2 } }
}

#[component]
fn About() -> Element {
    rsx! { AboutPage {} }
}

#[component]
fn Settings() -> Element {
    let email = crate::api::get_user_email();
    let role = crate::api::get_user_role();
    let role_ref = role.as_deref();

    // Check if user can edit settings
    let can_edit = has_permission(role_ref, &Permission::SettingsEdit);

    rsx! { SettingsPage { user_email: email, user_role: role, can_edit } }
}

#[component]
fn SettingsPage(user_email: Option<String>, user_role: Option<String>, can_edit: bool) -> Element {
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

            div { class: "section-title", "SECURITY" }
            crate::components::mfa_setup::MfaSetup {}

            div { class: "section-title", "PREFERENCES" }
            div { class: "card",
                div { style: "display: flex; align-items: center; justify-content: space-between;",
                    div {
                        div { style: "font-weight: 700; font-size: 1.1rem;", "High-Precision Mode" }
                        div { style: "font-size: 13px; color: var(--text-faint);", "Enable forensic tracing by default for all executions." }
                    }
                    if can_edit {
                        button { class: "btn", "Enabled" }
                    } else {
                        button { class: "btn", disabled: true, "Enabled" }
                    }
                }
            }
        }
    }
}

#[component]
fn AccessDenied() -> Element {
    let nav = use_navigator();
    let user_role = crate::api::get_user_role();

    rsx! {
        div { class: "fade-in", style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; padding: 24px;",
            div { style: "text-align: center; max-width: 480px;",
                div { style: "font-size: 4rem; margin-bottom: 16px;", "🔒" }
                h1 { style: "font-size: 2rem; margin-bottom: 16px;", "Access Denied" }
                p { style: "color: var(--text-secondary); margin-bottom: 24px;",
                    "You don't have permission to access this resource."
                }
                if let Some(role) = user_role {
                    p { style: "font-size: 0.9rem; color: var(--text-faint); margin-bottom: 32px;",
                        "Current role: "
                        span { style: "font-weight: 600; color: var(--text-primary);", "{role}" }
                    }
                }
                div { style: "display: flex; gap: 12px; justify-content: center;",
                    Link {
                        class: "btn btn-primary",
                        to: Route::Dashboard {},
                        "Go to Dashboard"
                    }
                    button {
                        class: "btn",
                        onclick: move |_| {
                            crate::api::logout();
                            nav.push(Route::Home {});
                        },
                        "Sign Out"
                    }
                }
            }
        }
    }
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


