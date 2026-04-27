//! High-fidelity SVG Forensic Graph engine for Rhexiom Studio.
//! 
//! Renders natural language synthesized workflows as auditable SVG node-link diagrams.
//! Uses Bezier curves for forensic connectivity and glassmorphism for node depth.

use dioxus::prelude::*;
use serde_json::Value;

#[derive(PartialEq, Props, Clone)]
pub struct WorkflowGraphProps {
    pub injected_ast: Value,
    #[props(default)]
    pub active_steps: Option<Vec<String>>,
}

// ── Design Tokens ────────────────────────────────────────────────

const NODE_WIDTH: f64 = 200.0;
const NODE_HEIGHT: f64 = 80.0;
const DIAMOND_SIZE: f64 = 60.0;
const HORIZONTAL_GAP: f64 = 120.0;
const VERTICAL_GAP: f64 = 160.0;

fn get_icon(action: &str) -> &'static str {
    let lower = action.to_lowercase();
    if lower.contains("hubspot") { "◎" }
    else if lower.contains("salesforce") { "☁" }
    else if lower.contains("email") || lower.contains("notify") { "✉" }
    else if lower.contains("webhook") || lower.contains("http") { "⇄" }
    else { "◆" }
}

// ── Graph State ──────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct VisualNode {
    name: String,
    x: f64,
    y: f64,
    is_decision: bool,
    action: String,
    then_target: Option<String>,
    else_target: Option<String>,
    goto_target: Option<String>,
}

#[derive(Debug, Clone)]
struct Connection {
    from_x: f64,
    from_y: f64,
    to_x: f64,
    to_y: f64,
    label: Option<String>,
    is_active: bool,
}

// ── Main Engine ──────────────────────────────────────────────────

