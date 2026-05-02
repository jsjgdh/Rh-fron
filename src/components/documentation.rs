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

static USER_MANUAL: &str = r#"
# Rhexiom User Manual

## Table of Contents

1. Introduction
2. Getting Started
3. Dashboard Overview
4. Features & How to Use Them
5. Common Workflows
6. Tips & Best Practices
7. Troubleshooting
8. FAQ

---

## 1. Introduction

### What is Rhexiom?

Rhexiom is a Policy Orchestration Platform that allows you to transform business policies into automated, traceable, and auditable workflows. Whether you need to automate expense approvals, compliance checks, CRM data synchronization, or any business process, Rhexiom provides the tools to design, execute, and monitor these policies with complete transparency.

Think of Rhexiom as a digital policy lawyer — it takes your written policies, converts them into executable logic, and ensures every decision is recorded and explainable.

### Who is Rhexiom for?

Rhexiom serves multiple user types within an organization:

- **Policy Architects**: Design and build automated workflows using natural language or code
- **Operators**: Execute workflows and monitor their outcomes
- **Compliance Officers**: Track policy executions for audit and regulatory requirements
- **System Administrators**: Manage users, integrations, and platform settings

### Key Concepts

| Concept | Description |
|---------|-------------|
| **Workflow** | An automated process you define (e.g., "Approve expenses under $500") |
| **Execution** | A single run of a workflow |
| **RheLang** | The policy language Rhexiom uses to define workflow logic |
| **Artifact** | A saved version of a compiled workflow |
| **Integration** | Connection to external services (HubSpot, Salesforce, Email, etc.) |
| **Shadow Mode** | Test mode that traces execution without triggering real side effects |

---

## 2. Getting Started

### Creating Your Account

1. Navigate to the Rhexiom login page
2. Click "Create Account" or the sign-up link
3. Enter your email address and create a strong password
   - Passwords require: uppercase, lowercase, digit, special character
   - Minimum 8 characters
4. Click "Sign Up"
5. Check your email for a verification link and click it

### Logging In

1. Go to the Rhexiom login page
2. Enter your email and password
3. Click "Sign In"

**If MFA is enabled on your account:**
- After entering credentials, you'll be prompted for a 6-digit code
- Open your authenticator app (Google Authenticator, Authy, etc.)
- Enter the code currently displayed for Rhexiom
- Click "Verify"

**Using Backup Codes:**
- If you cannot access your authenticator app, click "Use Backup Code"
- Enter one of your saved backup codes
- Note: Each backup code can only be used once

### Basic Navigation

Once logged in, you'll see the Workspace Interface with these sections:

| Group | Contains |
|-------|----------|
| **Console** | Main dashboard with system overview |
| **Studio** | Tools for creating and editing workflows |
| **Forensics** | Execution history, analytics, and visualization |
| **Management** | Integrations, team, and settings |
| **Automation** | Scheduling, testing, and approvals |
| **Support** | Documentation and help resources |

### Understanding Your Role

Your role determines what you can see and do in Rhexiom:

| Role | Can Do |
|------|--------|
| **System Administrator** | Full access to all features |
| **Policy Architect** | Create/edit workflows, manage integrations, view executions |
| **Operator** | Run and view executions, view dashboards |
| **Auditor** | View-only access to all data and audit logs |

---

## 3. Dashboard Overview

The Console (Dashboard) is your command center. It provides:

### System Health Card
- **Status**: Shows if Rhexiom is operational
- **Active Policies**: Number of workflows currently deployed
- **Total Runs**: All-time execution count
- **Risk Index**: System-wide risk assessment score

### Live Execution Stream
- Real-time feed of workflow executions
- Shows: workflow name, status (running/success/failed), timestamp
- Click any execution to see full details

### System Alerts
- Important notifications about policy updates
- System warnings and maintenance notices
- Compliance-related alerts

---

## 4. Features & How to Use Them

### Logic Studio (Policy Creation)

Logic Studio is where you create new workflows.

#### Creating a Workflow from Natural Language

