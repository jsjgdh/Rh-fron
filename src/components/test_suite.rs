use dioxus::prelude::*;
use crate::api;
use crate::app::{show_toast, ToastType};

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct TestCase {
    pub id: String,
    pub name: String,
    pub workflow_name: String,
    pub version: String,
    pub input: serde_json::Value,
    pub expected_step: String,
    pub expected_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    Idle,
    Running,
    Pass,
    Fail(String),
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_id: String,
    pub status: TestStatus,
    pub actual_step: Option<String>,
    pub duration_ms: Option<u64>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Policy Test Suite
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn TestSuite() -> Element {
    let mut test_cases = use_signal(|| Vec::<TestCase>::new());
    let mut results: Signal<Vec<TestResult>> = use_signal(|| vec![]);
    let mut running = use_signal(|| false);
    let mut show_new = use_signal(|| false);
    let mut active_tab = use_signal(|| "suite"); // "suite" | "new"

    // Form state for new test
    let mut new_name = use_signal(|| String::new());
    let mut new_workflow = use_signal(|| String::new());
    let mut new_version = use_signal(|| "v1.0".to_string());
    let mut new_input_json = use_signal(|| "{\n  \n}".to_string());
    let mut new_expected_step = use_signal(|| String::new());
    let mut input_error = use_signal(|| None::<String>);

    let workflows = use_resource(|| async { api::list_workflows(None).await.ok().unwrap_or_default() });

    // Load test cases on mount
    use_effect(move || {
        spawn(async move {
            match api::list_test_cases().await {
                Ok(cases) => test_cases.set(cases),
                Err(e) => tracing::warn!("Could not load test cases: {}", e),
            }
        });
    });

    // Compute summary
    let pass_count = results.read().iter().filter(|r| r.status == TestStatus::Pass).count();
    let fail_count = results.read().iter().filter(|r| matches!(r.status, TestStatus::Fail(_))).count();
    let run_count = test_cases.read().len();

    rsx! {
        div { class: "fade-in",
            // ── Header
            div { style: "display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 32px;",
                div {
                    h1 { class: "page-title", "POLICY TEST SUITE" }
                    p { style: "color: var(--text-faint); font-size: 14px; margin-top: 4px;",
                        "Author, save, and run automated tests against your compiled workflows."
                    }
                }
                div { style: "display: flex; gap: 10px;",
                    button {
                        class: "btn",
                        style: "height: 40px; padding: 0 18px; font-size: 13px;",
                        onclick: move |_| {
                            active_tab.set("new");
                            show_new.set(true);
                        },
                        "＋ New Test"
                    }
                    button {
                        class: "btn btn-primary",
                        style: "height: 40px; padding: 0 18px; font-size: 13px;",
                        disabled: *running.read() || test_cases.read().is_empty(),
                        onclick: {
                            let cases = test_cases.clone();
                            move |_| {
                                running.set(true);
                                results.set(cases.read().iter().map(|c| TestResult {
                                    test_id: c.id.clone(),
                                    status: TestStatus::Running,
                                    actual_step: None,
                                    duration_ms: None,
                                }).collect());
                                let all = cases.read().clone();
                                spawn(async move {
                                    for tc in all.iter() {
                                        let start = js_sys::Date::now() as u64;
                                        let res = api::run_test_case(tc).await;
                                        let duration = (js_sys::Date::now() as u64).saturating_sub(start);
                                        let status = match res {
                                            Ok(r) if r.final_step == tc.expected_step => TestStatus::Pass,
                                            Ok(r) => TestStatus::Fail(format!(
                                                "Expected step '{}', got '{}'", tc.expected_step, r.final_step
                                            )),
                                            Err(e) => TestStatus::Fail(e),
                                        };
                                        let tid = tc.id.clone();
                                        let actual = None; // would come from run response
                                        results.write().iter_mut().find(|r| r.test_id == tid).map(|r| {
                                            r.status = status;
                                            r.duration_ms = Some(duration);
                                            r.actual_step = actual;
                                        });
                                    }
                                    running.set(false);
                                });
                            }
                        },
                        if *running.read() { "Running Tests…" } else { "▶  Run All Tests" }
                    }
                }
            }

            // ── Stats bar (only shown after a run)
            if !results.read().is_empty() {
                div { class: "grid-4", style: "margin-bottom: 32px;",
                    div { class: "card stat-card",
                        div { class: "stat-label", "Total Tests" }
                        div { class: "stat-value", "{run_count}" }
                    }
                    div { class: "card stat-card",
                        div { class: "stat-label", "Passed" }
                        div { class: "stat-value", style: "color: var(--status-success);", "{pass_count}" }
                    }
                    div { class: "card stat-card",
                        div { class: "stat-label", "Failed" }
                        div { class: "stat-value", style: "color: var(--status-error);", "{fail_count}" }
                    }
                    div { class: "card stat-card",
                        div { class: "stat-label", "Pass Rate" }
                        div { class: "stat-value",
                            if run_count > 0 {
                                "{(pass_count * 100) / run_count}%"
                            } else {
                                "—"
                            }
                        }
                    }
                }
            }

            // ── Test case table
            if test_cases.read().is_empty() {
                div { class: "card", style: "padding: 80px; text-align: center; color: var(--text-faint); border: 1px dashed var(--border-strong); background: transparent; box-shadow: none;",
                    div { style: "font-size: 3rem; margin-bottom: 16px;", "🧪" }
                    div { style: "font-size: 1.1rem; font-weight: 600; margin-bottom: 8px;", "No test cases yet" }
                    div { style: "font-size: 13px; margin-bottom: 24px;", "Create your first test to verify your workflow logic automatically." }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| show_new.set(true),
                        "Create First Test"
                    }
                }
            } else {
                div { class: "card", style: "padding: 0; overflow: hidden;",
                    table { style: "width: 100%; border-collapse: collapse;",
                        thead {
                            tr { style: "background: var(--bg-elevated); border-bottom: 1px solid var(--border);",
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Test Name" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Workflow / Version" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Expected Step" }
                                th { style: "padding: 12px 20px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Result" }
                                th { style: "padding: 12px 20px; text-align: right; font-size: 11px; font-weight: 700; color: var(--text-faint); text-transform: uppercase;", "Duration" }
                            }
                        }
                        tbody {
                            for tc in test_cases.read().clone().iter() {
                                {
                                    let t = tc.clone();
                                    let result = results.read().iter().find(|r| r.test_id == t.id).cloned();
                                    rsx! {
                                        tr {
                                            key: "{t.id}",
                                            style: "border-bottom: 1px solid var(--border);",
                                            td { style: "padding: 16px 20px; font-weight: 600; font-size: 14px;", "{t.name}" }
                                            td { style: "padding: 16px 20px; font-size: 13px; color: var(--text-faint);",
                                                span { style: "font-weight: 600; color: var(--text-primary);", "{t.workflow_name}" }
                                                " @ "
                                                span { style: "font-family: var(--font-mono); font-size: 11px;", "{t.version}" }
                                            }
                                            td { style: "padding: 16px 20px; font-size: 13px; font-family: var(--font-mono);", "{t.expected_step}" }
                                            td { style: "padding: 16px 20px;",
                                                match result.as_ref().map(|r| &r.status) {
                                                    None => rsx! { span { style: "color: var(--text-faint); font-size: 12px;", "—" } },
                                                    Some(TestStatus::Running) => rsx! { div { class: "spinner", style: "width: 16px; height: 16px; border-width: 2px;" } },
                                                    Some(TestStatus::Pass) => rsx! { span { class: "status-pill status-pill-success", style: "font-size: 10px;", "✓ PASS" } },
                                                    Some(TestStatus::Fail(msg)) => rsx! {
                                                        div {
                                                            span { class: "status-pill status-pill-danger", style: "font-size: 10px; display: block; margin-bottom: 4px;", "✕ FAIL" }
                                                            div { style: "font-size: 11px; color: var(--status-error); font-family: var(--font-mono);", "{msg}" }
                                                        }
                                                    },
                                                    Some(TestStatus::Idle) => rsx! { span { style: "color: var(--text-faint); font-size: 12px;", "—" } },
                                                }
                                            }
                                            td { style: "padding: 16px 20px; text-align: right; font-size: 13px; color: var(--text-faint); font-family: var(--font-mono);",
                                                if let Some(r) = result {
                                                    if let Some(d) = r.duration_ms { "{d}ms" } else { "—" }
                                                } else {
                                                    "—"
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

            // ── New Test Modal
            if *show_new.read() {
                div { class: "modal-overlay", onclick: move |_| show_new.set(false) }
                div {
                    style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%,-50%); z-index: 1001; background: var(--bg-card); border: 1px solid var(--border); border-radius: 16px; padding: 32px; width: 560px; max-width: 90vw; max-height: 90vh; overflow-y: auto; box-shadow: 0 24px 64px rgba(0,0,0,0.4);",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 24px;", "Create Test Case" }
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        div {
                            label { class: "form-label", "Test Name" }
                            input {
                                class: "form-input",
                                style: "width: 100%;",
                                placeholder: "e.g. Approve under $500",
                                value: "{new_name}",
                                oninput: move |e| new_name.set(e.value()),
                            }
                        }
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
                        div {
                            label { class: "form-label", "Input JSON" }
                            textarea {
                                class: "form-input",
                                style: "width: 100%; height: 120px; font-family: var(--font-mono); font-size: 12px; resize: vertical;",
                                value: "{new_input_json}",
                                spellcheck: false,
                                oninput: move |e| {
                                    input_error.set(None);
                                    new_input_json.set(e.value());
                                }
                            }
                            if let Some(err) = input_error.read().as_ref() {
                                div { style: "font-size: 11px; color: var(--status-error); margin-top: 4px;", "⚠ {err}" }
                            }
                        }
                        div {
                            label { class: "form-label", "Expected Final Step" }
                            input {
                                class: "form-input",
                                style: "width: 100%;",
                                placeholder: "e.g. approve",
                                value: "{new_expected_step}",
                                oninput: move |e| new_expected_step.set(e.value()),
                            }
                        }
                    }
                    div { style: "display: flex; gap: 12px; margin-top: 24px; justify-content: flex-end;",
                        button {
                            class: "btn",
                            onclick: move |_| show_new.set(false),
                            "Cancel"
                        }
                        button {
                            class: "btn btn-primary",
                            disabled: new_name.read().is_empty() || new_workflow.read().is_empty() || new_expected_step.read().is_empty(),
                            onclick: {
                                let name = new_name.clone();
                                let wf = new_workflow.clone();
                                let ver = new_version.clone();
                                let input_j = new_input_json.clone();
                                let step = new_expected_step.clone();
                                move |_| {
                                    let raw = input_j.read().clone();
                                    match serde_json::from_str::<serde_json::Value>(&raw) {
                                        Err(e) => {
                                            input_error.set(Some(format!("Invalid JSON: {}", e)));
                                        }
                                        Ok(input_val) => {
                                            let tc = TestCase {
                                                id: format!("tc-{}", js_sys::Date::now() as u64),
                                                name: name.read().clone(),
                                                workflow_name: wf.read().clone(),
                                                version: ver.read().clone(),
                                                input: input_val,
                                                expected_step: step.read().clone(),
                                                expected_actions: vec![],
                                            };
                                            let tc2 = tc.clone();
                                            spawn(async move {
                                                match api::save_test_case(&tc2).await {
                                                    Ok(_) => show_toast("Test case saved", ToastType::Success),
                                                    Err(e) => show_toast(format!("Failed: {}", e), ToastType::Error),
                                                }
                                            });
                                            test_cases.write().push(tc);
                                            show_new.set(false);
                                        }
                                    }
                                }
                            },
                            "Save Test Case"
                        }
                    }
                }
            }
        }
    }
}
