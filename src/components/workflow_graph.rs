//! Node-based workflow graph visualization component.
//!
//! Renders workflows as an interactive node graph with:
//! - Glassmorphic service cards for actions
//! - Logic diamonds for decision points (IfElse)
//! - Animated connection lines between nodes

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct WorkflowGraphProps {
    pub injected_ast: serde_json::Value,
    #[props(default)]
    pub active_steps: Option<Vec<String>>,
}

// ── Helpers ──────────────────────────────────────────────────────

fn get_service_icon(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    if lower.contains("salesforce") {
        "☁"
    } else if lower.contains("hubspot") {
        "◎"
    } else if lower.contains("email") || lower.contains("notify") {
        "✉"
    } else if lower.contains("http") || lower.contains("webhook") {
        "⇄"
    } else if lower.contains("call_workflow") {
        "⟳"
    } else if lower.contains("complete") || lower.contains("done") {
        "✓"
    } else if lower.contains("fail") || lower.contains("reject") {
        "✕"
    } else {
        "◆"
    }
}

fn service_brand_class(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    if lower.contains("salesforce") {
        "salesforce"
    } else if lower.contains("hubspot") {
        "hubspot"
    } else {
        ""
    }
}

// ── Sub-components ───────────────────────────────────────────────

#[component]
fn StartNode(label: String) -> Element {
    rsx! {
        div { class: "vg-start-node",
            span { class: "vg-start-icon", "▶" }
            span { class: "vg-start-label", "{label}" }
        }
    }
}

#[component]
fn ActionNode(
    step_name: String,
    action_name: String,
    details: String,
    is_selected: bool,
    is_active: bool,
    on_click: EventHandler<String>,
) -> Element {
    let icon = get_service_icon(&action_name);
    let brand = service_brand_class(&action_name);
    let mut card_class = if is_selected { "vg-action-card selected" } else { "vg-action-card" }.to_string();
    if is_active {
        card_class.push_str(" vg-node-active");
    }

    rsx! {
        button {
            class: "{card_class}",
            onclick: {
                let name = step_name.clone();
                move |_| on_click.call(name.clone())
            },
            div { class: "vg-action-header",
                div { class: "vg-action-icon {brand}", "{icon}" }
                div { class: "vg-action-title", "{step_name}" }
            }
            if !action_name.is_empty() {
                div { class: "vg-action-type", "{action_name}" }
            }
            if !details.is_empty() {
                div { class: "vg-action-detail", "{details}" }
            }
        }
    }
}

#[component]
fn DecisionNode(
    step_name: String,
    condition: String,
    is_selected: bool,
    is_active: bool,
    on_click: EventHandler<String>,
) -> Element {
    let mut card_class = if is_selected { "vg-decision selected" } else { "vg-decision" }.to_string();
    if is_active {
        card_class.push_str(" vg-node-active");
    }

    rsx! {
        button {
            class: "{card_class}",
            onclick: {
                let name = step_name.clone();
                move |_| on_click.call(name.clone())
            },
            div { class: "vg-diamond-shape",
                div { class: "vg-diamond-inner",
                    span { "?" }
                }
            }
            div { class: "vg-decision-label", "{step_name}" }
        }
    }
}

#[component]
fn WireSegment(label: Option<String>, #[props(default)] is_active: bool) -> Element {
    let label_class = match label.as_deref() {
        Some("Yes") => "vg-wire-tag yes",
        Some("No") => "vg-wire-tag no",
        _ => "vg-wire-tag",
    };

    let wire_class = if is_active { "vg-wire active" } else { "vg-wire" };

    rsx! {
        div { class: "{wire_class}",
            div { class: "vg-wire-line" }
            if let Some(lbl) = &label {
                span { class: "{label_class}", "{lbl}" }
            }
        }
    }
}

// ── AST Walking ──────────────────────────────────────────────────

struct StepInfo {
    name: String,
    is_decision: bool,
    action_name: String,
    condition_summary: String,
    then_target: Option<String>,
    else_target: Option<String>,
    goto_target: Option<String>,
}

