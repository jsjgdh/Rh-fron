use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct NodeState {
    id: String,
    x: f64,
    y: f64,
    label: String,
    action: String,
    params: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
struct EdgeState {
    id: String,
    from: String,
    to: String,
    label: Option<String>,
}

#[component]
pub fn Builder() -> Element {
    let mut nodes = use_signal(|| vec![
        NodeState { 
            id: "start".to_string(), 
            x: 100.0, 
            y: 200.0, 
            label: "Start".to_string(), 
            action: "system_start".to_string(), 
            params: HashMap::new() 
        }
    ]);
    let edges = use_signal(|| Vec::<EdgeState>::new());
    let mut selected_node_id = use_signal(|| Option::<String>::None);
    let mut is_dragging = use_signal(|| Option::<String>::None);
    let mut drag_offset = use_signal(|| (0.0, 0.0));
    let mut node_counter = use_signal(|| 1);

    let add_node = move |name: String, action: String| {
        let current_count = *node_counter.read();
        let id = format!("node_{}", current_count);
        node_counter.set(current_count + 1);
        
        nodes.write().push(NodeState {
            id: id.clone(),
            x: 50.0,
            y: 50.0,
            label: name,
            action: action,
            params: HashMap::new(),
        });
        selected_node_id.set(Some(id));
    };

    let handle_mousedown = move |id: String, x: f64, y: f64, client_x: f64, client_y: f64| {
        selected_node_id.set(Some(id.clone()));
        is_dragging.set(Some(id));
        drag_offset.set((client_x - x, client_y - y));
    };

    let handle_mousemove = move |evt: MouseEvent| {
        if let Some(id) = is_dragging.read().clone() {
            let offset = drag_offset.read();
            let mut current_nodes = nodes.write();
            if let Some(node) = current_nodes.iter_mut().find(|n| n.id == id) {
                node.x = evt.client_coordinates().x - offset.0;
                node.y = evt.client_coordinates().y - offset.1;
            }
        }
    };

    let handle_mouseup = move |_| {
        is_dragging.set(None);
    };

    rsx! {
        div { 
            class: "builder-container fade-in",
            style: "display: flex; height: calc(100vh - 64px); background: var(--bg); overflow: hidden;",
            onmousemove: handle_mousemove,
            onmouseup: handle_mouseup,

            // Left Sidebar: Palette
            div { 
                class: "builder-palette",
                style: "width: 280px; border-right: 1px solid var(--border); background: var(--bg-card); padding: 24px; z-index: 10;",
                div { class: "label-caps", style: "margin-bottom: 24px;", "Logic Nodes" }
                
                PaletteItem { 
                    icon: "⬘", 
                    name: "Action Node".to_string(), 
                    desc: "External API or system call".to_string(),
                    on_click: {
                        let mut add = add_node.clone();
                        move |_| add("New Action".to_string(), "api_call".to_string())
                    }
                }
                PaletteItem { 
                    icon: "◇", 
                    name: "Condition".to_string(), 
                    desc: "Branching logic (If/Else)".to_string(),
                    on_click: {
                        let mut add = add_node.clone();
                        move |_| add("Check Condition".to_string(), "condition".to_string())
                    }
                }
                PaletteItem { 
                    icon: "⇥", 
                    name: "Goto".to_string(), 
                    desc: "Jump to another step".to_string(),
                    on_click: {
                        let mut add = add_node.clone();
                        move |_| add("Jump To".to_string(), "goto".to_string())
                    }
                }

                div { class: "label-caps", style: "margin-top: 40px; margin-bottom: 24px;", "Integrations" }
                PaletteItem { 
                    icon: "◎", name: "HubSpot".to_string(), desc: "CRM operations".to_string(),
                    on_click: {
                        let mut add = add_node.clone();
                        move |_| add("HubSpot Sync".to_string(), "hubspot_sync".to_string())
                    }
                }
                PaletteItem { 
                    icon: "☁", name: "Salesforce".to_string(), desc: "Cloud data sync".to_string(),
                    on_click: {
                        let mut add = add_node.clone();
                        move |_| add("Salesforce Push".to_string(), "salesforce_push".to_string())
                    }
                }
                PaletteItem { 
                    icon: "✉", name: "Email".to_string(), desc: "Notification relay".to_string(),
                    on_click: {
                        let mut add = add_node.clone();
                        move |_| add("Send Email".to_string(), "send_email".to_string())
                    }
                }
            }

            // Center: Canvas
            div { 
                class: "builder-canvas",
                style: "flex: 1; position: relative; overflow: auto; background-image: radial-gradient(var(--border) 1px, transparent 1px); background-size: 32px 32px;",
                
                svg {
                    width: "3000px",
                    height: "3000px",
                    style: "position: absolute; top: 0; left: 0;",

                    // Definitions for markers (arrows)
                    defs {
                        marker {
                            id: "arrowhead",
                            view_box: "0 0 10 10",
                            ref_x: "9",
                            ref_y: "5",
                            marker_width: "6",
                            marker_height: "6",
                            orient: "auto",
                            path { d: "M 0 0 L 10 5 L 0 10 z", fill: "var(--text-faint)" }
                        }
                    }

                    // Render Edges
                    for edge in edges.read().iter() {
                        {
                            let nodes_val = nodes.read();
                            let from_node = nodes_val.iter().find(|n| n.id == edge.from);
                            let to_node = nodes_val.iter().find(|n| n.id == edge.to);
                            
                            if let (Some(f), Some(t)) = (from_node, to_node) {
                                let start_x = f.x + 200.0;
                                let start_y = f.y + 40.0;
                                let end_x = t.x;
                                let end_y = t.y + 40.0;
                                let cp1x = start_x + (end_x - start_x) / 2.0;
                                let cp2x = start_x + (end_x - start_x) / 2.0;
                                
                                rsx! {
                                    path {
                                        d: "M {start_x} {start_y} C {cp1x} {start_y}, {cp2x} {end_y}, {end_x} {end_y}",
                                        stroke: "var(--text-faint)",
                                        stroke_width: "2",
                                        fill: "none",
                                        marker_end: "url(#arrowhead)"
                                    }
                                }
                            } else {
                                rsx! { g {} }
                            }
                        }
                    }

                    // Render Nodes
                    for node in nodes.read().iter() {
                        {
                            let n = node.clone();
                            let is_selected = selected_node_id.read().as_ref() == Some(&n.id);
                            let border_color = if is_selected { "var(--accent-primary)" } else { "var(--border-strong)" };
                            
                            rsx! {
                                foreignObject {
                                    key: "{n.id}",
                                    x: n.x,
                                    y: n.y,
                                    width: "200",
                                    height: "100",
                                    onmousedown: {
                                        let mut hmd = handle_mousedown.clone();
                                        let nid = n.id.clone();
                                        move |evt: MouseEvent| hmd(nid.clone(), n.x, n.y, evt.client_coordinates().x, evt.client_coordinates().y)
                                    },
                                    
                                    div { 
                                        class: "builder-node",
                                        style: "width: 100%; height: 80px; background: var(--bg-elevated); border: 2px solid {border_color}; border-radius: 12px; padding: 12px; cursor: move; box-shadow: var(--shadow-sm); display: flex; flex-direction: column; justify-content: center;",
                                        div { style: "font-weight: 700; font-size: 14px;", "{n.label}" }
                                        div { style: "font-size: 11px; color: var(--text-faint); margin-top: 4px;", "{n.action}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Right Sidebar: Properties
            div { 
                class: "builder-properties",
                style: "width: 320px; border-left: 1px solid var(--border); background: var(--bg-card); padding: 24px; z-index: 10;",
                div { class: "label-caps", style: "margin-bottom: 24px;", "Properties" }
                
                if let Some(id) = selected_node_id.read().clone() {
                    {
                        let nodes_val = nodes.read();
                        if let Some(node) = nodes_val.iter().find(|n| n.id == id) {
                            let n = node.clone();
                            rsx! {
                                div { class: "form-group",
                                    label { class: "form-label", "Node ID" }
                                    input { 
                                        class: "form-input", 
                                        readonly: true, 
                                        value: "{n.id}" 
                                    }
                                }
                                div { class: "form-group", style: "margin-top: 16px;",
                                    label { class: "form-label", "Label" }
                                    input { 
                                        class: "form-input", 
                                        value: "{n.label}",
                                        oninput: {
                                            let id_cloned = id.clone();
                                            move |evt: FormEvent| {
                                                let mut current_nodes = nodes.write();
                                                if let Some(node) = current_nodes.iter_mut().find(|n| n.id == id_cloned) {
                                                    node.label = evt.value();
                                                }
                                            }
                                        }
                                    }
                                }
                                div { class: "form-group", style: "margin-top: 16px;",
                                    label { class: "form-label", "Action Type" }
                                    select { 
                                        class: "form-input",
                                        value: "{n.action}",
                                        onchange: {
                                            let id_cloned = id.clone();
                                            move |evt: FormEvent| {
                                                let mut current_nodes = nodes.write();
                                                if let Some(node) = current_nodes.iter_mut().find(|n| n.id == id_cloned) {
                                                    node.action = evt.value();
                                                }
                                            }
                                        },
                                        option { value: "system_start", "Start" }
                                        option { value: "api_call", "API Call" }
                                        option { value: "condition", "If/Else Condition" }
                                        option { value: "goto", "Jump to Step" }
                                        option { value: "finish", "Terminate" }
                                    }
                                }
                            }
                        } else {
                            rsx! { div { style: "color: var(--text-faint); font-size: 13px;", "Node not found" } }
                        }
                    }
                } else {
                    div { style: "color: var(--text-faint); font-size: 13px; text-align: center; margin-top: 100px;",
                        "Select a node to edit its properties"
                    }
                }

                div { 
                    style: "margin-top: auto; padding-top: 40px;",
                    button { 
                        class: "btn btn-primary", 
                        style: "width: 100%;",
                        "Save Workflow" 
                    }
                    button { 
                        class: "btn btn-ghost", 
                        style: "width: 100%; margin-top: 12px;",
                        "Export RheLang" 
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct PaletteItemProps {
    icon: &'static str,
    name: String,
    desc: String,
    on_click: EventHandler<MouseEvent>,
}

#[component]
fn PaletteItem(props: PaletteItemProps) -> Element {
    rsx! {
        div { 
            class: "palette-item",
            style: "padding: 12px; border: 1px solid var(--border); border-radius: 8px; margin-bottom: 12px; cursor: pointer; background: var(--bg); transition: all 0.2s ease;",
            onclick: move |evt| props.on_click.call(evt),
            div { style: "display: flex; align-items: center; gap: 12px;",
                span { style: "font-size: 18px; color: var(--accent-primary);", "{props.icon}" }
                div {
                    div { style: "font-size: 13px; font-weight: 600;", "{props.name}" }
                    div { style: "font-size: 11px; color: var(--text-faint);", "{props.desc}" }
                }
            }
        }
    }
}
