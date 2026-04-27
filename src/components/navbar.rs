use dioxus::prelude::*;
use crate::app::Route;
use crate::auth::{has_permission, Permission};

#[component]
pub fn Navbar() -> Element {
    let token = crate::api::get_token();
    let user_role = crate::api::get_user_role();
    let role_ref = user_role.as_deref();
    let is_logged_in = token.is_some();

    // Determine what to show based on role
    let show_workflows = has_permission(role_ref, &Permission::WorkflowsView);
    let show_executions = has_permission(role_ref, &Permission::ExecutionsView);
    let show_integrations = has_permission(role_ref, &Permission::IntegrationsView);
    let show_settings = has_permission(role_ref, &Permission::SettingsView);

    rsx! {
        header {
            class: "nav-bar",
            role: "banner",
            // Skip to content link
            a {
                class: "skip-to-content",
                href: "#main-content",
                "Skip to main content"
            }
            Link {
                class: "brand-inline",
                to: Route::Home {},
                aria_label: "Rhexiom Home",
                img {
                    src: asset!("/assets/image.png"),
                    style: "width: 32px; height: 32px; object-fit: contain;",
                    alt: "Rhexiom Logo"
                }
                span { class: "brand-name", style: "font-size: 1.5rem; letter-spacing: -0.04em;", "RHEXIOM" }
            }

            nav {
                class: "nav-links",
                role: "navigation",
                aria_label: "Main navigation",
                Link { class: "nav-link", to: Route::About {}, "About" }

                div { style: "width: 1px; height: 20px; background: var(--border); margin: 0 8px;", aria_hidden: "true" }

                if is_logged_in {
                    // Show quick navigation for logged-in users
                    if show_workflows {
                        Link { class: "nav-link", to: Route::Ledger {}, "Workflows" }
                    }
                    if show_executions {
                        Link { class: "nav-link", to: Route::History {}, "Executions" }
                    }
                    if show_integrations {
                        Link { class: "nav-link", to: Route::Integrations {}, "Integrations" }
                    }
                    if show_settings {
                        Link { class: "nav-link", to: Route::Settings {}, "Settings" }
                    }
                    div { style: "width: 1px; height: 20px; background: var(--border); margin: 0 8px;", aria_hidden: "true" }
                    Link {
                        class: "nav-link",
                        style: "color: var(--text-primary); text-transform: none; letter-spacing: 0;",
                        to: Route::Dashboard {},
                        "Dashboard"
                    }
                } else {
                    // Show login/signup for guests
                    Link {
                        class: "nav-link",
                        style: "color: var(--text-primary); text-transform: none; letter-spacing: 0;",
                        to: Route::AuthForm {},
                        "Log In"
                    }
                    Link {
                        class: "btn btn-primary",
                        style: "height: 40px; padding: 0 24px; font-size: 14px; border-radius: 10px;",
                        to: Route::AuthForm {},
                        "Sign Up"
                    }
                }
            }
        }
    }
}
