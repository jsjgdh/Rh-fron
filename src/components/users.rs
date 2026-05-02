use dioxus::prelude::*;
use crate::api;
use crate::auth::Permission;
use crate::app::{show_toast, ToastType};

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct OrgMember {
    pub id: String,
    pub email: String,
    pub role: String,
    pub joined_at: String,
    pub status: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Main component
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn UserManagement() -> Element {
    let user_role = crate::api::get_user_role();
    let role_ref = user_role.as_deref();
    let can_manage = crate::auth::has_permission(role_ref, &Permission::UsersManage);
    let can_invite = crate::auth::has_permission(role_ref, &Permission::UsersInvite);

    let mut members = use_signal(|| Vec::<OrgMember>::new());
    let mut loading = use_signal(|| true);
    let mut invite_email = use_signal(|| String::new());
    let mut invite_role = use_signal(|| "Operator".to_string());
    let mut invite_loading = use_signal(|| false);
    let mut show_invite = use_signal(|| false);
    let mut confirm_remove_id = use_signal(|| None::<String>);
    let mut search = use_signal(|| String::new());

    // Load members on mount
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            match api::list_org_members().await {
                Ok(data) => {
                    members.set(data);
                    loading.set(false);
                }
                Err(e) => {
                    tracing::error!("Failed to load members: {}", e);
                    loading.set(false);
                }
            }
        });
    });

    let filtered_members: Vec<OrgMember> = members
        .read()
        .iter()
        .filter(|m| {
            let q = search.read().to_lowercase();
            q.is_empty() || m.email.to_lowercase().contains(&q) || m.role.to_lowercase().contains(&q)
        })
        .cloned()
        .collect();

    rsx! {
        div { class: "fade-in",
            // ── Header ─────────────────────────────────────────────────────
            div { style: "display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 40px;",
                div {
                    h1 { class: "page-title", "TEAM MANAGEMENT" }
                    p { style: "color: var(--text-faint); font-size: 14px; margin-top: 4px;",
                        "Manage organization members, roles, and access levels."
                    }
                }
                if can_invite {
                    button {
                        class: "btn btn-primary",
                        style: "height: 40px; padding: 0 20px; font-size: 13px;",
                        onclick: move |_| show_invite.set(true),
                        "＋ Invite Member"
                    }
                }
            }

            // ── Stats row ──────────────────────────────────────────────────
            div { class: "grid-4", style: "margin-bottom: 40px;",
                StatPill { label: "Total Members", value: "{members.read().len()}" }
                StatPill {
                    label: "Administrators",
                    value: "{members.read().iter().filter(|m| m.role.contains(\"Administrator\")).count()}"
                }
                StatPill {
                    label: "Active",
                    value: "{members.read().iter().filter(|m| m.status == \"active\").count()}"
                }
                StatPill { label: "Pending Invites", value: "0" }
            }

            // ── Search ─────────────────────────────────────────────────────
            div { style: "margin-bottom: 24px;",
                input {
                    class: "search-input",
                    style: "width: 100%; max-width: 420px; padding: 10px 16px; border-radius: 8px; border: 1px solid var(--border); background: var(--bg-card); color: var(--text-primary); font-size: 14px;",
                    placeholder: "Search by email or role…",
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }
            }

            // ── Members table ──────────────────────────────────────────────
            div { class: "card", style: "padding: 0; overflow: hidden;",
                if *loading.read() {
                    div { style: "padding: 60px; text-align: center;",
                        div { class: "spinner", style: "margin: 0 auto 16px;" }
                        div { style: "color: var(--text-faint); font-size: 14px;", "Loading team members…" }
                    }
                } else if filtered_members.is_empty() {
                    div { style: "padding: 60px; text-align: center; color: var(--text-faint);",
                        div { style: "font-size: 2rem; margin-bottom: 12px;", "👥" }
                        div { style: "font-size: 14px;", "No members found." }
                        if can_invite {
                            button {
                                class: "btn btn-primary",
                                style: "margin-top: 16px;",
                                onclick: move |_| show_invite.set(true),
                                "Invite your first member"
                            }
                        }
                    }
                } else {
                    table { style: "width: 100%; border-collapse: collapse;",
                        thead {
                            tr { style: "background: var(--bg-elevated); border-bottom: 1px solid var(--border);",
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase; letter-spacing: 0.05em;", "Member" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase; letter-spacing: 0.05em;", "Role" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase; letter-spacing: 0.05em;", "Status" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase; letter-spacing: 0.05em;", "Joined" }
                                if can_manage {
                                    th { style: "padding: 12px 20px; text-align: right; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase; letter-spacing: 0.05em;", "Actions" }
                                }
                            }
                        }
                        tbody {
                            for member in filtered_members.iter() {
                                {
                                    let m = member.clone();
                                    let m_remove = m.clone();
                                    rsx! {
                                        tr {
                                            key: "{m.id}",
                                            style: "border-bottom: 1px solid var(--border); transition: background 0.15s;",
                                            // Member email + avatar
                                            td { style: "padding: 16px 20px;",
                                                div { style: "display: flex; align-items: center; gap: 12px;",
                                                    div {
                                                        style: "width: 36px; height: 36px; border-radius: 50%; background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary, #8b5cf6)); display: flex; align-items: center; justify-content: center; color: white; font-size: 14px; font-weight: 700; flex-shrink: 0;",
                                                        "{m.email.chars().next().unwrap_or('?').to_uppercase()}"
                                                    }
                                                    div {
                                                        div { style: "font-size: 14px; font-weight: 600; color: var(--text-primary);", "{m.email}" }
                                                        div { style: "font-size: 12px; color: var(--text-faint); font-family: var(--font-mono);", "ID: {&m.id[..8.min(m.id.len())]}…" }
                                                    }
                                                }
                                            }
                                            // Role
                                            td { style: "padding: 16px 20px;",
                                                RoleBadge { role: m.role.clone() }
                                            }
                                            // Status
                                            td { style: "padding: 16px 20px;",
                                                span {
                                                    class: if m.status == "active" { "status-pill status-pill-success" } else { "status-pill status-pill-warning" },
                                                    style: "font-size: 10px;",
                                                    "{m.status}"
                                                }
                                            }
                                            // Joined
                                            td { style: "padding: 16px 20px; font-size: 13px; color: var(--text-faint);",
                                                "{m.joined_at.split('T').next().unwrap_or(\"\")}"
                                            }
                                            // Actions
                                            if can_manage {
                                                td { style: "padding: 16px 20px; text-align: right;",
                                                    div { style: "display: flex; justify-content: flex-end; gap: 8px;",
                                                        // Role change select
                                                        select {
                                                            style: "font-size: 12px; padding: 4px 8px; border-radius: 6px; border: 1px solid var(--border); background: var(--bg-card); color: var(--text-primary); cursor: pointer;",
                                                            value: "{m.role}",
                                                            onchange: {
                                                                let mid = m.id.clone();
                                                                move |e: Event<FormData>| {
                                                                    let new_role = e.value().clone();
                                                                    let mid2 = mid.clone();
                                                                    spawn(async move {
                                                                        match api::update_member_role(&mid2, &new_role).await {
                                                                            Ok(_) => show_toast(format!("Role updated to {}", new_role), ToastType::Success),
                                                                            Err(e) => show_toast(format!("Failed: {}", e), ToastType::Error),
                                                                        }
                                                                    });
                                                                }
                                                            },
                                                            option { value: "System Administrator", "System Administrator" }
                                                            option { value: "Policy Architect", "Policy Architect" }
                                                            option { value: "Operator", "Operator" }
                                                            option { value: "Auditor", "Auditor" }
                                                        }
                                                        button {
                                                            class: "btn btn-danger",
                                                            style: "font-size: 11px; padding: 4px 10px; border-radius: 6px;",
                                                            onclick: move |_| confirm_remove_id.set(Some(m_remove.id.clone())),
                                                            "Remove"
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
            }

            // ── Invite Modal ───────────────────────────────────────────────
            if *show_invite.read() {
                div { class: "modal-overlay", onclick: move |_| show_invite.set(false) }
                div {
                    class: "modal-content fade-in",
                    style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%,-50%); z-index: 1001; background: var(--bg-card); border: 1px solid var(--border); border-radius: 16px; padding: 32px; width: 480px; max-width: 90vw; box-shadow: 0 24px 64px rgba(0,0,0,0.4);",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 8px;", "Invite Team Member" }
                    p { style: "color: var(--text-faint); font-size: 13px; margin-bottom: 24px;",
                        "They'll receive an email invitation to join your organization."
                    }
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        div {
                            label { class: "form-label", "Email Address" }
                            input {
                                class: "form-input",
                                style: "width: 100%;",
                                r#type: "email",
                                placeholder: "colleague@company.com",
                                value: "{invite_email}",
                                oninput: move |e| invite_email.set(e.value()),
                            }
                        }
                        div {
                            label { class: "form-label", "Assign Role" }
                            select {
                                class: "form-input",
                                style: "width: 100%;",
                                value: "{invite_role}",
                                onchange: move |e| invite_role.set(e.value()),
                                option { value: "System Administrator", "System Administrator" }
                                option { value: "Policy Architect", "Policy Architect" }
                                option { value: "Operator", selected: true, "Operator" }
                                option { value: "Auditor", "Auditor" }
                            }
                        }
                    }
                    div { style: "display: flex; gap: 12px; margin-top: 24px; justify-content: flex-end;",
                        button {
                            class: "btn",
                            onclick: move |_| show_invite.set(false),
                            "Cancel"
                        }
                        button {
                            class: "btn btn-primary",
                            disabled: *invite_loading.read() || invite_email.read().is_empty(),
                            onclick: {
                                let email = invite_email.clone();
                                let role = invite_role.clone();
                                move |_| {
                                    let e = email.read().clone();
                                    let r = role.read().clone();
                                    invite_loading.set(true);
                                    spawn(async move {
                                        match api::invite_member(&e, &r).await {
                                            Ok(_) => {
                                                show_toast(format!("Invite sent to {}", e), ToastType::Success);
                                                invite_loading.set(false);
                                                show_invite.set(false);
                                            }
                                            Err(err) => {
                                                show_toast(format!("Failed: {}", err), ToastType::Error);
                                                invite_loading.set(false);
                                            }
                                        }
                                    });
                                }
                            },
                            if *invite_loading.read() { "Sending…" } else { "Send Invite" }
                        }
                    }
                }
            }

            // ── Remove Confirm Modal ───────────────────────────────────────
            if let Some(mid) = confirm_remove_id.read().clone() {
                {
                    let mid_action = mid.clone();
                    rsx! {
                        div { class: "modal-overlay", onclick: move |_| confirm_remove_id.set(None) }
                        div {
                            style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%,-50%); z-index: 1001; background: var(--bg-card); border: 1px solid var(--status-error); border-radius: 16px; padding: 32px; width: 420px; max-width: 90vw; box-shadow: 0 24px 64px rgba(0,0,0,0.4);",
                            div { style: "font-size: 2rem; margin-bottom: 12px;", "⚠️" }
                            h2 { style: "font-size: 1.25rem; margin-bottom: 8px;", "Remove Member" }
                            p { style: "color: var(--text-faint); font-size: 13px; margin-bottom: 24px;",
                                "This will immediately revoke their access to the organization. This action cannot be undone."
                            }
                            div { style: "display: flex; gap: 12px; justify-content: flex-end;",
                                button {
                                    class: "btn",
                                    onclick: move |_| confirm_remove_id.set(None),
                                    "Cancel"
                                }
                                button {
                                    class: "btn btn-danger",
                                    onclick: move |_| {
                                        let id = mid_action.clone();
                                        confirm_remove_id.set(None);
                                        spawn(async move {
                                            match api::remove_member(&id).await {
                                                Ok(_) => show_toast("Member removed.", ToastType::Success),
                                                Err(e) => show_toast(format!("Failed: {}", e), ToastType::Error),
                                            }
                                        });
                                    },
                                    "Remove Member"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Sub-components
// ─────────────────────────────────────────────────────────────────────────────

#[component]
fn StatPill(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "card stat-card",
            div { class: "stat-label", "{label}" }
            div { class: "stat-value", "{value}" }
        }
    }
}

#[component]
fn RoleBadge(role: String) -> Element {
    let (color, bg) = match role.as_str() {
        "System Administrator" => ("#f59e0b", "rgba(245,158,11,0.12)"),
        "Policy Architect"     => ("#6366f1", "rgba(99,102,241,0.12)"),
        "Operator"             => ("#22c55e", "rgba(34,197,94,0.12)"),
        "Auditor"              => ("#64748b", "rgba(100,116,139,0.12)"),
        _                      => ("#94a3b8", "rgba(148,163,184,0.12)"),
    };
    rsx! {
        span {
            style: "font-size: 11px; font-weight: 700; padding: 3px 10px; border-radius: 20px; color: {color}; background: {bg}; border: 1px solid {color}33; letter-spacing: 0.04em;",
            "{role}"
        }
    }
}