1. Click Studio in the sidebar
2. In the Source Logic text area, describe your policy in plain English
   - Example: "When an employee submits an expense report over $1000, require manager approval. Send notification to manager via email. If approved, process reimbursement. If rejected, notify employee."
3. Select LIVE or SHADOW mode
   - **LIVE**: Executes all actions including external service calls
   - **SHADOW**: Captures trace data only, no side effects
4. Click START COMPILATION
5. The AI will deconstruct your natural language into workflow components, generate RheLang code, and compile the workflow
6. On success, you'll be redirected to the workflow detail page
7. If errors occur, click REPAIR WITH AI to auto-fix issues

#### Creating a Workflow from RheLang Code

1. Click Studio in the sidebar
2. Write or paste your RheLang code directly in the text area
3. Select execution mode (LIVE or SHADOW)
4. Click START COMPILATION
5. Review the generated workflow

#### Importing from PDF

1. Click PDF IMPORT button
2. Select a PDF file containing policy documentation
3. Rhexiom extracts and displays the text
4. Review the extracted content
5. Click START COMPILATION to process

---

### Visual Workflow Builder

The Builder provides a drag-and-drop interface for creating workflows.

#### Building a Workflow

1. Click Builder in the sidebar
2. Add Nodes by clicking items in the Palette:
   - **Action Node**: Performs an API call or system operation
   - **Condition**: Creates branching logic (if/else)
   - **Goto**: Jumps to a specific step
3. Move Nodes by clicking and dragging them on the canvas
4. Connect Nodes by drawing edges (click and drag from node edge)
5. Select a Node by clicking it to edit properties
6. Save Workflow when complete
7. Export RheLang to view the generated code

---

### Workflow Templates

Templates are pre-built workflows you can use as starting points.

1. Click Templates in the sidebar
2. Browse available templates by category
3. Click on a template to preview
4. Click Use This Template
5. Customize the workflow as needed
6. Save as your own workflow

---

### Workflow Execution

1. Navigate to the workflow detail page
2. Click Run Workflow
3. Fill in the required input parameters
4. Choose execution mode (LIVE or SHADOW)
5. Click Execute
6. View the Execution Trace for detailed step-by-step results

---

### Execution History & Forensics

1. Click History in the sidebar
2. Browse the list of past executions
3. Use search and filters to find specific executions
4. Click any execution to view full details with tabs:
   - **Execution Trace**: Step-by-step breakdown with timestamps
   - **Decision Paths**: Branches taken during execution
   - **Actions**: Integration calls made
   - **Timing & Memory**: Performance metrics

---

### Integrations

Connect Rhexiom to your external services.

#### Supported Services

| Service | Capabilities |
|---------|--------------|
| **HubSpot** | Sync CRM data, update contacts, trigger workflows |
| **Salesforce** | Cloud data operations, record updates |
| **Email** | Send notifications, process incoming emails |
| **Slack** | Send messages to channels |

#### Configuring an Integration

1. Click Integrations in the sidebar
2. Click on a Service Connector card (HubSpot, Salesforce, Email, Slack)
3. Enter your API Key/Token
4. Click Save Configuration

---

### Approval Chains

Require human authorization before workflow completion.

1. Click Approvals in the sidebar
2. View pending requests
3. Click Review on a pending item
4. Optionally add a comment
5. Choose Approve or Reject

---

### Test Suite

Create automated tests for your workflows.

1. Click Test Suite in the sidebar
2. Click New Test
3. Enter test name, workflow, expected step, and input JSON
4. Click Save
5. Run tests individually or Run All Tests

---

### Schedule Manager

Automate workflow execution on a schedule.

1. Click Schedules in the sidebar
2. Click New Schedule
3. Select workflow and version
4. Choose schedule type (Cron or One-shot)
5. Set the schedule and inputs
6. Save and activate

---

## 5. Common Workflows

### Expense Approval Policy

1. Go to Studio
2. Describe: "When an expense report is submitted, check the amount. If under $1000, auto-approve and notify accounting. If $1000 or more, require manager approval."
3. Click START COMPILATION
4. Test in SHADOW mode
5. Deploy when ready

