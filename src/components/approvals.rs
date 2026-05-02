use dioxus::prelude::*;
use crate::api;
use crate::app::{show_toast, ToastType};

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub id: String,
    pub workflow_name: String,
    pub version: String,
    pub requested_by: String,
    pub requested_at: String,
    pub input_summary: String,
    pub status: ApprovalStatus,
}

// ─────────────────────────────────────────────────────────────────────────────
// Approval Chains page
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Approvals() -> Element {
    let mut approvals = use_signal(|| Vec::<ApprovalRequest>::new());
    let mut loading = use_signal(|| true);
    let mut active_filter = use_signal(|| "pending"); // "pending" | "all"
    let mut review_id = use_signal(|| None::<String>);
    let mut review_comment = use_signal(|| String::new());
    let mut action_loading = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            loading.set(true);
            match api::list_pending_approvals().await {
                Ok(data) => { approvals.set(data); loading.set(false); }
                Err(e) => { tracing::warn!("approvals load error: {}", e); loading.set(false); }
            }
        });
    });

    let filtered: Vec<ApprovalRequest> = approvals.read().iter().filter(|a| {
        match *active_filter.read() {
            "pending" => a.status == ApprovalStatus::Pending,
            _ => true,
        }
    }).cloned().collect();

    let pending_count = approvals.read().iter().filter(|a| a.status == ApprovalStatus::Pending).count();

    // Pre-compute tab styles
    let pending_tab_border = if *active_filter.read() == "pending" { "var(--accent-primary)" } else { "transparent" };
    let pending_tab_color  = if *active_filter.read() == "pending" { "var(--accent-primary)" } else { "var(--text-faint)" };
    let all_tab_border = if *active_filter.read() == "all" { "var(--accent-primary)" } else { "transparent" };
    let all_tab_color  = if *active_filter.read() == "all" { "var(--accent-primary)" } else { "var(--text-faint)" };

    rsx! {
        div { class: "fade-in",
            // ── Header
            div { style: "display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 40px;",
                div {
                    div { style: "display: flex; align-items: center; gap: 12px;",
                        h1 { class: "page-title", "APPROVAL CHAINS" }
                        if pending_count > 0 {
                            span {
                                style: "font-size: 12px; font-weight: 800; background: var(--status-error); color: white; padding: 2px 8px; border-radius: 20px;",
                                "{pending_count}"
                            }
                        }
                    }
                    p { style: "color: var(--text-faint); font-size: 14px; margin-top: 4px;",
                        "Review and approve policy workflow executions that require human authorization."
                    }
                }
            }

            // ── Filter tabs
            div { style: "display: flex; gap: 0; margin-bottom: 24px; border-bottom: 1px solid var(--border);",
                button {
                    style: "padding: 10px 20px; font-size: 13px; font-weight: 600; border: none; border-bottom: 2px solid {pending_tab_border}; background: transparent; color: {pending_tab_color}; cursor: pointer;",
                    onclick: move |_| active_filter.set("pending"),
                    "Pending ({pending_count})"
                }
                button {
                    style: "padding: 10px 20px; font-size: 13px; font-weight: 600; border: none; border-bottom: 2px solid {all_tab_border}; background: transparent; color: {all_tab_color}; cursor: pointer;",
                    onclick: move |_| active_filter.set("all"),
                    "All ({approvals.read().len()})"
                }
            }

            // ── Approvals list
            if *loading.read() {
                div { style: "padding: 60px; text-align: center;",
                    div { class: "spinner", style: "margin: 0 auto 16px;" }
                    div { style: "color: var(--text-faint); font-size: 14px;", "Loading approvals…" }
                }
            } else if filtered.is_empty() {
                div { class: "card", style: "padding: 80px; text-align: center; color: var(--text-faint); border: 1px dashed var(--border-strong); background: transparent; box-shadow: none;",
                    div { style: "font-size: 3rem; margin-bottom: 16px;", "✅" }
                    div { style: "font-size: 1.1rem; font-weight: 600;", "No pending approvals" }
                    div { style: "font-size: 13px; margin-top: 8px;", "All workflows requiring approval have been actioned." }
                }
            } else {
                div { style: "display: flex; flex-direction: column; gap: 16px;",
                    for approval in filtered.iter() {
                        {
                            let a = approval.clone();
                            let aid = a.id.clone();
                            let border_color = match &a.status {
                                ApprovalStatus::Pending  => "var(--status-warning)",
                                ApprovalStatus::Approved => "var(--status-success)",
                                ApprovalStatus::Rejected => "var(--status-error)",
                            };
                            let badge_style = match &a.status {
                                ApprovalStatus::Pending  => "background: rgba(245,158,11,0.12); color: #f59e0b;",
                                ApprovalStatus::Approved => "background: rgba(34,197,94,0.12); color: #22c55e;",
                                ApprovalStatus::Rejected => "background: rgba(239,68,68,0.12); color: #ef4444;",
                            };
                            let badge_text = match &a.status {
                                ApprovalStatus::Pending  => "⏳ PENDING",
                                ApprovalStatus::Approved => "✓ APPROVED",
                                ApprovalStatus::Rejected => "✕ REJECTED",
                            };
                            let is_pending = a.status == ApprovalStatus::Pending;
                            rsx! {
                                div {
                                    key: "{a.id}",
                                    class: "card",
                                    style: "padding: 20px 24px; border-left: 4px solid {border_color};",
                                    div { style: "display: flex; align-items: flex-start; justify-content: space-between; gap: 16px;",
                                        div { style: "flex: 1;",
                                            div { style: "display: flex; align-items: center; gap: 10px; margin-bottom: 8px;",
                                                span { style: "font-size: 15px; font-weight: 700;", "{a.workflow_name}" }
                                                span { style: "font-size: 11px; font-family: var(--font-mono); color: var(--text-faint);", "{a.version}" }
                                                span {
                                                    style: "font-size: 10px; font-weight: 700; padding: 2px 8px; border-radius: 20px; {badge_style}",
                                                    "{badge_text}"
                                                }
                                            }
                                            div { style: "font-size: 12px; color: var(--text-faint); margin-bottom: 10px;",
                                                "Requested by "
                                                span { style: "font-weight: 600; color: var(--text-secondary);", "{a.requested_by}" }
                                                " on {a.requested_at.split('T').next().unwrap_or(\"\")}"
                                            }
                                            div {
                                                style: "font-size: 12px; font-family: var(--font-mono); background: var(--bg-elevated); padding: 8px 12px; border-radius: 6px; color: var(--text-secondary); border: 1px solid var(--border);",
                                                "{a.input_summary}"
                                            }
                                        }
                                        if is_pending {
                                            div { style: "display: flex; gap: 8px; flex-shrink: 0;",
                                                button {
                                                    class: "btn btn-ghost",
                                                    style: "font-size: 12px; padding: 6px 14px; border: 1px solid var(--border-strong);",
                                                    onclick: move |_| {
                                                        review_id.set(Some(aid.clone()));
                                                        review_comment.set(String::new());
                                                    },
                                                    "Review"
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


            // ── Review Modal
            if let Some(rid) = review_id.read().clone() {
                {
                    let rid_approve = rid.clone();
                    let rid_reject = rid.clone();
                    rsx! {
                        div { class: "modal-overlay", onclick: move |_| review_id.set(None) }
                        div {
                            style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%,-50%); z-index: 1001; background: var(--bg-card); border: 1px solid var(--border); border-radius: 16px; padding: 32px; width: 480px; max-width: 90vw; box-shadow: 0 24px 64px rgba(0,0,0,0.4);",
                            h2 { style: "font-size: 1.5rem; margin-bottom: 8px;", "Review Request" }
                            p { style: "color: var(--text-faint); font-size: 13px; margin-bottom: 24px;",
                                "Add an optional comment before approving or rejecting this execution."
                            }
                            div {
                                label { class: "form-label", "Comment (optional)" }
                                textarea {
                                    class: "form-input",
                                    style: "width: 100%; height: 100px; resize: none;",
                                    placeholder: "e.g. Approved — within budget threshold for Q2",
                                    value: "{review_comment}",
                                    oninput: move |e| review_comment.set(e.value()),
                                }
                            }
                            div { style: "display: flex; gap: 12px; margin-top: 20px;",
                                button {
                                    class: "btn",
                                    style: "flex: 1;",
                                    onclick: move |_| review_id.set(None),
                                    "Cancel"
                                }
                                button {
                                    class: "btn btn-danger",
                                    style: "flex: 1;",
                                    disabled: *action_loading.read(),
                                    onclick: {
                                        let id = rid_reject.clone();
                                        let comment = review_comment.clone();
                                        move |_| {
                                            action_loading.set(true);
                                            let id2 = id.clone();
                                            let c = comment.read().clone();
                                            spawn(async move {
                                                match api::reject_execution(&id2, &c).await {
                                                    Ok(_) => { show_toast("Execution rejected", ToastType::Warning); review_id.set(None); }
                                                    Err(e) => show_toast(format!("Error: {}", e), ToastType::Error),
                                                }
                                                action_loading.set(false);
                                            });
                                        }
                                    },
                                    "✕ Reject"
                                }
                                button {
                                    class: "btn btn-primary",
                                    style: "flex: 1;",
                                    disabled: *action_loading.read(),
                                    onclick: {
                                        let id = rid_approve.clone();
                                        let comment = review_comment.clone();
                                        move |_| {
                                            action_loading.set(true);
                                            let id2 = id.clone();
                                            let c = comment.read().clone();
                                            spawn(async move {
                                                match api::approve_execution(&id2, &c).await {
                                                    Ok(_) => { show_toast("Execution approved ✓", ToastType::Success); review_id.set(None); }
                                                    Err(e) => show_toast(format!("Error: {}", e), ToastType::Error),
                                                }
                                                action_loading.set(false);
                                            });
                                        }
                                    },
                                    "✓ Approve"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
