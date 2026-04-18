//! Execution history row component.

use dioxus::prelude::*;

#[component]
pub fn HistoryRow(name: String, version: String, status: String, step: String, created: String, id: String) -> Element {
    let date_part = created.split('T').next().unwrap_or("").to_string();
    let id_short = id.split('-').next().unwrap_or("").to_string();
    
    let status_class = match status.as_str() {
        "Completed" => "badge badge-success",
        "Failed" => "badge badge-danger",
        _ => "badge badge-warning",
    };

    rsx! {
        div { class: "type-row",
            div { class: "type-cell-code", "{name}" }
            div { "{version}" }
            div {
                span { class: "{status_class}", "{status}" }
            }
            div { class: "type-cell-code", "{step}" }
            div { class: "type-cell-small", "{date_part}" }
            div { class: "type-cell-small type-cell-code", "{id_short}..." }
        }
    }
}