fn extract_step_info(ast: &serde_json::Value) -> (String, Vec<StepInfo>) {
    let start = ast.get("start_step")
        .and_then(|v| v.as_str())
        .unwrap_or("start")
        .to_string();

    let mut infos = Vec::new();

    if let Some(steps) = ast.get("steps").and_then(|v| v.as_array()) {
        for step in steps {
            let name = step.get("name").and_then(|v| v.as_str()).unwrap_or("?").to_string();
            let body = step.get("body").and_then(|v| v.as_array());

            let mut is_decision = false;
            let mut action_name = String::new();
            let mut condition_summary = String::new();
            let mut then_target: Option<String> = None;
            let mut else_target: Option<String> = None;
            let mut goto_target: Option<String> = None;

            if let Some(stmts) = body {
                for stmt in stmts {
                    if let Some(goto) = stmt.get("Goto") {
                        if let Some(t) = goto.get("target").and_then(|v| v.as_str()) {
                            goto_target = Some(t.to_string());
                        }
                    }
                    if let Some(action) = stmt.get("Action") {
                        if let Some(n) = action.get("name").and_then(|v| v.as_str()) {
                            action_name = n.to_string();
                        }
                    }
                    if let Some(ifelse) = stmt.get("IfElse") {
                        is_decision = true;
                        if let Some(cond) = ifelse.get("condition") {
                            condition_summary = cond.to_string();
                        }
                        then_target = ifelse.get("then_body")
                            .and_then(|b| b.as_array())
                            .and_then(|stmts| stmts.iter().find_map(|s|
                                s.get("Goto").and_then(|g| g.get("target").and_then(|t| t.as_str().map(String::from)))
                            ));
                        else_target = ifelse.get("else_body")
                            .and_then(|b| b.as_array())
                            .and_then(|stmts| stmts.iter().find_map(|s|
                                s.get("Goto").and_then(|g| g.get("target").and_then(|t| t.as_str().map(String::from)))
                            ));
                    }
                }
            }

            infos.push(StepInfo {
                name,
                is_decision,
                action_name,
                condition_summary,
                then_target,
                else_target,
                goto_target,
            });
        }
    }

    (start, infos)
}

// ── Main Component ───────────────────────────────────────────────

