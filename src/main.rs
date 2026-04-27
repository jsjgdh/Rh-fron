//! Rhexiom Frontend — Workflow Operating Interface
//!
//! Built with Dioxus for the web. Provides workflow creation, visualization,
//! execution, and version management.

mod api;
mod app;
mod auth;
mod components;
mod i18n;

fn main() {
    dioxus::launch(app::App);
}