### CRM Sync

1. Configure HubSpot and Salesforce integrations
2. Create a sync workflow in Studio
3. Test in SHADOW mode
4. Activate in LIVE mode

---

## 6. Tips & Best Practices

### Policy Design
- Start Simple: Begin with basic workflows, add complexity gradually
- Use Descriptive Names: Name workflows and steps clearly
- Test Thoroughly: Always test in SHADOW mode first
- Version Everything: Don't overwrite — create new versions

### Security
- Enable MFA: Always enable multi-factor authentication
- Use Backup Codes: Store them securely when enabling MFA
- Limit Permissions: Give users only the access they need

---

## 7. Troubleshooting

| Problem | Solution |
|---------|----------|
| Login fails | Check email/password. Try password reset |
| MFA code not working | Check time sync on authenticator app. Try backup code |
| Workflow compilation error | Check RheLang code for typos |
| Execution fails | Check execution trace for specific error |
| Integration timeout | Verify external service is operational |

---

## 8. FAQ

**Q: What is Rhexiom?**
A: Rhexiom is a policy orchestration platform that transforms business policies into automated, traceable workflows.

**Q: What's the difference between LIVE and SHADOW mode?**
A: LIVE executes all actions including external service calls. SHADOW simulates execution without side effects.

**Q: How do I reset my password?**
A: On the login page, click "Forgot Password" and enter your email.

**Q: How do I invite a team member?**
A: Go to Team → Invite Member. Enter their email and select a role.
"#;

