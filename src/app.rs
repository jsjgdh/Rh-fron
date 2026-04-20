//! Root application component and routing.

use dioxus::prelude::*;

use crate::components::documentation::Documentation;
use crate::components::execution_detail::ExecutionDetail;
use crate::components::execution_form::ExecutionForm;
use crate::components::execution_history::ExecutionHistory;
use crate::components::home::Home;
use crate::components::upload::Upload;
use crate::components::version_list::VersionList;
use crate::components::view_edit::ViewEdit;
use crate::components::view_gen_workflow::ViewGenWorkflow;

/// The view currently displayed in the main content area.
#[derive(Clone, PartialEq)]
pub enum View {
    Home,
    Upload,
    ViewGenWorkflow { name: String, version: String },
    ViewEdit { name: String, version: String },
    Dashboard,
    Execute,
    Versions,
    Integrations,
    Documentation,
    Visualize,
    History,
    ExecutionDetail { id: String },
}

fn view_meta(view: &View) -> (&'static str, String, String) {
    match view {
        View::Home => (
            "Launchpad",
            "Rhexiom".to_string(),
            "Policy execution with a design system that matches the product.".to_string(),
        ),
        View::Upload => (
            "Create",
            "Policy Studio".to_string(),
            "Describe a policy, import a document, and compile an immutable workflow version."
                .to_string(),
        ),
        View::ViewGenWorkflow { name, version } => (
            "Inspect",
            format!("{} · {}", name, version),
            "Review the generated graph, source artifacts, and deployment posture.".to_string(),
        ),
        View::ViewEdit { name, version } => (
            "Refine",
            format!("Edit {} · {}", name, version),
            "Tune source code and publish a fresh compiled version when you are ready.".to_string(),
        ),
        View::Dashboard => (
            "Overview",
            "Operations Console".to_string(),
            "Track the pipeline, active workflows, and the runtime posture of the platform."
                .to_string(),
        ),
        View::Execute => (
            "Run",
            "Execution Console".to_string(),
            "Inject inputs, execute a workflow, and inspect the resulting trace.".to_string(),
        ),
        View::Versions => (
            "Archive",
            "Version Ledger".to_string(),
            "Browse immutable artifacts and compare source, AST, and IR snapshots.".to_string(),
        ),
        View::Documentation => (
            "Learn",
            "RheLang Guide".to_string(),
            "Keep the language guide, examples, and modeling rules close to the editor."
                .to_string(),
        ),
        View::Integrations => (
            "Connect",
            "Service Registry".to_string(),
            "Manage API credentials for external services like HubSpot and Salesforce."
                .to_string(),
        ),
        View::Visualize => (
            "Explore",
            "Policy Visualizer".to_string(),
            "Browse deployed workflow versions and inspect their logic graphs.".to_string(),
        ),
        View::History => (
            "Activity",
            "Execution History".to_string(),
            "Browse past runs, check status, and audit policy outcomes.".to_string(),
        ),
        View::ExecutionDetail { id } => (
            "Audit",
            format!("Audit Run {}", &id[..8.min(id.len())]),
            "Forensic view of the policy execution path and telemetry trace.".to_string(),
        ),
    }
}

