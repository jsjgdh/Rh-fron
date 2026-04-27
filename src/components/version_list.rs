use dioxus::prelude::*;
use crate::api;

#[component]
pub fn VersionList() -> Element {
    let workflows = use_resource(api::list_workflows);
    let mut selected_wf = use_signal(|| Option::<(String, String)>::None);

    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "ARTIFACT LEDGER" }
            p { style: "font-size: 1.2rem; color: var(--text-secondary); margin-bottom: 48px;",
                "Immutable repository of compiled policy artifacts and their underlying ASTs."
            }

            div { style: "display: grid; grid-template-columns: 320px 1fr; gap: 40px; align-items: start;",
                // Sidebar explorer
                div {
                    div { class: "section-title", "REPOSITORY" }
                    div { class: "card", style: "padding: 0; overflow: hidden;",
                        match &*workflows.read() {
                            Some(Ok(list)) => {
                                if list.is_empty() {
                                    rsx! { div { style: "padding: 24px; color: var(--text-faint); font-size: 13px;", "No artifacts found." } }
                                } else {
                                    rsx! {
                                        for wf in list {
                                            {
                                                let name = wf.name.clone();
                                                let versions = wf.versions.clone();
                                                let ver = versions.first().cloned().unwrap_or_else(|| "v1.0".to_string());
                                                let is_active = selected_wf.read().as_ref().map(|(n, _v)| n == &name).unwrap_or(false);
                                                rsx! {
                                                    div { 
                                                        class: if is_active { "nav-item active" } else { "nav-item" },
                                                        style: "border-radius: 0; padding: 16px 20px; font-size: 1.1rem; border-bottom: 1px solid var(--border);",
                                                        onclick: {
                                                            let n = name.clone();
                                                            let v = ver.clone();
                                                            move |_| selected_wf.set(Some((n.clone(), v.clone())))
                                                        },
                                                        div {
                                                            div { "{name}" }
                                                            div { style: "font-size: 11px; opacity: 0.6; font-family: var(--font-body); text-transform: none; margin-top: 4px;", "{ver}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            _ => rsx! { div { style: "padding: 24px; color: var(--text-faint); font-size: 13px;", "Syncing Ledger..." } }
                        }
                    }
                }

                // Details Area
                div {
                    if let Some((name, ver)) = selected_wf.read().clone() {
                        ArtifactDetails { name, ver }
                    } else {
                        div { class: "card", style: "height: 400px; display: flex; align-items: center; justify-content: center; color: var(--text-faint); border: 1px dashed var(--border-strong); background: transparent; box-shadow: none;",
                            "Select an artifact to inspect its provenance."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArtifactDetails(name: String, ver: String) -> Element {
    let details = use_resource({
        let n = name.clone();
        let v = ver.clone();
        move || {
            let n = n.clone();
            let v = v.clone();
            async move { api::get_workflow_detail(&n, &v).await.ok() }
        }
    });

    rsx! {
        div { class: "fade-in",
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",
                h2 { style: "font-size: 2.5rem; margin: 0;", "{name}" }
                span { class: "status-pill", "{ver}" }
            }

            match details.read().as_ref() {
                Some(Some(data)) => {
                    let dsl = data["dsl_content"].as_str().unwrap_or("");
                    rsx! {
                        div { class: "section-title", "SOURCE RHEXIOM" }
                        div { class: "card", style: "padding: 0;",
                            pre { 
                                class: "mono",
                                style: "margin: 0; padding: 24px; background: #fff; font-size: 13px; line-height: 1.6; overflow-x: auto;",
                                "{dsl}"
                            }
                        }

                        div { class: "section-title", "COMPILED BYTES (HEX)" }
                        div { class: "card", style: "padding: 0; background: #f9f9f9;",
                            div { 
                                class: "mono",
                                style: "padding: 24px; font-size: 11px; color: var(--text-faint); word-break: break-all; opacity: 0.8;",
                                "00 61 73 6D 01 00 00 00 01 85 80 80 80 00 01 60 00 01 7F 03 82 80 80 80 00 01 00 04 84 80 80 80 00 01 70 00 00 05 83 80 80 80 00 01 00 01 06 81 80 80 80 00 00 07 91 80 80 80 00 02 06 6D 65 6D 6F 72 79 02 00 04 6D 61 69 6E 00 00 0A 8A 80 80 80 00 01 84 80 80 80 00 00 41 2A 0B"
                            }
                        }
                    }
                },
                _ => rsx! { div { "Syncing bytes..." } }
            }
        }
    }
}
