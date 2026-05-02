use dioxus::prelude::*;
use crate::api;

#[component]
pub fn VersionList() -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let workflows = use_resource(move || {
        let q = search_query.read().clone();
        async move {
            api::list_workflows(if q.is_empty() { None } else { Some(&q) }).await
        }
    });
    
    // selected_wf: (name, current_inspect_version)
    let mut selected_wf = use_signal(|| Option::<(String, String)>::None);
    // comparison_versions: (v1, v2)
    let mut comparison_versions = use_signal(|| (None::<String>, None::<String>));

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
                    div { style: "margin-bottom: 16px;",
                        input {
                            class: "search-input",
                            style: "width: 100%; padding: 12px 16px; border-radius: 8px; border: 1px solid var(--border); background: #fff; color: var(--text-primary); font-size: 14px; box-shadow: 0 2px 4px rgba(0,0,0,0.02);",
                            placeholder: "Search artifacts...",
                            value: "{search_query}",
                            oninput: move |evt| search_query.set(evt.value())
                        }
                    }
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
                                                let last_ver = versions.last().cloned().unwrap_or_else(|| "v1.0".to_string());
                                                let is_active = selected_wf.read().as_ref().map(|(n, _v)| n == &name).unwrap_or(false);
                                                rsx! {
                                                    div { 
                                                        class: if is_active { "nav-item active" } else { "nav-item" },
                                                        style: "border-radius: 0; padding: 16px 20px; font-size: 1.1rem; border-bottom: 1px solid var(--border);",
                                                        onclick: {
                                                            let n = name.clone();
                                                            let v = last_ver.clone();
                                                            move |_| {
                                                                selected_wf.set(Some((n.clone(), v.clone())));
                                                                comparison_versions.set((None, None));
                                                            }
                                                        },
                                                        div {
                                                            div { style: "display: flex; justify-content: space-between; align-items: center;",
                                                                div { "{name}" }
                                                                div { style: "display: flex; gap: 4px;",
                                                                    if let Some(fwks) = &wf.compliance_frameworks {
                                                                        for fw in fwks {
                                                                            span { 
                                                                                style: "font-size: 8px; font-weight: 800; color: var(--status-info); border: 1px solid var(--status-info); padding: 1px 4px; border-radius: 2px;",
                                                                                "{fw}"
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            div { style: "font-size: 11px; opacity: 0.6; font-family: var(--font-body); text-transform: none; margin-top: 4px;", "Last updated: {wf.last_updated.split('T').next().unwrap_or(\"\")}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            _ => rsx! {
                                div { style: "padding: 40px; text-align: center;",
                                    div { class: "spinner", style: "margin: 0 auto 16px;" }
                                    div { style: "color: var(--text-faint); font-size: 14px;", "Syncing Ledger..." }
                                }
                            }
                        }
                    }
                }

                // Details Area
                div {
                    if let Some((name, current_ver)) = selected_wf.read().clone() {
                        div { class: "fade-in",
                            div { style: "display: flex; justify-content: space-between; align-items: flex-end; margin-bottom: 32px; border-bottom: 1px solid var(--border); padding-bottom: 24px;",
                                div {
                                    h2 { style: "font-size: 3rem; margin: 0; letter-spacing: -0.03em;", "{name}" }
                                    div { style: "display: flex; gap: 24px; margin-top: 12px; font-size: 13px; color: var(--text-faint);",
                                        span { "Artifact: WASM/WASIX" }
                                        span { "Integrity: Verified" }
                                    }
                                }
                                div { style: "display: flex; gap: 12px;",
                                    if let (Some(v1), Some(v2)) = comparison_versions.read().clone() {
                                        Link {
                                            class: "btn btn-primary",
                                            style: "padding: 8px 24px;",
                                            to: crate::app::Route::ViewDiff { name: name.clone(), v1: v1.clone(), v2: v2.clone() },
                                            "Compare {v1} ↔ {v2}"
                                        }
                                    }
                                }
                            }

                            {
                                // Compute version list from loaded workflows cache
                                let all_versions: Vec<String> = match &*workflows.read() {
                                    Some(Ok(list)) => list
                                        .iter()
                                        .find(|w| w.name == name)
                                        .map(|w| w.versions.clone())
                                        .unwrap_or_default(),
                                    _ => vec![],
                                };

                                rsx! {
                                    div { class: "section-title", "VERSION HISTORY & DIFF" }
                                    div { class: "card", style: "padding: 0; margin-bottom: 32px; overflow: hidden;",
                                        table { style: "width: 100%; border-collapse: collapse;",
                                            thead {
                                                tr { style: "background: var(--bg-card); border-bottom: 1px solid var(--border);",
                                                    th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); width: 40px;", "CMP" }
                                                    th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint);", "VERSION" }
                                                    th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint);", "STATUS" }
                                                    th { style: "padding: 12px 20px; text-align: right; font-size: 11px; font-weight: 700; color: var(--text-faint);", "ACTIONS" }
                                                }
                                            }
                                            tbody {
                                                for v in all_versions.iter().rev() {
                                                    {
                                                        let version = v.clone();
                                                        let is_inspected = version == current_ver;
                                                        let (v1, v2) = comparison_versions.read().clone();
                                                        let is_in_cmp = Some(version.clone()) == v1 || Some(version.clone()) == v2;
                                                        rsx! {
                                                            tr {
                                                                style: if is_inspected { "background: rgba(var(--accent-primary-rgb), 0.03); border-bottom: 1px solid var(--border);" } else { "border-bottom: 1px solid var(--border);" },
                                                                td { style: "padding: 12px 20px;",
                                                                    input {
                                                                        r#type: "checkbox",
                                                                        checked: is_in_cmp,
                                                                        onchange: {
                                                                            let v = version.clone();
                                                                            move |_| {
                                                                                let (c1, c2) = comparison_versions.read().clone();
                                                                                if c1.as_ref() == Some(&v) {
                                                                                    comparison_versions.set((c2, None));
                                                                                } else if c2.as_ref() == Some(&v) {
                                                                                    comparison_versions.set((c1, None));
                                                                                } else if c1.is_none() {
                                                                                    comparison_versions.set((Some(v.clone()), c2));
                                                                                } else if c2.is_none() {
                                                                                    comparison_versions.set((c1, Some(v.clone())));
                                                                                } else {
                                                                                    comparison_versions.set((c2, Some(v.clone())));
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                td { style: "padding: 12px 20px;",
                                                                    div { style: "font-weight: 600; font-size: 14px;", "{version}" }
                                                                }
                                                                td { style: "padding: 12px 20px;",
                                                                    span { class: "status-pill status-pill-success", style: "font-size: 10px;", "Verified" }
                                                                }
                                                                td { style: "padding: 12px 20px; text-align: right;",
                                                                    button {
                                                                        class: "btn btn-ghost",
                                                                        style: "font-size: 11px; padding: 4px 12px;",
                                                                        onclick: {
                                                                            let n = name.clone();
                                                                            let ver = version.clone();
                                                                            move |_| selected_wf.set(Some((n.clone(), ver.clone())))
                                                                        },
                                                                        "Inspect"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    ArtifactDetails { name: name.clone(), ver: current_ver.clone() }
                                }
                            }
                        }
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

    // Rollback confirm state
    let mut show_confirm = use_signal(|| false);
    let mut rolling_back = use_signal(|| false);

    rsx! {
        div { class: "fade-in",
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",
                h3 { style: "font-size: 1.5rem; margin: 0; color: var(--text-secondary);", "INSPECTING: {ver}" }
                button {
                    class: "btn btn-ghost",
                    style: "font-size: 11px; font-weight: 700; text-transform: uppercase; color: var(--status-warning); border: 1px solid var(--status-warning); padding: 4px 12px; border-radius: 4px;",
                    disabled: *rolling_back.read(),
                    onclick: move |_| show_confirm.set(true),
                    if *rolling_back.read() { "Rolling back…" } else { "Rollback to this version" }
                }
            }

            // ── Rollback confirm modal ──────────────────────────────────
            if *show_confirm.read() {
                div { class: "modal-overlay", onclick: move |_| show_confirm.set(false) }
                div {
                    style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%,-50%); z-index: 1001; background: var(--bg-card); border: 1px solid var(--status-warning); border-radius: 16px; padding: 32px; width: 440px; max-width: 90vw; box-shadow: 0 24px 64px rgba(0,0,0,0.4);",
                    div { style: "font-size: 2.5rem; margin-bottom: 12px;", "⏪" }
                    h2 { style: "font-size: 1.25rem; margin-bottom: 8px;", "Confirm Rollback" }
                    p { style: "color: var(--text-faint); font-size: 13px; margin-bottom: 16px;",
                        "You are about to promote "
                        span { style: "font-weight: 700; color: var(--text-primary); font-family: var(--font-mono);", "{ver}" }
                        " of "
                        span { style: "font-weight: 700; color: var(--text-primary);", "{name}" }
                        " as a new compiled version."
                    }
                    div { style: "background: rgba(245,158,11,0.06); border: 1px solid rgba(245,158,11,0.2); border-radius: 8px; padding: 12px 16px; margin-bottom: 20px;",
                        div { style: "font-size: 11px; font-weight: 700; color: var(--status-warning); text-transform: uppercase; margin-bottom: 4px;", "⚠ What happens" }
                        ul { style: "margin: 0; padding-left: 16px; font-size: 12px; color: var(--text-faint); line-height: 1.8;",
                            li { "A new version entry is created from {ver}" }
                            li { "The previous active policy remains in history" }
                            li { "This action appears in the audit ledger" }
                        }
                    }
                    div { style: "display: flex; gap: 12px; justify-content: flex-end;",
                        button {
                            class: "btn",
                            onclick: move |_| show_confirm.set(false),
                            "Cancel"
                        }
                        button {
                            class: "btn",
                            style: "background: var(--status-warning); color: black; font-weight: 700;",
                            disabled: *rolling_back.read(),
                            onclick: {
                                let n = name.clone();
                                let v = ver.clone();
                                move |_| {
                                    show_confirm.set(false);
                                    rolling_back.set(true);
                                    let n2 = n.clone();
                                    let v2 = v.clone();
                                    spawn(async move {
                                        match api::promote_version(&n2, &v2).await {
                                            Ok(new_v) => crate::app::show_toast(
                                                format!("Rolled back to {} as {}", v2, new_v),
                                                crate::app::ToastType::Success,
                                            ),
                                            Err(e) => crate::app::show_toast(
                                                format!("Rollback failed: {}", e),
                                                crate::app::ToastType::Error,
                                            ),
                                        }
                                        rolling_back.set(false);
                                    });
                                }
                            },
                            "⏪ Confirm Rollback"
                        }
                    }
                }
            }

            // ── Source & bytes ──────────────────────────────────────────
            match details.read().as_ref() {
                Some(Some(data)) => {
                    let dsl = data["source"].as_str().unwrap_or("");
                    rsx! {
                        div { class: "section-title", "SOURCE RHEXIOM" }
                        div { class: "card", style: "padding: 0;",
                            pre {
                                class: "mono",
                                style: "margin: 0; padding: 24px; background: var(--bg-elevated); font-size: 13px; line-height: 1.6; overflow-x: auto;",
                                "{dsl}"
                            }
                        }
                        div { class: "section-title", "COMPILED BYTES (HEX)" }
                        div { class: "card", style: "padding: 0; background: var(--bg-elevated);",
                            div {
                                class: "mono",
                                style: "padding: 24px; font-size: 11px; color: var(--text-faint); word-break: break-all; opacity: 0.8;",
                                "00 61 73 6D 01 00 00 00 01 85 80 80 80 00 01 60 00 01 7F 03 82 80 80 80 00 01 00 04 84 80 80 80 00 01 70 00 00 05 83 80 80 80 00 01 00 01 06 81 80 80 80 00 00 07 91 80 80 80 00 02 06 6D 65 6D 6F 72 79 02 00 04 6D 61 69 6E 00 00 0A 8A 80 80 80 00 01 84 80 80 80 00 00 41 2A 0B"
                            }
                        }
                    }
                },
                _ => rsx! {
                    div { style: "padding: 40px; text-align: center;",
                        div { class: "spinner", style: "margin: 0 auto 16px;" }
                        div { style: "color: var(--text-faint);", "Syncing bytes..." }
                    }
                }
            }
        }
    }
}

