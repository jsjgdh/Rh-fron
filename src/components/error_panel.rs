use dioxus::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────
// Structured compiler error type
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CompileError {
    pub line: Option<u32>,
    pub col: Option<u32>,
    pub message: String,
    pub hint: Option<String>,
    pub error_code: Option<String>,
}

impl CompileError {
    /// Parse backend error strings like "line 12: unexpected token 'END'"
    pub fn from_str(raw: &str) -> Vec<CompileError> {
        let mut errors = Vec::new();
        for line_text in raw.lines() {
            let line_text = line_text.trim();
            if line_text.is_empty() { continue; }

            // Try "line N: message" format
            if let Some(rest) = line_text.strip_prefix("line ") {
                let mut parts = rest.splitn(2, ':');
                if let (Some(num), Some(msg)) = (parts.next(), parts.next()) {
                    if let Ok(n) = num.trim().parse::<u32>() {
                        errors.push(CompileError {
                            line: Some(n),
                            col: None,
                            message: msg.trim().to_string(),
                            hint: Self::hint_for(msg.trim()),
                            error_code: None,
                        });
                        continue;
                    }
                }
            }

            // Try "N:M: message" format
            let colon_parts: Vec<&str> = line_text.splitn(3, ':').collect();
            if colon_parts.len() >= 3 {
                if let (Ok(l), Ok(c)) = (colon_parts[0].trim().parse::<u32>(), colon_parts[1].trim().parse::<u32>()) {
                    errors.push(CompileError {
                        line: Some(l),
                        col: Some(c),
                        message: colon_parts[2].trim().to_string(),
                        hint: Self::hint_for(colon_parts[2].trim()),
                        error_code: None,
                    });
                    continue;
                }
            }

            // Generic error line
            errors.push(CompileError {
                line: None,
                col: None,
                message: line_text.to_string(),
                hint: Self::hint_for(line_text),
                error_code: None,
            });
        }
        if errors.is_empty() && !raw.trim().is_empty() {
            errors.push(CompileError {
                line: None,
                col: None,
                message: raw.trim().to_string(),
                hint: None,
                error_code: None,
            });
        }
        errors
    }

    fn hint_for(msg: &str) -> Option<String> {
        let m = msg.to_lowercase();
        if m.contains("unexpected token") || m.contains("expected") {
            Some("Check your syntax — a keyword, bracket, or semicolon may be missing.".into())
        } else if m.contains("undefined") || m.contains("not found") {
            Some("This identifier hasn't been declared. Check spelling or define it earlier.".into())
        } else if m.contains("type mismatch") || m.contains("cannot convert") {
            Some("The types don't match — ensure input/output types are consistent.".into())
        } else if m.contains("duplicate") {
            Some("A name is defined more than once in the same scope.".into())
        } else {
            None
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Error Panel component
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn ErrorPanel(errors: Vec<CompileError>) -> Element {
    if errors.is_empty() {
        return rsx! { div {} };
    }

    let error_count = errors.len();
    let plural = if error_count == 1 { "" } else { "s" };

    rsx! {
        div {
            style: "border-top: 1px solid var(--status-error); background: rgba(239,68,68,0.04); animation: fadeIn 0.2s ease;",
            div {
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 16px; border-bottom: 1px solid rgba(239,68,68,0.2); background: rgba(239,68,68,0.06);",
                span { style: "font-size: 12px; font-weight: 800; color: var(--status-error); text-transform: uppercase; letter-spacing: 0.06em;",
                    "✕  {error_count} Error{plural}"
                }
            }
            div { style: "max-height: 220px; overflow-y: auto; padding: 8px 0;",
                for (i, err) in errors.iter().enumerate() {
                    {
                        let e = err.clone();
                        let loc_label = match (e.line, e.col) {
                            (Some(l), Some(c)) => format!("L{l}:{c}"),
                            (Some(l), None)    => format!("L{l}"),
                            _                  => "ERR".to_string(),
                        };
                        let hint = e.hint.clone();
                        rsx! {
                            div {
                                key: "{i}",
                                style: "padding: 10px 16px; border-bottom: 1px solid rgba(239,68,68,0.08);",
                                div { style: "display: flex; align-items: baseline; gap: 8px;",
                                    span {
                                        style: "font-size: 10px; font-weight: 700; color: var(--status-error); font-family: var(--font-mono); background: rgba(239,68,68,0.12); padding: 1px 6px; border-radius: 3px; flex-shrink: 0;",
                                        "{loc_label}"
                                    }
                                    span { style: "font-size: 13px; font-weight: 600; color: var(--text-primary); font-family: var(--font-mono);",
                                        "{e.message}"
                                    }
                                }
                                if let Some(h) = hint {
                                    div { style: "margin-top: 4px; font-size: 11px; color: var(--text-faint); padding-left: 2px;",
                                        "💡 {h}"
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

