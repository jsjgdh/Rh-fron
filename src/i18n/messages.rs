//! Frontend message catalog for the Rhexiom web application.
//!
//! This module provides consistent, user-facing messages across the UI.
//! All messages follow a standardized format with:
//! - Sentence case
//! - Professional, clear tone
//! - Actionable next steps where applicable

/// Message severity levels.
#[derive(Clone, Debug, PartialEq)]
pub enum MessageSeverity {
    /// Informational message
    Info,
    /// Success message
    Success,
    /// Warning message
    Warning,
    /// Error message
    Error,
}

/// A structured message with optional action.
#[derive(Clone, Debug)]
pub struct Message {
    /// The human-readable message text
    pub text: &'static str,
    /// The severity level
    pub severity: MessageSeverity,
    /// Optional suggested action for the user
    pub action: Option<&'static str>,
}

impl Message {
    /// Create a new message.
    pub const fn new(text: &'static str, severity: MessageSeverity) -> Self {
        Self {
            text,
            severity,
            action: None,
        }
    }

    /// Add a suggested action to the message.
    pub const fn with_action(mut self, action: &'static str) -> Self {
        self.action = Some(action);
        self
    }
}

/// Authentication-related messages.
pub mod auth {
    use super::{Message, MessageSeverity};

    // Success messages
    pub const LOGIN_SUCCESS: Message = Message::new("Welcome back.", MessageSeverity::Success);
    pub const LOGOUT_SUCCESS: Message = Message::new("You have been signed out.", MessageSeverity::Success);
    pub const SIGNUP_SUCCESS: Message = Message::new("Account created successfully. You can now sign in.", MessageSeverity::Success);
    pub const PASSWORD_CHANGED: Message = Message::new("Password updated successfully.", MessageSeverity::Success);

    // MFA messages
    pub const MFA_SETUP_SUCCESS: Message = Message::new("Two-factor authentication set up successfully. Save your backup codes.", MessageSeverity::Success);
    pub const MFA_ENABLED: Message = Message::new("Two-factor authentication is now enabled.", MessageSeverity::Success);
    pub const MFA_DISABLED: Message = Message::new("Two-factor authentication has been disabled.", MessageSeverity::Success);
    pub const MFA_VERIFIED: Message = Message::new("Verification code accepted.", MessageSeverity::Success);
    pub const MFA_BACKUP_CODES_GENERATED: Message = Message::new("Backup codes generated. Save these in a secure location.", MessageSeverity::Success);

    // Error messages
    pub const INVALID_CREDENTIALS: Message = Message::new(
        "Invalid email or password.",
        MessageSeverity::Error
    ).with_action("Please check your credentials and try again.");

    pub const SESSION_EXPIRED: Message = Message::new(
        "Your session has expired.",
        MessageSeverity::Error
    ).with_action("Please sign in again to continue.");

    pub const INVALID_TOKEN: Message = Message::new(
        "Your session has expired.",
        MessageSeverity::Error
    ).with_action("Please sign in again to continue.");

    pub const INVALID_MFA_CODE: Message = Message::new(
        "Invalid verification code.",
        MessageSeverity::Error
    ).with_action("Please enter the current code from your authenticator app.");

    pub const EMAIL_ALREADY_EXISTS: Message = Message::new(
        "An account with this email address already exists.",
        MessageSeverity::Error
    ).with_action("Sign in with your existing account or use a different email address.");

    pub const WEAK_PASSWORD: Message = Message::new(
        "Password does not meet security requirements.",
        MessageSeverity::Error
    ).with_action("Use at least 12 characters with a mix of letters, numbers, and symbols.");

    pub const INVALID_BACKUP_CODE: Message = Message::new(
        "Invalid backup code.",
        MessageSeverity::Error
    ).with_action("Please enter a valid backup code or use your authenticator app.");

    // Status messages
    pub const SIGNING_IN: &str = "Signing in...";
    pub const CREATING_ACCOUNT: &str = "Creating account...";
    pub const VERIFYING: &str = "Verifying...";
    pub const MFA_REQUIRED: &str = "Enter the code from your authenticator app";
}

/// Workflow-related messages.
pub mod workflow {
    use super::{Message, MessageSeverity};

    // Success messages
    pub const COMPILE_SUCCESS: Message = Message::new("Workflow compiled successfully.", MessageSeverity::Success);
    pub const SAVE_SUCCESS: Message = Message::new("Workflow saved.", MessageSeverity::Success);
    pub const DELETE_SUCCESS: Message = Message::new("Workflow deleted.", MessageSeverity::Success);
    pub const EXECUTE_SUCCESS: Message = Message::new("Workflow executed successfully.", MessageSeverity::Success);
    pub const DEPLOY_SUCCESS: Message = Message::new("Workflow deployed.", MessageSeverity::Success);
    pub const GENERATED_SUCCESS: Message = Message::new("Workflow generated from your description.", MessageSeverity::Success);