#[component]
pub fn WorkflowGraph(props: WorkflowGraphProps) -> Element {
    let mut selected_node = use_signal(|| Option::<String>::None);
    let active_steps = props.active_steps.clone().unwrap_or_default();

    // 1. Extract and Position Nodes
    let start_name = props.injected_ast.get("start_step")
        .and_then(|v| v.as_str())
        .unwrap_or("start")
        .to_string();
    
    let steps_array = props.injected_ast.get("steps").and_then(|v| v.as_array());
    
    let mut nodes = Vec::new();
    let mut connections = Vec::new();

    if let Some(steps) = steps_array {
        let mut curr_x = 100.0;
        let mut curr_y = 150.0;
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        queue.push_back((start_name.clone(), 100.0, 150.0));

        while let Some((name, x, y)) = queue.pop_front() {
            if visited.contains(&name) { continue; }
            visited.insert(name.clone());

            if let Some(step) = steps.iter().find(|s| s.get("name").and_then(|v| v.as_str()) == Some(&name)) {
                let mut is_decision = false;
                let mut action = String::new();
                let mut then_target = None;
                let mut else_target = None;
                let mut goto_target = None;

                if let Some(body) = step.get("body").and_then(|v| v.as_array()) {
                    for stmt in body {
                        if let Some(a) = stmt.get("Action") {
                            action = a.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        }
                        if let Some(g) = stmt.get("Goto") {
                            goto_target = g.get("target").and_then(|v| v.as_str()).map(String::from);
                        }
                        if let Some(i) = stmt.get("IfElse") {
                            is_decision = true;
                            then_target = i.get("then_body").and_then(|b| b.as_array()).and_then(|sts| sts.iter().find_map(|s| s.get("Goto").and_then(|g| g.get("target").and_then(|t| t.as_str().map(String::from)))));
                            else_target = i.get("else_body").and_then(|b| b.as_array()).and_then(|sts| sts.iter().find_map(|s| s.get("Goto").and_then(|g| g.get("target").and_then(|t| t.as_str().map(String::from)))));
                        }
                    }
                }

                nodes.push(VisualNode { name: name.clone(), x, y, is_decision, action, then_target: then_target.clone(), else_target: else_target.clone(), goto_target: goto_target.clone() });

                // Layout next nodes
                if let Some(t) = goto_target {
                    let next_x = x + NODE_WIDTH + HORIZONTAL_GAP;
                    connections.push(Connection { from_x: x + NODE_WIDTH, from_y: y + NODE_HEIGHT/2.0, to_x: next_x, to_y: y + NODE_HEIGHT/2.0, label: None, is_active: active_steps.contains(&name) });
                    queue.push_back((t, next_x, y));
                }
                if let Some(t) = then_target {
                    let next_x = x + NODE_WIDTH + HORIZONTAL_GAP;
                    connections.push(Connection { from_x: x + NODE_WIDTH, from_y: y + NODE_HEIGHT/2.0, to_x: next_x, to_y: y + NODE_HEIGHT/2.0, label: Some("Yes".into()), is_active: active_steps.contains(&name) });
                    queue.push_back((t, next_x, y));
                }
                if let Some(e) = else_target {
                    let next_y = y + NODE_HEIGHT + VERTICAL_GAP;
                    connections.push(Connection { from_x: x + NODE_WIDTH/2.0, from_y: y + NODE_HEIGHT, to_x: x + NODE_WIDTH/2.0, to_y: next_y, label: Some("No".into()), is_active: active_steps.contains(&name) });
                    queue.push_back((e, x, next_y));
                }
            }
        }
    }

    // 2. Compute viewBox to fit graph
    let (max_x, max_y) = nodes.iter().fold((0.0_f64, 0.0_f64), |(mx, my), n| {
        (mx.max(n.x + NODE_WIDTH + 120.0), my.max(n.y + NODE_HEIGHT + 160.0))
    });
    let vb_w = max_x.max(1200.0);
    let vb_h = max_y.max(720.0);

    let selected = selected_node.read().clone();
    let selected_details = selected.as_ref().and_then(|name| nodes.iter().find(|n| &n.name == name).cloned());

    rsx! {
        div { class: "vg-shell",
            div { class: "vg-toolbar",
                div { class: "label-caps", "visual flow" }
                div { class: "vg-legend",
                    span { class: "vg-legend-item", span { class: "vg-swatch vg-swatch-active" } "active" }
                    span { class: "vg-legend-item", span { class: "vg-swatch vg-swatch-decision" } "decision" }
                    span { class: "vg-legend-item", span { class: "vg-swatch vg-swatch-selected" } "selected" }
                }
            }

            div { class: "vg-canvas",
                svg {
                    view_box: "0 0 {vb_w} {vb_h}",
                    width: "100%",
                    height: "100%",

                    defs {
                        marker {
                            id: "vg-arrow",
                            view_box: "0 0 10 10",
                            ref_x: "9",
                            ref_y: "5",
                            marker_width: "8",
                            marker_height: "8",
                            orient: "auto-start-reverse",
                            path { d: "M 0 0 L 10 5 L 0 10 z", class: "forensic-arrow" }
                        }
                    }

                    // 1. Draw Forensic Connections (Bezier Paths + arrows)
                    for conn in connections {
                        {
                            let (c1x, c1y, c2x, c2y) = if (conn.from_y - conn.to_y).abs() < 0.01 {
                                let dx = ((conn.to_x - conn.from_x).abs() * 0.55).min(140.0).max(60.0);
                                (conn.from_x + dx, conn.from_y, conn.to_x - dx, conn.to_y)
                            } else {
                                let dy = ((conn.to_y - conn.from_y).abs() * 0.55).min(140.0).max(60.0);
                                (conn.from_x, conn.from_y + dy, conn.to_x, conn.to_y - dy)
                            };

                            let path_data = format!(
                                "M {} {} C {} {}, {} {}, {} {}",
                                conn.from_x, conn.from_y,
                                c1x, c1y,
                                c2x, c2y,
                                conn.to_x, conn.to_y
                            );

                            let label_x = (conn.from_x + conn.to_x) * 0.5;
                            let label_y = (conn.from_y + conn.to_y) * 0.5 - 8.0;

                            rsx! {
                                g {
                                    path {
                                        class: if conn.is_active { "forensic-path active" } else { "forensic-path" },
                                        d: "{path_data}",
                                        marker_end: "url(#vg-arrow)"
                                    }
                                    if let Some(lbl) = &conn.label {
                                        text {
                                            x: "{label_x}",
                                            y: "{label_y}",
                                            class: "vg-edge-label",
                                            "{lbl}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 2. Draw Nodes
                    for node in nodes {
                        {
                            let is_sel = selected.as_ref() == Some(&node.name);
                            let is_active = active_steps.contains(&node.name);
                            let icon = get_icon(&node.action);
                            let card_class = {
                                let mut c = String::from("node-card");
                                if node.is_decision { c.push_str(" decision"); }
                                if is_active { c.push_str(" active"); }
                                if is_sel { c.push_str(" selected"); }
                                c
                            };

                            rsx! {
                                g {
                                    class: "forensic-node",
                                    onclick: {
                                        let name = node.name.clone();
                                        move |_| selected_node.set(Some(name.clone()))
                                    },
                                    foreignObject {
                                        x: node.x,
                                        y: node.y,
                                        width: NODE_WIDTH,
                                        height: NODE_HEIGHT + 44.0,
                                        div { class: "{card_class}",
                                            div { class: "vg-action-header",
                                                div { class: "vg-action-icon", "{icon}" }
                                                div { style: "min-width: 0;",
                                                    div { class: "vg-action-title", "{node.name}" }
                                                    if !node.action.is_empty() {
                                                        div { class: "vg-action-type", "{node.action}" }
                                                    } else {
                                                        div { class: "vg-action-type faint", "state" }
                                                    }
                                                }
                                            }
                                            if node.is_decision {
                                                div { class: "vg-decision-hint",
                                                    span { class: "vg-decision-dot yes" } "then"
                                                    span { class: "vg-decision-dot no" } "else"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 3. Start Indicator
                    g { class: "vg-start",
                        circle { cx: 40, cy: 150.0 + NODE_HEIGHT/2.0, r: 10, class: "vg-start-dot" }
                        path { d: "M 40 {150.0 + NODE_HEIGHT/2.0} L 100 {150.0 + NODE_HEIGHT/2.0}", class: "vg-start-line" }
                        text { x: 18, y: 150.0 + NODE_HEIGHT/2.0 - 16.0, class: "vg-start-label", "start" }
                    }
                }

                if let Some(sel) = selected_details {
                    div { class: "vg-inspector",
                        div { class: "label-caps", "node inspector" }
                        div { class: "vg-inspector-title", "{sel.name}" }
                        if !sel.action.is_empty() {
                            div { class: "vg-inspector-row",
                                span { class: "vg-inspector-k", "action" }
                                span { class: "vg-inspector-v", "{sel.action}" }
                            }
                        }
                        if sel.is_decision {
                            div { class: "vg-inspector-row",
                                span { class: "vg-inspector-k", "branch" }
                                span { class: "vg-inspector-v", "if/else" }
                            }
                        }
                        div { class: "vg-inspector-row",
                            span { class: "vg-inspector-k", "position" }
                            span { class: "vg-inspector-v mono", "x:{sel.x as i64} y:{sel.y as i64}" }
                        }
                        button {
                            class: "btn btn-secondary",
                            style: "margin-top: 12px; width: 100%;",
                            onclick: move |_| selected_node.set(None),
                            "Clear selection"
                        }
                    }
                }
            }
        }
    }
}
