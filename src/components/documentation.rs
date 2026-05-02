use dioxus::prelude::*;

static BULK_ENDPOINT: &str = "/api/workflows/{name}/bulk";
static EXECUTION_ENDPOINT: &str = "/api/executions/{id}";
static RESUME_ENDPOINT: &str = "/api/executions/{id}/resume";

static WORKFLOW_EXAMPLE: &str = r#"workflow ExpenseApproval v1.0 {
    input {
        amount: number
        receipt: boolean
        department: string
    }

    start check_amount

    step check_amount {
        if amount > 5000 {
            goto manager_review
        } else {
            goto auto_approve
        }
    }

    step manager_review {
        action request_manager_approval
        goto done
    }

    step auto_approve {
        action approve
        goto done
    }

    step done {
        action complete
    }
}"#;

static WORKFLOW_DECL: &str = r#"workflow [Name] v[Version] {
    // workflow body
}"#;

static WORKFLOW_EXAMPLE2: &str = r#"workflow PaymentProcessing v2.1.0 { ... }"#;

static INPUT_BLOCK: &str = r#"input {
    amount: number
    email: string
    is_approved: boolean
    tags: [string]
}"#;

static STEP_BLOCK: &str = r#"step step_name {
    // statements
    if condition { goto target_step }
    action action_name
    return "result"
}"#;

static LET_BINDING: &str = r#"step process {
    let subtotal = items.map(|i| i.price).sum()
    let discount = eligible ? subtotal * 0.1 : 0
    let total = subtotal - discount
    if total > 1000 { goto manager_approve }
}"#;

static METHOD_CHAIN: &str = r#"let domain = email.split("@")[1]
let cleaned = email.trim().to_lowercase()
let is_valid = domain.starts_with("acme") || domain.ends_with(".com")
let parts = full_name.split(" ")
let first_name = parts.first()
let last_name = parts.last()"#;

static ARRAY_LITERAL: &str = r#"let tiers = ["bronze", "silver", "gold", "platinum"]
let emails = ["a@example.com", "b@example.com"]
let numbers = [1, 2, 3, 4, 5]

// Array access with index (0-based)
if tiers[0] == "bronze" { goto bronze_tier }"#;

static CHAINED_COMPARE: &str = r#"// Valid: (x > 0) > -1 is evaluated left-to-right
if x > 0 > -1 { goto valid_range }

// Common use case: bounds checking
if 0 < amount < 1000 { goto auto_approve }"#;

static ACTIONS_CODE: &str = r#"action complete
action approve
action send_email { to: user_email, subject: "Hello", body: "Message" }
action hubspot_get_contact { email: customer_email }
action salesforce_create_task { lead_id: contact_id, subject: "Follow up" }"#;

static EMAIL_VALIDATION: &str = r#"workflow EmailValidation v1.0 {
    input { email: string }

    start validate

    step validate {
        let domain = email.split("@")[1]
        let cleaned = email.trim().to_lowercase()
        let is_corporate = ["acme.com", "corp.com"].contains(domain)

        if is_corporate {
            goto corporate_route
        } else {
            goto external_route
        }
    }

    step corporate_route {
        action notify { message: "Corporate email received" }
        goto done
    }

    step external_route {
        action log { entry: "External email detected" }
        goto done
    }

    step done {
        action complete
    }
}"#;

static DISCOUNT_CALC: &str = r#"workflow DiscountCalculator v1.0 {
    input {
        subtotal: number
        customer_tier: string
        is_loyalty_member: boolean
    }

    start calculate

    step calculate {
        let discount_pct = if customer_tier == "gold" {
            if subtotal > 1000 { 20 } else { 15 }
        } else if customer_tier == "silver" {
            if subtotal > 500 { 10 } else { 5 }
        } else {
            if is_loyalty_member { 5 } else { 0 }
        }

        let discount_amount = subtotal * (discount_pct / 100)
        let total = subtotal - discount_amount

        if total > 5000 {
            goto manager_approval
        } else {
            goto auto_approve
        }
    }

    step manager_approval {
        action request_manager_approval
        goto done
    }

    step auto_approve {
        action approve
        goto done
    }

    step done {
        action complete
    }
}"#;