    // Error messages
    pub const COMPILE_FAILED: Message = Message::new(
        "Workflow compilation failed.",
        MessageSeverity::Error
    );

    pub const SYNTAX_ERROR: Message = Message::new(
        "Workflow contains syntax errors.",
        MessageSeverity::Error
    ).with_action("Review the error details and correct your workflow definition.");

    pub const WORKFLOW_NOT_FOUND: Message = Message::new(
        "Workflow not found.",
        MessageSeverity::Error
    ).with_action("Check the workflow name and version are correct.");

    pub const EXECUTION_FAILED: Message = Message::new(
        "Workflow execution failed.",
        MessageSeverity::Error
    );

    pub const INVALID_INPUT: Message = Message::new(
        "Invalid workflow input.",
        MessageSeverity::Error
    ).with_action("Check that all required inputs are provided and of the correct type.");

    pub const GENERATION_FAILED: Message = Message::new(
        "Failed to generate workflow.",
        MessageSeverity::Error
    ).with_action("Try providing more specific details about your workflow requirements.");

    // Status messages
    pub const COMPILING: &str = "Compiling...";
    pub const GENERATING: &str = "Generating workflow...";
    pub const EXECUTING: &str = "Executing...";
    pub const SAVING: &str = "Saving...";
    pub const LOADING: &str = "Loading...";
    pub const EXECUTION_PENDING: &str = "Pending approval";
    pub const EXECUTION_RUNNING: &str = "Running...";
    pub const EXECUTION_SUSPENDED: &str = "Suspended";
    pub const EXECUTION_COMPLETED: &str = "Completed";
    pub const EXECUTION_FAILED: &str = "Failed";
}

/// Organization-related messages.
pub mod organization {
    use super::{Message, MessageSeverity};

    // Success messages
    pub const CREATED: Message = Message::new("Organization created.", MessageSeverity::Success);
    pub const UPDATED: Message = Message::new("Organization settings updated.", MessageSeverity::Success);
    pub const MEMBER_ADDED: Message = Message::new("Member added.", MessageSeverity::Success);
    pub const MEMBER_REMOVED: Message = Message::new("Member removed.", MessageSeverity::Success);
    pub const INVITATION_SENT: Message = Message::new("Invitation sent.", MessageSeverity::Success);
    pub const INVITATION_ACCEPTED: Message = Message::new("Invitation accepted.", MessageSeverity::Success);

    // Error messages
    pub const ACCESS_DENIED: Message = Message::new(
        "You do not have access to this organization.",
        MessageSeverity::Error
    ).with_action("Contact your organization administrator for access.");

    pub const ADMIN_REQUIRED: Message = Message::new(
        "Administrator access required.",
        MessageSeverity::Error
    ).with_action("Contact your organization administrator.");

    pub const INVITATION_EXPIRED: Message = Message::new(
        "Invitation has expired.",
        MessageSeverity::Error
    ).with_action("Request a new invitation from your organization administrator.");
}

/// Integration-related messages.
pub mod integration {
    use super::{Message, MessageSeverity};

    // Success messages
    pub const CONNECTED: Message = Message::new("Integration connected successfully.", MessageSeverity::Success);
    pub const UPDATED: Message = Message::new("Integration settings updated.", MessageSeverity::Success);
    pub const DISCONNECTED: Message = Message::new("Integration disconnected.", MessageSeverity::Success);
    pub const TEST_SUCCESS: Message = Message::new("Connection test successful.", MessageSeverity::Success);

    // Error messages
    pub const CONNECTION_FAILED: Message = Message::new(
        "Failed to connect to integration.",
        MessageSeverity::Error
    ).with_action("Check your API key and network settings, then try again.");

    pub const INVALID_API_KEY: Message = Message::new(
        "Invalid API key.",
        MessageSeverity::Error
    ).with_action("Verify your API key in the integration settings.");
}

/// Execution-related messages.
pub mod execution {
    use super::{Message, MessageSeverity};

    // Success messages
    pub const RESUME_SUCCESS: Message = Message::new("Execution resumed.", MessageSeverity::Success);
    pub const APPROVE_SUCCESS: Message = Message::new("Execution approved.", MessageSeverity::Success);
    pub const CANCEL_SUCCESS: Message = Message::new("Execution cancelled.", MessageSeverity::Success);

    // Error messages
    pub const NOT_FOUND: Message = Message::new(
        "Execution not found.",
        MessageSeverity::Error
    ).with_action("Check the execution ID or view the execution history.");

