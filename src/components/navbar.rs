use dioxus::prelude::*;
use crate::app::Route;

#[component]
pub fn Navbar() -> Element {
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
