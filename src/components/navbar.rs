use dioxus::prelude::*;
use crate::app::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header { class: "nav-bar",
            div { 
                class: "nav-links",
                Link { 
                    class: "brand-inline",
                    style: "cursor: pointer; gap: 12px;",
                    to: Route::Home {},
                    img { 
                        src: asset!("/assets/image.png"), 
                        style: "width: 24px; height: 24px; object-fit: contain;" 
                    }
                    span { class: "brand-name", style: "font-size: 1.25rem;", "RHEXIOM" }
                }
            }

            nav { class: "nav-links",
                Link { class: "nav-link", to: Route::About {}, "About" }
                
                div { style: "width: 1px; height: 16px; background: var(--border-strong); margin: 0 8px;" }
                
                Link { 
                    class: "nav-link", 
                    style: "color: var(--text-primary);",
                    to: Route::AuthForm {},
                    "Log In" 
                }
                Link { 
                    class: "btn btn-primary",
                    style: "padding: 8px 20px; font-size: 13px; display: flex; align-items: center;",
                    to: Route::AuthForm {},
                    "Sign Up"
                }
            }
        }
    }
}
