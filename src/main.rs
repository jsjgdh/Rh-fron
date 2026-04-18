//! Rhexiom Frontend — Workflow Operating Interface
//!
//! Built with Dioxus for the web. Provides workflow creation, visualization,
//! execution, and version management.

mod api;
mod app;
mod components;

fn main() {
    dioxus::launch(app::App);
}
