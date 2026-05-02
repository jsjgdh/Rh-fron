use dioxus::prelude::*;
use crate::api;
use crate::app::{show_toast, ToastType};

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleMode {
    Cron,
    Once,
}

impl Default for ScheduleMode {
    fn default() -> Self {
        ScheduleMode::Cron
    }
}

impl ScheduleMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScheduleMode::Cron => "cron",
            ScheduleMode::Once => "once",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ScheduledJob {
    pub id: String,
    pub workflow_name: String,
    pub version: String,
    pub cron_expr: Option<String>,
    pub run_at: Option<String>,      // ISO timestamp for one-shot
    pub enabled: bool,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
    pub created_at: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Schedule component
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn ScheduleManager() -> Element {
    let mut jobs = use_signal(|| Vec::<ScheduledJob>::new());
    let mut loading = use_signal(|| true);
    let mut show_create = use_signal(|| false);

    // New schedule form state
    let mut new_workflow = use_signal(|| String::new());
    let mut new_version = use_signal(|| "v1.0".to_string());
    let mut schedule_mode = use_signal(|| ScheduleMode::Cron);
    let mut new_cron = use_signal(|| "0 9 * * 1-5".to_string()); // Mon-Fri 9am
    let mut new_run_at = use_signal(|| String::new());
    let mut saving = use_signal(|| false);

    let workflows = use_resource(|| async { api::list_workflows(None).await.ok().unwrap_or_default() });

    use_effect(move || {
        spawn(async move {
            loading.set(true);
            match api::list_scheduled_jobs().await {
                Ok(j) => { jobs.set(j); loading.set(false); }
                Err(e) => { tracing::warn!("schedule load error: {}", e); loading.set(false); }
            }
        });
    });

    // Pre-compute mode styles to avoid if-expr inside RSX interpolation
    let cron_btn_bg = if *schedule_mode.read() == ScheduleMode::Cron { "var(--accent-primary)" } else { "var(--bg-card)" };
    let cron_btn_color = if *schedule_mode.read() == ScheduleMode::Cron { "black" } else { "var(--text-secondary)" };
    let once_btn_bg = if *schedule_mode.read() == ScheduleMode::Once { "var(--accent-primary)" } else { "var(--bg-card)" };
    let once_btn_color = if *schedule_mode.read() == ScheduleMode::Once { "black" } else { "var(--text-secondary)" };
    let show_cron_input = *schedule_mode.read() == ScheduleMode::Cron;

    rsx! {
        div { class: "fade-in",
            // ── Header
            div { style: "display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 40px;",
                div {
                    h1 { class: "page-title", "WORKFLOW SCHEDULING" }
                    p { style: "color: var(--text-faint); font-size: 14px; margin-top: 4px;",
                        "Schedule workflows on a cron cadence or at a specific time."
                    }
                }
                button {
                    class: "btn btn-primary",
                    style: "height: 40px; padding: 0 20px; font-size: 13px;",
                    onclick: move |_| show_create.set(true),
                    "＋ New Schedule"
                }
            }

            // ── Cron reference card
            div { class: "card", style: "padding: 16px 20px; margin-bottom: 32px; background: rgba(99,102,241,0.04); border: 1px solid rgba(99,102,241,0.15);",
                div { style: "font-size: 12px; font-weight: 700; color: #6366f1; margin-bottom: 8px; text-transform: uppercase; letter-spacing: 0.05em;", "⏱ Cron Quick Reference" }
                div { style: "display: flex; gap: 24px; flex-wrap: wrap;",
                    CronHint { expr: "0 9 * * 1-5", desc: "Mon–Fri at 9 AM" }
                    CronHint { expr: "0 0 * * *", desc: "Daily at midnight" }
                    CronHint { expr: "0 */6 * * *", desc: "Every 6 hours" }
                    CronHint { expr: "0 0 1 * *", desc: "1st of each month" }
                    CronHint { expr: "*/15 * * * *", desc: "Every 15 minutes" }
                }
            }

            // ── Jobs list
            if *loading.read() {
                div { style: "padding: 60px; text-align: center;",
                    div { class: "spinner", style: "margin: 0 auto 16px;" }
                    div { style: "color: var(--text-faint); font-size: 14px;", "Loading schedules…" }
                }
            } else if jobs.read().is_empty() {
                div { class: "card", style: "padding: 80px; text-align: center; color: var(--text-faint); border: 1px dashed var(--border-strong); background: transparent; box-shadow: none;",
                    div { style: "font-size: 3rem; margin-bottom: 16px;", "🕐" }
                    div { style: "font-size: 1.1rem; font-weight: 600; margin-bottom: 8px;", "No scheduled workflows yet" }
                    div { style: "font-size: 13px; margin-bottom: 24px;", "Set up automatic runs on a cron schedule or at a specific date and time." }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| show_create.set(true),
                        "Create First Schedule"
                    }
                }
            } else {
                div { class: "card", style: "padding: 0; overflow: hidden;",
                    table { style: "width: 100%; border-collapse: collapse;",
                        thead {
                            tr { style: "background: var(--bg-elevated); border-bottom: 1px solid var(--border);",
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Workflow" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Schedule" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Next Run" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Last Run" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Status" }
                                th { style: "padding: 12px 20px; text-align: right; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Actions" }
                            }
                        }
                        tbody {
                            for job in jobs.read().clone().iter() {
                                {
                                    let j = job.clone();
                                    let jid = j.id.clone();
                                    rsx! {
                                        tr {
                                            key: "{j.id}",
                                            style: "border-bottom: 1px solid var(--border);",
                                            td { style: "padding: 16px 20px;",
                                                div { style: "font-weight: 600; font-size: 14px;", "{j.workflow_name}" }
                                                div { style: "font-size: 11px; color: var(--text-faint); font-family: var(--font-mono);", "{j.version}" }
                                            }
                                            td { style: "padding: 16px 20px; font-family: var(--font-mono); font-size: 13px; color: var(--accent-primary);",
                                                if let Some(ref cron) = j.cron_expr {
                                                    "{cron}"
                                                } else if let Some(ref at) = j.run_at {
                                                    "Once @ {at.split('T').next().unwrap_or(\"\")}"
                                                } else {
                                                    "—"
                                                }
                                            }
                                            td { style: "padding: 16px 20px; font-size: 13px; color: var(--text-faint);",
                                                "{j.next_run.as_deref().unwrap_or(\"—\").split('T').next().unwrap_or(\"—\")}"
                                            }
                                            td { style: "padding: 16px 20px; font-size: 13px; color: var(--text-faint);",
                                                "{j.last_run.as_deref().unwrap_or(\"Never\").split('T').next().unwrap_or(\"Never\")}"
                                            }
                                            td { style: "padding: 16px 20px;",
                                                span {
                                                    class: if j.enabled { "status-pill status-pill-success" } else { "status-pill" },
                                                    style: "font-size: 10px;",
                                                    if j.enabled { "Active" } else { "Paused" }
                                                }
                                            }
                                            td { style: "padding: 16px 20px; text-align: right;",
                                                div { style: "display: flex; justify-content: flex-end; gap: 8px;",
                                                    button {
                                                        class: "btn btn-ghost",
                                                        style: "font-size: 11px; padding: 4px 10px;",
                                                        onclick: {
                                                            let jid2 = jid.clone();
                                                            move |_| {
                                                                let id = jid2.clone();
                                                                spawn(async move {
                                                                    match api::toggle_schedule(&id).await {
                                                                        Ok(_) => show_toast("Schedule updated", ToastType::Success),
                                                                        Err(e) => show_toast(format!("Error: {}", e), ToastType::Error),
                                                                    }
                                                                });
                                                            }
                                                        },
                                                        if j.enabled { "Pause" } else { "Resume" }
                                                    }
                                                    button {
                                                        class: "btn btn-danger",
                                                        style: "font-size: 11px; padding: 4px 10px;",
                                                        onclick: {
                                                            let id = jid.clone();
                                                            move |_| {
                                                                let id2 = id.clone();
                                                                spawn(async move {
                                                                    match api::delete_schedule(&id2).await {
                                                                        Ok(_) => show_toast("Schedule deleted", ToastType::Success),
                                                                        Err(e) => show_toast(format!("Error: {}", e), ToastType::Error),
                                                                    }
                                                                });
                                                            }
                                                        },
                                                        "Delete"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Create Schedule Modal
            if *show_create.read() {
                div { class: "modal-overlay", onclick: move |_| show_create.set(false) }
                div {
                    style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%,-50%); z-index: 1001; background: var(--bg-card); border: 1px solid var(--border); border-radius: 16px; padding: 32px; max-width: 520px; width: 90vw; box-shadow: 0 24px 64px rgba(0,0,0,0.4);",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 24px;", "New Schedule" }
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        // Workflow selector
                        div { style: "display: grid; grid-template-columns: 1fr 120px; gap: 12px;",
                            div {
                                label { class: "form-label", "Workflow" }
                                select {
                                    class: "form-input",
                                    style: "width: 100%;",
                                    value: "{new_workflow}",
                                    onchange: move |e| new_workflow.set(e.value()),
                                    option { value: "", disabled: true, "Select workflow" }
                                    if let Some(wfs) = workflows.read().as_ref() {
                                        for wf in wfs.iter() {
                                            {
                                                let n = wf.name.clone();
                                                rsx! { option { value: "{n}", "{n}" } }
                                            }
                                        }
                                    }
                                }
                            }
                            div {
                                label { class: "form-label", "Version" }
                                input {
                                    class: "form-input",
                                    style: "width: 100%;",
                                    placeholder: "v1.0",
                                    value: "{new_version}",
                                    oninput: move |e| new_version.set(e.value()),
                                }
                            }
                        }
                        // Mode toggle
                        div {
                            label { class: "form-label", "Schedule Type" }
                            div { style: "display: flex; gap: 0; border: 1px solid var(--border); border-radius: 8px; overflow: hidden;",
                                button {
                                    style: "flex: 1; padding: 8px; font-size: 13px; font-weight: 600; border: none; cursor: pointer; background: {cron_btn_bg}; color: {cron_btn_color};",
                                    onclick: move |_| schedule_mode.set(ScheduleMode::Cron),
                                    "🔄 Recurring (Cron)"
                                }
                                button {
                                    style: "flex: 1; padding: 8px; font-size: 13px; font-weight: 600; border: none; border-left: 1px solid var(--border); cursor: pointer; background: {once_btn_bg}; color: {once_btn_color};",
                                    onclick: move |_| schedule_mode.set(ScheduleMode::Once),
                                    "📅 One-shot"
                                }
                            }
                        }
                        // Cron / datetime input
                        if show_cron_input {
                            div {
                                label { class: "form-label", "Cron Expression" }
                                input {
                                    class: "form-input",
                                    style: "width: 100%; font-family: var(--font-mono);",
                                    placeholder: "0 9 * * 1-5",
                                    value: "{new_cron}",
                                    oninput: move |e| new_cron.set(e.value()),
                                }
                                div { style: "font-size: 11px; color: var(--text-faint); margin-top: 4px;",
                                    "Format: min hour day month weekday"
                                }
                            }
                        } else {
                            div {
                                label { class: "form-label", "Run At (UTC)" }
                                input {
                                    class: "form-input",
                                    style: "width: 100%;",
                                    r#type: "datetime-local",
                                    value: "{new_run_at}",
                                    oninput: move |e| new_run_at.set(e.value()),
                                }
                            }
                        }
                    }
                    div { style: "display: flex; gap: 12px; margin-top: 24px; justify-content: flex-end;",
                        button {
                            class: "btn",
                            onclick: move |_| show_create.set(false),
                            "Cancel"
                        }
                        button {
                            class: "btn btn-primary",
                            disabled: *saving.read() || new_workflow.read().is_empty(),
                            onclick: {
                                let wf = new_workflow.clone();
                                let ver = new_version.clone();
                                let mode = schedule_mode.clone();
                                let cron = new_cron.clone();
                                let run_at = new_run_at.clone();
                                move |_| {
                                    saving.set(true);
                                    let wf_val = wf.read().clone();
                                    let ver_val = ver.read().clone();
                                    let cron_val = if *mode.read() == ScheduleMode::Cron { Some(cron.read().clone()) } else { None };
                                    let run_at_val = if *mode.read() == ScheduleMode::Once { Some(run_at.read().clone()) } else { None };
                                    spawn(async move {
                                        match api::create_schedule(&wf_val, &ver_val, cron_val.as_deref(), run_at_val.as_deref()).await {
                                            Ok(_) => {
                                                show_toast("Schedule created", ToastType::Success);
                                                saving.set(false);
                                                show_create.set(false);
                                            }
                                            Err(e) => {
                                                show_toast(format!("Failed: {}", e), ToastType::Error);
                                                saving.set(false);
                                            }
                                        }
                                    });
                                }
                            },
                            if *saving.read() { "Saving…" } else { "Create Schedule" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CronHint(expr: &'static str, desc: &'static str) -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 6px;",
            code { style: "font-size: 11px; background: rgba(99,102,241,0.08); color: #6366f1; padding: 2px 6px; border-radius: 4px; font-family: var(--font-mono);", "{expr}" }
            span { style: "font-size: 11px; color: var(--text-faint);", "{desc}" }
        }
    }
}