    pub const NOT_SUSPENDED: Message = Message::new(
        "Execution cannot be resumed in its current state.",
        MessageSeverity::Error
    );
}

/// API Key-related messages.
pub mod api_key {
    use super::{Message, MessageSeverity};

    // Success messages
    pub const CREATED: Message = Message::new("API key created. Copy it now as it won't be shown again.", MessageSeverity::Success);
    pub const ROTATED: Message = Message::new("API key rotated successfully.", MessageSeverity::Success);
    pub const DELETED: Message = Message::new("API key deleted.", MessageSeverity::Success);
}

/// Webhook-related messages.
pub mod webhook {
    use super::{Message, MessageSeverity};

    // Success messages
    pub const CREATED: Message = Message::new("Webhook created.", MessageSeverity::Success);
    pub const DELETED: Message = Message::new("Webhook deleted.", MessageSeverity::Success);
    pub const COPIED: Message = Message::new("Webhook URL copied to clipboard.", MessageSeverity::Success);
}

/// System and general messages.
pub mod system {
    use super::{Message, MessageSeverity};

    pub const HEALTH_OK: Message = Message::new("System operational.", MessageSeverity::Success);

    pub const NETWORK_ERROR: Message = Message::new(
        "Network connection failed.",
        MessageSeverity::Error
    ).with_action("Check your internet connection and try again.");

    pub const SERVER_ERROR: Message = Message::new(
        "Server error occurred.",
        MessageSeverity::Error
    ).with_action("Please try again later or contact support.");

    pub const SERVICE_UNAVAILABLE: Message = Message::new(
        "Service temporarily unavailable.",
        MessageSeverity::Error
    ).with_action("Please try again in a few moments.");

    pub const VALIDATION_ERROR: Message = Message::new(
        "Please check your input and try again.",
        MessageSeverity::Error
    );

    pub const PERMISSION_DENIED: Message = Message::new(
        "You do not have permission to perform this action.",
        MessageSeverity::Error
    ).with_action("Contact your administrator if you need access.");
}

/// Form validation messages.
pub mod validation {
    use super::{Message, MessageSeverity};

    pub const REQUIRED_FIELD: Message = Message::new(
        "This field is required.",
        MessageSeverity::Error
    );

    pub const INVALID_FORMAT: Message = Message::new(
        "Invalid format.",
        MessageSeverity::Error
    );

    pub const INVALID_EMAIL: Message = Message::new(
        "Invalid email address.",
        MessageSeverity::Error
    );

    pub const PASSWORD_MISMATCH: Message = Message::new(
        "Passwords do not match.",
        MessageSeverity::Error
    );

    pub const TOO_SHORT: Message = Message::new(
        "Value is too short.",
        MessageSeverity::Error
    );

    pub const TOO_LONG: Message = Message::new(
        "Value is too long.",
        MessageSeverity::Error
    );
}

/// Empty state messages.
pub mod empty {
    pub const NO_WORKFLOWS: &str = "No workflows found. Create your first workflow to get started.";
    pub const NO_EXECUTIONS: &str = "No executions yet. Run a workflow to see results here.";
    pub const NO_INTEGRATIONS: &str = "No integrations connected. Connect an integration to extend your workflows.";
    pub const NO_WEBHOOKS: &str = "No webhooks configured. Create a webhook to trigger workflows externally.";
    pub const NO_AUDIT_LOGS: &str = "No audit logs available. Activity will appear here.";
    pub const NO_API_KEYS: &str = "No API keys. Create an API key to access the API programmatically.";
    pub const NO_MEMBERS: &str = "No members in this organization yet.";
    pub const NO_INVITATIONS: &str = "No pending invitations.";
    pub const NO_RESULTS: &str = "No results found.";
}

/// Button and action labels.
pub mod labels {
    pub const SIGN_IN: &str = "Sign in";
    pub const SIGN_OUT: &str = "Sign out";
    pub const CREATE_ACCOUNT: &str = "Create account";
    pub const SAVE: &str = "Save";
    pub const CANCEL: &str = "Cancel";
    pub const DELETE: &str = "Delete";
    pub const EDIT: &str = "Edit";
    pub const CREATE: &str = "Create";
    pub const COMPILE: &str = "Compile";
    pub const EXECUTE: &str = "Execute";
    pub const RUN: &str = "Run";
    pub const GENERATE: &str = "Generate";
    pub const UPLOAD: &str = "Upload";
    pub const DOWNLOAD: &str = "Download";
    pub const COPY: &str = "Copy";
    pub const CLOSE: &str = "Close";
    pub const BACK: &str = "Back";
    pub const NEXT: &str = "Next";
    pub const DONE: &str = "Done";
    pub const CONTINUE: &str = "Continue";
    pub const APPROVE: &str = "Approve";
    pub const REJECT: &str = "Reject";
    pub const RESUME: &str = "Resume";
    pub const CONNECT: &str = "Connect";
    pub const DISCONNECT: &str = "Disconnect";
    pub const TEST: &str = "Test connection";
    pub const REFRESH: &str = "Refresh";
    pub const SEARCH: &str = "Search";
    pub const FILTER: &str = "Filter";
    pub const CLEAR: &str = "Clear";
    pub const SHOW: &str = "Show";
    pub const HIDE: &str = "Hide";
    pub const ENABLE: &str = "Enable";
    pub const DISABLE: &str = "Disable";
    pub const CONFIGURE: &str = "Configure";
    pub const LEARN_MORE: &str = "Learn more";
}