/// Root application component.
#[component]
pub fn App() -> Element {
    let mut current_view = use_signal(|| View::Home);
    let mut token = use_signal(crate::api::get_token);
    let mut user_email = use_signal(crate::api::get_user_email);

    // If we have a token and we are on Home, auto-redirect to Dashboard
    use_effect(move || {
        if token.read().is_some() && *current_view.read() == View::Home {
            current_view.set(View::Dashboard);
        }
    });

    let current = current_view.read().clone();
    let (eyebrow, title, description) = view_meta(&current);

    let handle_login = move |res: crate::api::AuthResponse| {
        if let Some(t) = res.token {
            crate::api::set_token(&t);
            token.set(Some(t));
        }
        if let Some(e) = res.email {
            crate::api::set_user_email(&e);
            user_email.set(Some(e));
        }
        current_view.set(View::Dashboard);
    };


    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }

        if matches!(current, View::Home) {
            Home {
                on_login: handle_login
            }
        } else {
            div { class: "app-container control-shell",
                nav { class: "sidebar control-sidebar",
                    div { class: "sidebar-header",
                        div { class: "brand-lockup",
                            div { class: "brand-mark", "RX" }
                            div {
                                div { class: "brand-name", "Rhexiom" }
                                div { class: "brand-caption", "Control Tower" }
                            }
                        }
                    }

                    div { class: "sidebar-nav",
                        div { class: "sidebar-section-label", "Intelligence" }

                        div {
                            class: if matches!(current, View::Dashboard) { "nav-item active" } else { "nav-item" },
                            onclick: move |_| current_view.set(View::Dashboard),
                            span { class: "nav-icon", "CT" }
                            span { "Control Tower" }
                        }

                        div { class: "sidebar-section-label", "Production" }

                        div {
                            class: if matches!(
                                current,
                                View::Upload | View::ViewGenWorkflow { .. } | View::ViewEdit { .. }
                            ) {
                                "nav-item active"
                            } else {
                                "nav-item"
                            },
                            onclick: move |_| current_view.set(View::Upload),
                            span { class: "nav-icon", "WF" }
                            span { "Policy Studio" }
                        }

                        div {
                            class: if matches!(current, View::Execute) { "nav-item active" } else { "nav-item" },
                            onclick: move |_| current_view.set(View::Execute),
                            span { class: "nav-icon", "EX" }
                            span { "Execution Sandbox" }
                        }

                        div {
                            class: if matches!(current, View::Integrations) { "nav-item active" } else { "nav-item" },
                            onclick: move |_| current_view.set(View::Integrations),
                            span { class: "nav-icon", "SG" }
                            span { "Service Registry" }
                        }

                        div { class: "sidebar-section-label", "Governance" }

                        div {
                            class: if matches!(current, View::History | View::ExecutionDetail { .. }) { "nav-item active" } else { "nav-item" },
                            onclick: move |_| current_view.set(View::History),
                            span { class: "nav-icon", "AL" }
                            span { "Audit Logs" }
                        }

                        div {
                            class: if matches!(current, View::Versions) { "nav-item active" } else { "nav-item" },
                            onclick: move |_| current_view.set(View::Versions),
                            span { class: "nav-icon", "VL" }
                            span { "Version Ledger" }
                        }

                        div {
                            class: if matches!(current, View::Documentation) { "nav-item active" } else { "nav-item" },
                            onclick: move |_| current_view.set(View::Documentation),
                            span { class: "nav-icon", "DX" }
                            span { "Documentation" }
                        }
                    }

                    div { class: "sidebar-footer",
                        if let Some(email) = user_email.read().as_ref() {
                            div { class: "profile-info",
                                div {
                                    div { class: "profile-role", "System Administrator" }
                                    div { class: "profile-email", "{email}" }
                                }
                                button {
                                    class: "btn",
                                    style: "width: 100%; height: 36px; font-size: 11px; background: rgba(255,255,255,0.05); color: var(--sidebar-text); border-color: rgba(255,255,255,0.1);",
                                    onclick: move |_| {
                                        crate::api::logout();
                                        token.set(None);
                                        user_email.set(None);
                                        current_view.set(View::Home);
                                    },
                                    "Sign out of Rhexiom"
                                }
                            }
                        }
                    }
                }

                main { class: "main-content control-main",
                    header { class: "app-header control-header",
                        div { class: "control-headline",
                            div { class: "app-eyebrow", "{eyebrow}" }
                            h1 { class: "app-title", "{title}" }
                            p { class: "app-subtitle", "{description}" }
                        }
                        div { class: "control-clock",
                            div { class: "clock-label", "System State" }
                            div { class: "clock-value", "Live Telemetry" }
                        }
                    }

                    div { class: "content-frame control-frame",
                        match &current {
                            View::Home => rsx! {
                                Home {
                                    on_login: handle_login
                                }
                            },
                            View::Upload => rsx! {
                                Upload {
                                    on_generation_complete: move |(name, version)| {
                                        current_view.set(View::ViewGenWorkflow { name, version })
                                    }
                                }
                            },
                            View::ViewGenWorkflow { name, version } => rsx! {
                                ViewGenWorkflow {
                                    workflow_name: name.clone(),
                                    version: version.clone(),
                                    on_edit_requested: {
                                        let name = name.clone();
                                        let version = version.clone();
                                        move |_| current_view.set(View::ViewEdit {
                                            name: name.clone(),
                                            version: version.clone(),
                                        })
                                    }
                                }
                            },
                            View::ViewEdit { name, version } => rsx! {
                                ViewEdit {
                                    workflow_name: name.clone(),
                                    version: version.clone(),
                                    on_back: {
                                        let name = name.clone();
                                        let version = version.clone();
                                        move |_| current_view.set(View::ViewGenWorkflow {
                                            name: name.clone(),
                                            version: version.clone(),
                                        })
                                    },
                                    on_recompiled: move |(name, version)| {
                                        current_view.set(View::ViewGenWorkflow { name, version })
                                    }
                                }
                            },
                            View::Dashboard => rsx! { DashboardView { current_view } },
                            View::Execute => rsx! { ExecutionForm {} },
                            View::Versions => rsx! { VersionList {} },
                            View::Integrations => rsx! { crate::components::integrations::Integrations {} },
                            View::Documentation => rsx! { Documentation {} },
                            View::Visualize => rsx! { crate::components::visualize::Visualize {} },
                            View::History => rsx! {
                                ExecutionHistory { 
                                    on_select: move |id| current_view.set(View::ExecutionDetail { id })
                                } 
                            },
                            View::ExecutionDetail { id } => rsx! { 
                                ExecutionDetail { 
                                    execution_id: id.clone() 
                                } 
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Dashboard overview showing system stats and recent workflows.
#[component]
fn DashboardView(mut current_view: Signal<View>) -> Element {
    let stats_res = use_resource(|| async move { crate::api::get_stats().await.ok() });
    let (active_workflows, total_executions, system_status) =
        if let Some(Some(stats)) = stats_res.read().as_ref() {
            (
                stats.active_workflows.to_string(),
                stats.total_executions.to_string(),
                stats.system_status.clone(),
            )
        } else {
            (
                "...".to_string(),
                "...".to_string(),
                "Stabilizing".to_string(),
            )
        };

    rsx! {
        div { class: "dashboard-stack control-dashboard",
            section { class: "grid-metrics",
                div { class: "industrial-card glass",
                    div { class: "label-caps", "System Telemetry" }
                    div { class: "stat-value", "{system_status}" }
                    div { class: "kpi-meta", "Forensic trace online" }
                }

                div { class: "industrial-card glass",
                    div { class: "label-caps", "Active Policies" }
                    div { class: "stat-value", "{active_workflows}" }
                    div { class: "kpi-meta", "Zero drift detected" }
                }

                div { class: "industrial-card glass",
                    div { class: "label-caps", "Total Throughput" }
                    div { class: "stat-value", "{total_executions}" }
                    div { class: "kpi-meta", "Verified on-chain" }
                }
            }

            section { class: "control-two-column",
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: var(--gap-main);",
                article { class: "industrial-card",
                    div { class: "label-caps", "Approval Queue" }
                    h3 { style: "margin: 8px 0 12px; font-size: 20px;", "Pending Authorization" }
                    p { class: "panel-copy", style: "margin-bottom: 24px; color: var(--text-secondary);",
                        "Policy transitions held for architect sign-off. Review the immutable audit trail before continuing."
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| current_view.set(View::History),
                        "Review Audit Trail"
                    }
                }

                article { class: "industrial-card",
                    div { class: "label-caps", "Studio Intelligence" }
                    h3 { style: "margin: 8px 0 12px; font-size: 20px;", "Authoring Queue" }
                    p { class: "panel-copy", style: "margin-bottom: 24px; color: var(--text-secondary);",
                        "Jump straight into your workspace to draft logic or audit the version ledger."
                    }
                    div { class: "control-action-grid", style: "display: flex; gap: 12px;",
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| current_view.set(View::Upload),
                            "Open Studio"
                        }
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| current_view.set(View::Versions),
                            "View Ledger"
                        }
                    }
                }
            }

            section { class: "industrial-card",
                div { class: "label-caps", "Forensic Execution Stream" }
                div { class: "type-table", style: "margin-top: 24px;",
                    div { class: "type-row type-row-head",
                        div { "Workflow" }
                        div { "Status" }
                        div { "Timestamp" }
                        div { "Trace ID" }
                    }
                    crate::components::activity_list::ActivityList { 
                        on_select: move |id| current_view.set(View::ExecutionDetail { id })
                    }
                }
            }
        }
    }
}