#[component]
pub fn Documentation() -> Element {
    rsx! {
        div { class: "fade-in",
            h1 { class: "page-title", "RheLang Documentation" }
            p { style: "font-size: 1.2rem; color: var(--text-secondary); margin-bottom: 48px; max-width: 900px;",
                "RheLang v2.0 is a deterministic domain-specific language for modeling business logic as directed acyclic graphs. Every policy is compiled from RheLang into a sandboxed WASM artifact."
            }

            div { class: "section-title", "QUICK START" }
            div { class: "card",
                h3 { style: "margin-bottom: 16px; font-size: 1.5rem;", "Your First Workflow" }
                p { style: "line-height: 1.6; margin-bottom: 16px;", "A workflow defines the logic of your policy. It has inputs, steps, and actions." }
                pre {
                    class: "mono",
                    style: "padding: 20px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px; overflow-x: auto;",
                    "{WORKFLOW_EXAMPLE}"
                }
            }

            div { class: "section-title", "LANGUAGE STRUCTURE" }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Workflow Declaration" }
                p { style: "line-height: 1.6; margin-bottom: 12px;", "Every workflow starts with a declaration that defines its name and version." }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{WORKFLOW_DECL}"
                }
                p { style: "line-height: 1.6; margin-top: 12px;", "Example: " }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{WORKFLOW_EXAMPLE2}"
                }
            }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Input Block" }
                p { style: "line-height: 1.6; margin-bottom: 12px;", "The input block declares all external data that flows into the workflow. Inputs are immutable and strongly typed." }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{INPUT_BLOCK}"
                }
                p { style: "line-height: 1.6; margin-top: 12px; color: var(--text-secondary);", "Available types: number, string, boolean, [type] (array)" }
            }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Step Definition" }
                p { style: "line-height: 1.6; margin-bottom: 12px;", "Steps are the building blocks of a workflow. Each step contains statements that execute sequentially." }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{STEP_BLOCK}"
                }
            }

            div { class: "section-title", "DATA TYPES" }

            div { class: "card",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Type System" }
                table { style: "width: 100%; border-collapse: collapse; margin-bottom: 20px;",
                    thead {
                        tr { style: "border-bottom: 2px solid var(--border);",
                            th { style: "text-align: left; padding: 12px; font-size: 12px; text-transform: uppercase;", "Type" }
                            th { style: "text-align: left; padding: 12px; font-size: 12px; text-transform: uppercase;", "Description" }
                            th { style: "text-align: left; padding: 12px; font-size: 12px; text-transform: uppercase;", "Example" }
                        }
                    }
                    tbody {
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "number" }
                            td { style: "padding: 10px;", "64-bit floating point" }
                            td { class: "mono", style: "padding: 10px;", "42.5, -3.14, 1000" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "string" }
                            td { style: "padding: 10px;", "UTF-8 text" }
                            td { class: "mono", style: "padding: 10px;", "\"hello world\"" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "boolean" }
                            td { style: "padding: 10px;", "true or false" }
                            td { class: "mono", style: "padding: 10px;", "true, false" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "[type]" }
                            td { style: "padding: 10px;", "Array of type" }
                            td { class: "mono", style: "padding: 10px;", "[\"a\", \"b\", \"c\"]" }
                        }
                    }
                }
            }

            div { class: "section-title", "EXPRESSIONS" }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Let Bindings" }
                p { style: "line-height: 1.6; margin-bottom: 12px;", "Local variables store intermediate values. They are scoped to the step and can reference inputs or other variables." }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{LET_BINDING}"
                }
            }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Method Calls & Chaining" }
                p { style: "line-height: 1.6; margin-bottom: 12px;", "Objects support method chaining for transformations." }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{METHOD_CHAIN}"
                }
            }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Array Literals" }
                p { style: "line-height: 1.6; margin-bottom: 12px;", "Arrays can be created inline with literal syntax." }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{ARRAY_LITERAL}"
                }
            }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Chained Comparisons" }
                p { style: "line-height: 1.6; margin-bottom: 12px;", "Mathematical comparisons can be chained together." }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{CHAINED_COMPARE}"
                }
            }

            div { class: "section-title", "ACTIONS" }

            div { class: "card",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Integration Actions" }
                p { style: "line-height: 1.6; margin-bottom: 16px;", "Actions trigger side effects through integration providers." }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 13px;",
                    "{ACTIONS_CODE}"
                }
                p { style: "line-height: 1.6; margin-top: 16px; color: var(--text-secondary);",
                    "Available actions: complete, approve, fail, done, log, send_email, notify, request_approval, request_manager_approval, escalate, hubspot_get_contact, hubspot_create_deal, salesforce_query, salesforce_upsert_lead, slack_post_message, datadog_send_event, stripe_capture_payment"
                }
            }

            div { class: "section-title", "EXAMPLE WORKFLOWS" }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Email Validation & Routing" }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 12px; overflow-x: auto;",
                    "{EMAIL_VALIDATION}"
                }
            }

            div { class: "card", style: "margin-bottom: 24px;",
                h3 { style: "margin-bottom: 16px; font-size: 1.4rem;", "Expense with Discount Calculation" }
                pre {
                    class: "mono",
                    style: "padding: 16px; background: var(--bg); border: 1px solid var(--border); border-radius: 8px; font-size: 12px; overflow-x: auto;",
                    "{DISCOUNT_CALC}"
                }
            }

            div { class: "section-title", "API ENDPOINTS" }
            div { class: "card",
                table { style: "width: 100%; border-collapse: collapse;",
                    thead {
                        tr { style: "border-bottom: 2px solid var(--border);",
                            th { style: "text-align: left; padding: 12px; font-size: 12px; text-transform: uppercase;", "Method" }
                            th { style: "text-align: left; padding: 12px; font-size: 12px; text-transform: uppercase;", "Endpoint" }
                            th { style: "text-align: left; padding: 12px; font-size: 12px; text-transform: uppercase;", "Description" }
                        }
                    }
                    tbody {
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "POST" }
                            td { class: "mono", style: "padding: 10px;", "/api/workflows/compile" }
                            td { style: "padding: 10px;", "Compile RheLang to WASM" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "POST" }
                            td { class: "mono", style: "padding: 10px;", "/api/workflows/run" }
                            td { style: "padding: 10px;", "Execute a compiled workflow" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "POST" }
                            td { class: "mono", style: "padding: 10px;", "{BULK_ENDPOINT}" }
                            td { style: "padding: 10px;", "Bulk execution with concurrency" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "GET" }
                            td { class: "mono", style: "padding: 10px;", "/api/workflows" }
                            td { style: "padding: 10px;", "List all deployed workflows" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "GET" }
                            td { class: "mono", style: "padding: 10px;", "{EXECUTION_ENDPOINT}" }
                            td { style: "padding: 10px;", "Get execution details & trace" }
                        }
                        tr { style: "border-bottom: 1px solid var(--border-subtle);",
                            td { class: "mono", style: "padding: 10px; color: var(--accent-primary);", "POST" }
                            td { class: "mono", style: "padding: 10px;", "{RESUME_ENDPOINT}" }
                            td { style: "padding: 10px;", "Resume suspended execution" }
                        }
                    }
                }
            }

            div { class: "section-title", "INTEGRATIONS" }
            div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 24px;",
                DocCard {
                    title: "Email".to_string(),
                    desc: "Send emails via SMTP or APIs (SendGrid, Mailgun). Actions: send_email".to_string(),
                    tag: "stable".to_string()
                }
                DocCard {
                    title: "Salesforce".to_string(),
                    desc: "CRM integration for leads and contacts. Actions: get_lead, create_task, upsert_lead".to_string(),
                    tag: "stable".to_string()
                }
                DocCard {
                    title: "HubSpot".to_string(),
                    desc: "Marketing and sales automation. Actions: get_contact, create_deal".to_string(),
                    tag: "stable".to_string()
                }
                DocCard {
                    title: "Slack".to_string(),
                    desc: "Team notifications and alerts. Actions: post_message".to_string(),
                    tag: "stable".to_string()
                }
                DocCard {
                    title: "Datadog".to_string(),
                    desc: "Monitoring and observability events. Actions: send_event".to_string(),
                    tag: "stable".to_string()
                }
                DocCard {
                    title: "Stripe".to_string(),
                    desc: "Payment processing. Actions: capture_payment".to_string(),
                    tag: "stable".to_string()
                }
            }
        }
    }
}

#[component]
fn DocCard(title: String, desc: String, tag: String) -> Element {
    rsx! {
        div { class: "card",
            div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 12px;",
                h3 { style: "font-size: 1.25rem; margin: 0;", "{title}" }
                span { class: "status-pill", "{tag}" }
            }
            p { style: "font-size: 14px; color: var(--text-secondary); line-height: 1.6;", "{desc}" }
        }
    }
}