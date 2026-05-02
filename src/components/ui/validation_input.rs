//! Input validation utilities for forms with inline error display.

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ValidationInputProps {
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub error: Option<String>,
    pub input_type: String,
    pub on_input: EventHandler<String>,
    pub validate: Option<fn(&str) -> Option<String>>,
}

#[component]
pub fn ValidationInput(props: ValidationInputProps) -> Element {
    let mut touched = use_signal(|| false);

    let error = if *touched.read() {
        if let Some(validate) = props.validate.as_ref() {
            validate(&props.value)
        } else {
            None
        }
    } else {
        None
    };

    let display_error = error.or(props.error.clone());

    rsx! {
        div { class: "validation-input-group",
            label { class: "nav-label", style: "padding: 0; margin-bottom: 8px;", "{props.label}" }
            input {
                r#type: "{props.input_type}",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                style: if display_error.is_some() { "border-color: var(--status-error);" } else { "" },
                oninput: move |e| {
                    touched.set(true);
                    props.on_input.call(e.value())
                },
                onblur: move |_| touched.set(true)
            }
            if let Some(ref err) = display_error {
                div { style: "color: var(--status-error); font-size: 12px; margin-top: 4px;", "{err}" }
            }
        }
    }
}

pub fn validate_email(email: &str) -> Option<String> {
    if email.is_empty() {
        Some("Email is required".to_string())
    } else if !email.contains('@') {
        Some("Please enter a valid email address".to_string())
    } else {
        None
    }
}

pub fn validate_password(password: &str) -> Option<String> {
    if password.is_empty() {
        Some("Password is required".to_string())
    } else if password.len() < 8 {
        Some("Password must be at least 8 characters".to_string())
    } else {
        None
    }
}

pub fn validate_required(value: &str, field_name: &str) -> Option<String> {
    if value.trim().is_empty() {
        Some(format!("{} is required", field_name))
    } else {
        None
    }
}