#[component]
pub fn UserManual() -> Element {
    rsx! {
        div { class: "fade-in user-manual",
            div { style: "max-width: 900px; margin: 0 auto;",
                h1 { class: "page-title", "User Manual" }
                p { style: "font-size: 1.2rem; color: var(--text-secondary); margin-bottom: 48px;",
                    "Complete guide to using Rhexiom. Learn how to create workflows, manage integrations, and automate your business policies."
                }

                div { class: "card", style: "margin-bottom: 24px;",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 16px; color: var(--accent-primary);", "1. Introduction" }
                    p { style: "line-height: 1.8; margin-bottom: 16px;", "Rhexiom is a Policy Orchestration Platform that allows you to transform business policies into automated, traceable, and auditable workflows." }
                    p { style: "line-height: 1.8; margin-bottom: 16px;", "Think of Rhexiom as a digital policy lawyer — it takes your written policies, converts them into executable logic, and ensures every decision is recorded and explainable." }
                    div { style: "margin-top: 20px;",
                        h3 { style: "font-size: 1.1rem; margin-bottom: 12px;", "Who is Rhexiom for?" }
                        ul { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Policy Architects: Design and build automated workflows" }
                            li { "Operators: Execute workflows and monitor their outcomes" }
                            li { "Compliance Officers: Track policy executions for audit" }
                            li { "System Administrators: Manage users and platform settings" }
                        }
                    }
                }

                div { class: "card", style: "margin-bottom: 24px;",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 16px; color: var(--accent-primary);", "2. Getting Started" }
                    div { style: "margin-bottom: 20px;",
                        h3 { style: "font-size: 1.1rem; margin-bottom: 8px;", "Creating Your Account" }
                        ol { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Navigate to the Rhexiom login page" }
                            li { "Click Create Account" }
                            li { "Enter your email and create a strong password" }
                            li { "Click Sign Up and verify your email" }
                        }
                    }
                    div { style: "margin-bottom: 20px;",
                        h3 { style: "font-size: 1.1rem; margin-bottom: 8px;", "Logging In" }
                        ol { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Enter your email and password" }
                            li { "If MFA is enabled, enter the 6-digit code from your authenticator app" }
                            li { "Click Sign In" }
                        }
                    }
                    div { style: "margin-bottom: 20px;",
                        h3 { style: "font-size: 1.1rem; margin-bottom: 8px;", "MFA Setup" }
                        p { style: "line-height: 1.8; color: var(--text-secondary);", "Go to Settings → Security → Enable MFA. Scan the QR code with your authenticator app and save your backup codes in a secure location." }
                    }
                }

                div { class: "card", style: "margin-bottom: 24px;",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 16px; color: var(--accent-primary);", "3. Dashboard Overview" }
                    p { style: "line-height: 1.8; margin-bottom: 16px;", "The Console is your command center showing:" }
                    ul { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                        li { strong { "System Health: " } "Operational status and health metrics" }
                        li { strong { "Active Policies: " } "Number of deployed workflows" }
                        li { strong { "Total Runs: " } "All-time execution count" }
                        li { strong { "Live Execution Stream: " } "Real-time feed of workflow executions" }
                        li { strong { "System Alerts: " } "Important notifications and warnings" }
                    }
                }

                div { class: "card", style: "margin-bottom: 24px;",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 16px; color: var(--accent-primary);", "4. Key Features" }

                    div { style: "margin-bottom: 24px;",
                        h3 { style: "font-size: 1.2rem; margin-bottom: 12px;", "Logic Studio" }
                        p { style: "line-height: 1.8; color: var(--text-secondary); margin-bottom: 12px;", "Create workflows using natural language or RheLang code." }
                        ol { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Go to Studio in the sidebar" }
                            li { "Describe your policy or write RheLang code" }
                            li { "Select LIVE or SHADOW mode" }
                            li { "Click START COMPILATION" }
                        }
                    }

                    div { style: "margin-bottom: 24px;",
                        h3 { style: "font-size: 1.2rem; margin-bottom: 12px;", "Visual Workflow Builder" }
                        p { style: "line-height: 1.8; color: var(--text-secondary); margin-bottom: 12px;", "Drag-and-drop interface for creating workflows." }
                        ol { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Go to Builder in the sidebar" }
                            li { "Click palette items to add nodes" }
                            li { "Drag nodes to position them" }
                            li { "Click a node to edit its properties" }
                            li { "Click Save Workflow when done" }
                        }
                    }

                    div { style: "margin-bottom: 24px;",
                        h3 { style: "font-size: 1.2rem; margin-bottom: 12px;", "Integrations" }
                        p { style: "line-height: 1.8; color: var(--text-secondary); margin-bottom: 12px;", "Connect to external services like HubSpot, Salesforce, Email, and Slack." }
                        ol { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Go to Integrations in the sidebar" }
                            li { "Click on a service connector" }
                            li { "Enter your API key/token" }
                            li { "Click Save Configuration" }
                        }
                    }

                    div { style: "margin-bottom: 24px;",
                        h3 { style: "font-size: 1.2rem; margin-bottom: 12px;", "Approval Chains" }
                        p { style: "line-height: 1.8; color: var(--text-secondary); margin-bottom: 12px;", "Require human authorization before workflow completion." }
                        ol { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Go to Approvals in the sidebar" }
                            li { "View pending approval requests" }
                            li { "Click Review on a pending item" }
                            li { "Add optional comment and Approve or Reject" }
                        }
                    }

                    div { style: "margin-bottom: 24px;",
                        h3 { style: "font-size: 1.2rem; margin-bottom: 12px;", "Test Suite" }
                        p { style: "line-height: 1.8; color: var(--text-secondary); margin-bottom: 12px;", "Create automated tests for your workflows." }
                        ol { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Go to Test Suite in the sidebar" }
                            li { "Click New Test" }
                            li { "Enter test name, workflow, and input JSON" }
                            li { "Click Run to execute the test" }
                        }
                    }

                    div { style: "margin-bottom: 24px;",
                        h3 { style: "font-size: 1.2rem; margin-bottom: 12px;", "Schedule Manager" }
                        p { style: "line-height: 1.8; color: var(--text-secondary); margin-bottom: 12px;", "Automate workflow execution on a schedule." }
                        ol { style: "line-height: 2; padding-left: 24px; color: var(--text-secondary);",
                            li { "Go to Schedules in the sidebar" }
                            li { "Click New Schedule" }
                            li { "Select workflow and schedule type (Cron or One-shot)" }
                            li { "Configure the schedule and inputs" }
                        }
                    }
                }

                div { class: "card", style: "margin-bottom: 24px;",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 16px; color: var(--accent-primary);", "5. Roles & Permissions" }
                    table { style: "width: 100%; border-collapse: collapse;",
                        thead {
                            tr { style: "border-bottom: 2px solid var(--border);",
                                th { style: "text-align: left; padding: 12px;", "Role" }
                                th { style: "text-align: left; padding: 12px;", "Can Do" }
                            }
                        }
                        tbody {
                            tr { style: "border-bottom: 1px solid var(--border-subtle);",
                                td { style: "padding: 10px;", "System Administrator" }
                                td { style: "padding: 10px; color: var(--text-secondary);", "Full access to all features" }
                            }
                            tr { style: "border-bottom: 1px solid var(--border-subtle);",
                                td { style: "padding: 10px;", "Policy Architect" }
                                td { style: "padding: 10px; color: var(--text-secondary);", "Create/edit workflows, manage integrations, view executions" }
                            }
                            tr { style: "border-bottom: 1px solid var(--border-subtle);",
                                td { style: "padding: 10px;", "Operator" }
                                td { style: "padding: 10px; color: var(--text-secondary);", "Run and view executions, view dashboards" }
                            }
                            tr { style: "border-bottom: 1px solid var(--border-subtle);",
                                td { style: "padding: 10px;", "Auditor" }
                                td { style: "padding: 10px; color: var(--text-secondary);", "View-only access to all data and audit logs" }
                            }
                        }
                    }
                }

                div { class: "card", style: "margin-bottom: 24px;",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 16px; color: var(--accent-primary);", "6. Troubleshooting" }
                    div { style: "display: grid; gap: 12px;",
                        div { style: "padding: 12px; background: var(--bg); border-radius: 8px;",
                            strong { style: "color: var(--text-primary);", "Login issues: " }
                            span { style: "color: var(--text-secondary);", "Check email/password. Try password reset. Use backup codes if MFA fails." }
                        }
                        div { style: "padding: 12px; background: var(--bg); border-radius: 8px;",
                            strong { style: "color: var(--text-primary);", "Workflow errors: " }
                            span { style: "color: var(--text-secondary);", "Check RheLang code for typos. Use REPAIR WITH AI to auto-fix." }
                        }
                        div { style: "padding: 12px; background: var(--bg); border-radius: 8px;",
                            strong { style: "color: var(--text-primary);", "Execution failures: " }
                            span { style: "color: var(--text-secondary);", "Check execution trace for specific errors. Verify integration status." }
                        }
                        div { style: "padding: 12px; background: var(--bg); border-radius: 8px;",
                            strong { style: "color: var(--text-primary);", "Integration timeouts: " }
                            span { style: "color: var(--text-secondary);", "Verify external service is operational. Check API keys are valid." }
                        }
                    }
                }

                div { class: "card",
                    h2 { style: "font-size: 1.5rem; margin-bottom: 16px; color: var(--accent-primary);", "7. FAQ" }
                    div { style: "margin-bottom: 16px;",
                        p { strong { "Q: What's the difference between LIVE and SHADOW mode?" } }
                        p { style: "color: var(--text-secondary);", "LIVE executes all actions including external service calls. SHADOW simulates execution without side effects." }
                    }
                    div { style: "margin-bottom: 16px;",
                        p { strong { "Q: How do I reset my password?" } }
                        p { style: "color: var(--text-secondary);", "Click Forgot Password on the login page and enter your email." }
                    }
                    div { style: "margin-bottom: 16px;",
                        p { strong { "Q: How do I invite a team member?" } }
                        p { style: "color: var(--text-secondary);", "Go to Team in the sidebar, click Invite Member, enter their email and select a role." }
                    }
                    div { style: "margin-bottom: 16px;",
                        p { strong { "Q: What integrations are supported?" } }
                        p { style: "color: var(--text-secondary);", "HubSpot, Salesforce, Email, Slack, Stripe, and Datadog." }
                    }
                }
            }
        }
    }
}