/// Placeholder text.
pub mod placeholders {
    pub const EMAIL: &str = "Enter your email address";
    pub const PASSWORD: &str = "Enter your password";
    pub const CONFIRM_PASSWORD: &str = "Confirm your password";
    pub const WORKFLOW_NAME: &str = "Enter workflow name";
    pub const VERSION: &str = "Enter version (e.g., v1.0)";
    pub const DESCRIPTION: &str = "Describe your workflow...";
    pub const SEARCH: &str = "Search...";
    pub const API_KEY_NAME: &str = "Enter a name for this API key";
    pub const WEBHOOK_NAME: &str = "Enter a name for this webhook";
}

/// Tooltip/help text.
pub mod help {
    pub const WORKFLOW_INPUTS: &str = "Define the inputs your workflow expects";
    pub const WORKFLOW_STEPS: &str = "Define the steps and logic of your workflow";
    pub const EXECUTION_MODE: &str = "Live: Execute real actions. Shadow: Simulate without side effects.";
    pub const WEBHOOK_SECRET: &str = "Use this secret to verify webhook requests from Rhexiom";
    pub const API_KEY_SCOPES: &str = "Select the permissions this API key will have";
    pub const MFA_SETUP: &str = "Scan the QR code with your authenticator app";
    pub const BACKUP_CODES: &str = "Save these codes in a secure location. Each code can only be used once.";
    pub const ORGANIZATION_SLUG: &str = "Used in URLs. Use lowercase letters, numbers, and hyphens only.";
}

/// Section titles and headings.
pub mod sections {
    pub const WORKFLOWS: &str = "Workflows";
    pub const EXECUTIONS: &str = "Executions";
    pub const INTEGRATIONS: &str = "Integrations";
    pub const SETTINGS: &str = "Settings";
    pub const PROFILE: &str = "Profile";
    pub const SECURITY: &str = "Security";
    pub const ORGANIZATION: &str = "Organization";
    pub const MEMBERS: &str = "Members";
    pub const API_KEYS: &str = "API Keys";
    pub const WEBHOOKS: &str = "Webhooks";
    pub const AUDIT_LOGS: &str = "Audit Logs";
    pub const QUOTA: &str = "Usage & Quotas";
}

/// Error page messages.
pub mod errors {
    pub const PAGE_NOT_FOUND: &str = "Page not found";
    pub const PAGE_NOT_FOUND_DESCRIPTION: &str = "The page you are looking for does not exist or has been moved.";
    pub const UNAUTHORIZED: &str = "Access denied";
    pub const UNAUTHORIZED_DESCRIPTION: &str = "You do not have permission to view this page.";
    pub const SERVER_ERROR: &str = "Something went wrong";
    pub const SERVER_ERROR_DESCRIPTION: &str = "An unexpected error occurred. Please try again later.";
    pub const GO_HOME: &str = "Go to home";
    pub const GO_BACK: &str = "Go back";
}

/// Format a message for display, optionally including the action.
pub fn format_message(message: &Message) -> String {
    if let Some(action) = message.action {
        format!("{} {}", message.text, action)
    } else {
        message.text.to_string()
    }
}

/// Convert MessageSeverity to ToastType.
#[cfg(target_arch = "wasm32")]
pub fn severity_to_toast_type(severity: &MessageSeverity) -> crate::app::ToastType {
    match severity {
        MessageSeverity::Success => crate::app::ToastType::Success,
        MessageSeverity::Error => crate::app::ToastType::Error,
        MessageSeverity::Warning => crate::app::ToastType::Warning,
        MessageSeverity::Info => crate::app::ToastType::Info,
    }
}

/// Show a toast notification using the message catalog.
#[cfg(target_arch = "wasm32")]
pub fn show_message_toast(message: &Message) {
    let text = format_message(message);
    let toast_type = severity_to_toast_type(&message.severity);
    crate::app::show_toast(text, toast_type);
}