#[component]
pub fn WorkflowGraph(props: WorkflowGraphProps) -> Element {
    let mut selected = use_signal(|| Option::<String>::None);
    let active_steps = props.active_steps.clone().unwrap_or_default();
    let (start_step, steps) = extract_step_info(&props.injected_ast);

    if steps.is_empty() {
        return rsx! {
            div { class: "empty-state",
                div { class: "empty-state-icon", "?" }
                div { class: "empty-state-text", "No steps were found in the workflow graph." }
            }
        };
    }

    // Split steps into the "main spine" (following start → goto chain)
    // and any branching targets (then/else from decisions).
    let mut spine_order: Vec<usize> = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut current_name = start_step.clone();

    // Walk the main spine
    loop {
        if visited.contains(&current_name) {
            break;
        }
        visited.insert(current_name.clone());
        if let Some(idx) = steps.iter().position(|s| s.name == current_name) {
            spine_order.push(idx);
            let step = &steps[idx];
            if step.is_decision {
                // For decisions, follow the "then" branch as the spine
                if let Some(ref t) = step.then_target {
                    current_name = t.clone();
                } else {
                    break;
                }
            } else if let Some(ref t) = step.goto_target {
                current_name = t.clone();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Collect "else" branch targets that aren't on the main spine
    let mut else_branches: Vec<(String, usize)> = Vec::new(); // (from_decision_name, target_idx)
    for &idx in &spine_order {
        let step = &steps[idx];
        if step.is_decision {
            if let Some(ref else_t) = step.else_target {
                if let Some(else_idx) = steps.iter().position(|s| &s.name == else_t) {
                    if !spine_order.contains(&else_idx) {
                        else_branches.push((step.name.clone(), else_idx));
                    }
                }
            }
        }
    }

    rsx! {
        div { class: "vg-canvas",
            // ─── Main spine (horizontal row) ───
            div { class: "vg-spine",
                StartNode { label: start_step.clone() }

                for &idx in spine_order.iter() {
                    {
                        let step = &steps[idx];
                        let step_name = step.name.clone();
                        let is_sel = *selected.read() == Some(step_name.clone());
                        let is_active = active_steps.contains(&step_name);

                        rsx! {
                            WireSegment { label: None::<String>, is_active }

                            if step.is_decision {
                                div { class: "vg-decision-column",
                                    DecisionNode {
                                        step_name: step_name.clone(),
                                        condition: step.condition_summary.clone(),
                                        is_selected: is_sel,
                                        is_active,
                                        on_click: move |name: String| selected.set(Some(name)),
                                    }

                                    // Show branch labels below the diamond
                                    div { class: "vg-branch-labels",
                                        if step.then_target.is_some() {
                                            span { class: "vg-branch-tag yes", "Yes →" }
                                        }
                                        if step.else_target.is_some() {
                                            span { class: "vg-branch-tag no", "↓ No" }
                                        }
                                    }
                                }
                            } else {
                                ActionNode {
                                    step_name: step_name.clone(),
                                    action_name: step.action_name.clone(),
                                    details: String::new(),
                                    is_selected: is_sel,
                                    is_active,
                                    on_click: move |name: String| selected.set(Some(name)),
                                }
                            }
                        }
                    }
                }
            }

            // ─── Else branches (rendered below) ───
            if !else_branches.is_empty() {
                div { class: "vg-branch-row",
                    for (from_name, else_idx) in else_branches.iter() {
                        {
                            let step = &steps[*else_idx];
                            let step_name = step.name.clone();
                            let is_sel = *selected.read() == Some(step_name.clone());
                            let from = from_name.clone();

                            rsx! {
                                div { class: "vg-branch-group",
                                    div { class: "vg-branch-connector",
                                        div { class: "vg-branch-wire-vert" }
                                        span { class: "vg-branch-from", "from {from}" }
                                    }
                                    ActionNode {
                                        step_name: step_name.clone(),
                                        action_name: step.action_name.clone(),
                                        details: String::new(),
                                        is_selected: is_sel,
                                        is_active: active_steps.contains(&step_name.clone()),
                                        on_click: move |name: String| selected.set(Some(name)),
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ─── Detail panel for selected node ───
            if let Some(sel_name) = selected.read().as_ref() {
                {
                    let body_json = steps.iter()
                        .find(|s| &s.name == sel_name)
                        .map(|s| {
                            let mut info = serde_json::Map::new();
                            info.insert("step".into(), serde_json::Value::String(s.name.clone()));
                            if s.is_decision {
                                info.insert("type".into(), serde_json::Value::String("decision".into()));
                                info.insert("condition".into(), serde_json::Value::String(s.condition_summary.clone()));
                                if let Some(ref t) = s.then_target {
                                    info.insert("then_goto".into(), serde_json::Value::String(t.clone()));
                                }
                                if let Some(ref e) = s.else_target {
                                    info.insert("else_goto".into(), serde_json::Value::String(e.clone()));
                                }
                            } else {
                                info.insert("type".into(), serde_json::Value::String("action".into()));
                                if !s.action_name.is_empty() {
                                    info.insert("action".into(), serde_json::Value::String(s.action_name.clone()));
                                }
                                if let Some(ref t) = s.goto_target {
                                    info.insert("goto".into(), serde_json::Value::String(t.clone()));
                                }
                            }
                            serde_json::to_string_pretty(&serde_json::Value::Object(info)).unwrap_or_default()
                        })
                        .unwrap_or_default();

                    rsx! {
                        div { class: "vg-detail-panel",
                            div { class: "vg-detail-header",
                                span { class: "vg-detail-title", "{sel_name}" }
                                button {
                                    class: "vg-detail-close",
                                    onclick: move |_| selected.set(None),
                                    "✕"
                                }
                            }
                            pre { class: "code-block", "{body_json}" }
                        }
                    }
                }
            }
        }
    }
}
