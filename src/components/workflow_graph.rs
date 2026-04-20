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

    rsx! {
        div { class: "vg-canvas-svg",
            svg { 
                view_box: "0 0 2000 1200", 
                width: "100%", 
                height: "100%",
                
                // 1. Draw Forensic Connections (Bezier Paths)
                for conn in connections {
                    {
                        let path_data = if conn.from_y == conn.to_y {
                            // Horizontal smooth line
                            format!("M {} {} L {} {}", conn.from_x, conn.from_y, conn.to_x, conn.to_y)
                        } else {
                            // Curved Bezier for branching
                            format!("M {} {} C {} {}, {} {}, {} {}", 
                                conn.from_x, conn.from_y,
                                conn.from_x, conn.to_y,
                                conn.from_x, conn.to_y,
                                conn.to_x, conn.to_y
                            )
                        };
                        rsx! {
                            g {
                                path {
                                    class: if conn.is_active { "forensic-path active" } else { "forensic-path" },
                                    d: "{path_data}"
                                }
                                if let Some(lbl) = &conn.label {
                                    text {
                                        x: (conn.from_x + 10.0),
                                        y: (conn.from_y + (conn.to_y - conn.from_y)/2.0 + 5.0),
                                        fill: "#94a3b8",
                                        font_size: "10",
                                        font_weight: "bold",
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
                        let is_sel = *selected_node.read() == Some(node.name.clone());
                        let is_active = active_steps.contains(&node.name);
                        let icon = get_icon(&node.action);
                        
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
                                    height: NODE_HEIGHT+40.0,
                                    
                                    div { class: if is_sel { "node-card selected" } else { "node-card" },
                                        div { class: "vg-action-header",
                                            div { class: "vg-action-icon", "{icon}" }
                                            div { class: "vg-action-title", "{node.name}" }
                                        }
                                        if !node.action.is_empty() {
                                            div { class: "vg-action-type", "{node.action}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 3. Start Indicator
                circle { cx: 40, cy: 150.0 + NODE_HEIGHT/2.0, r: 10, fill: "var(--brand-emerald)" }
                path { d: "M 40 {150.0 + NODE_HEIGHT/2.0} L 100 {150.0 + NODE_HEIGHT/2.0}", stroke: "rgba(255,255,255,0.2)", stroke_width: 2 }
            }
        }
    }
}
