//! API client for communicating with the Rhexiom backend.
//!
//! Provides typed functions for all backend API calls.
//! Currently contains type definitions and placeholder implementations
//! that will use `reqwest` or `gloo-net` for actual HTTP calls.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct StatsResponse {
    pub active_workflows: usize,
    pub total_executions: usize,
    pub system_status: String,
}

/// Metadata for an integration action.
#[derive(Debug, Deserialize, Clone)]
pub struct ActionSchema {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, String>,
    pub required: Vec<String>,
}

/// Metadata for an integration provider.
#[derive(Debug, Deserialize, Clone)]
pub struct ProviderMetadata {
    pub name: String,
    pub description: String,
    pub actions: Vec<ActionSchema>,
}

/// Integration provider summary for list view.
#[derive(Debug, Deserialize, Clone)]
pub struct IntegrationSummary {
    pub name: String,
    pub description: String,
    pub actions: usize,
}

/// Dynamically detect the backend API base URL.
fn api_base() -> String {
    // Priority 1: Compile-time override
    if let Some(url) = option_env!("BACKEND_URL") {
        return url.to_string();
    }

    // Priority 2: Runtime detection (WASM)
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(hostname) = window.location().hostname() {
                // If we are on a custom domain or IP, assume Backend is on Port 3001
                // except if we are already on 3001 (Production Proxy Case)
                if let Ok(port) = window.location().port() {
                    if port == "3001" {
                        return "/api".to_string();
                    }
                }
                return format!("http://{}:3001/api", hostname);
            }
        }
    }
    
    // Default for local development
    "http://localhost:3001/api".to_string()
}

lazy_static::lazy_static! {
    static ref API_BASE: String = api_base();
}

/// Request to compile RheLang source.
#[derive(Debug, Serialize)]
pub struct CompileRequest {
    pub source: String,
}

/// Request to generate AI code constraints
#[derive(Debug, Serialize)]
pub struct GenerateRequest {
    pub prompt: String,
}

/// Response containing the generated RheLang source.
#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    pub source_code: String,
    pub error: Option<String>,
}

/// Response from compilation.
#[derive(Debug, Deserialize)]
pub struct CompileResponse {
    pub success: bool,
    pub workflow_name: String,
    pub version: String,
    pub warnings: Vec<String>,
    pub error: Option<String>,
    pub generated_rust: Option<String>,
}

/// Request to execute a workflow.
#[derive(Debug, Serialize)]
pub struct RunRequest {
    pub workflow_name: String,
    pub version: String,
    pub input: HashMap<String, serde_json::Value>,
    pub execution_mode: String,
}

/// A step generated locally for trace tracking
#[derive(Debug, Deserialize, Clone)]
pub struct TraceStepResponse {
    pub step_name: String,
    pub action: Option<String>,
    pub timestamp_us: u64,
}

/// Execution trace response format.
#[derive(Debug, Deserialize, Clone)]
pub struct ExecutionTraceResponse {
    pub workflow_name: String,
    pub version: String,
    pub steps: Vec<TraceStepResponse>,
    pub total_duration_us: u64,
}

/// Response from execution.
#[derive(Debug, Deserialize, Clone)]
pub struct RunResponse {
    pub success: bool,
    pub execution_id: Option<String>,
    pub status: String,
    pub final_step: String,
    pub actions: Vec<String>,
    pub trace: ExecutionTraceResponse,
    pub error: Option<String>,
}

/// Request to resume a suspended workflow.
#[derive(Debug, Serialize)]
pub struct ResumeRequest {
    pub additional_input: HashMap<String, serde_json::Value>,
}

/// Summary of a workflow.
#[derive(Debug, Deserialize)]
pub struct WorkflowSummary {
    pub name: String,
    pub versions: Vec<String>,
}

/// Get a pre-configured HTTP client safely wrapping origins.
fn client() -> reqwest::Client {
    reqwest::Client::new()
}

/// Helpers for session persistence using LocalStorage.
pub fn get_token() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                return storage.get_item("rhexiom_token").ok().flatten();
            }
        }
    }
    None
}

pub fn set_token(token: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("rhexiom_token", token);
            }
        }
    }
}

pub fn get_user_email() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                return storage.get_item("rhexiom_email").ok().flatten();
            }
        }
    }
    None
}

pub fn set_user_email(email: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("rhexiom_email", email);
            }
        }
    }
}

pub fn get_user_role() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                return storage.get_item("rhexiom_role").ok().flatten();
            }
        }
    }
    None
}

pub fn set_user_role(role: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("rhexiom_role", role);
            }
        }
    }
}

pub fn logout() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("rhexiom_token");
                let _ = storage.remove_item("rhexiom_email");
                let _ = storage.remove_item("rhexiom_role");
            }
        }
    }
}

/// Internal helper to build an authorized request.
fn authorized_request(method: reqwest::Method, url: String) -> reqwest::RequestBuilder {
    let mut builder = client().request(method, url);
    if let Some(token) = get_token() {
        builder = builder.header("Authorization", format!("Bearer {}", token));
    }
    builder
}

/// Request to compile RheLang source via engine layer.
pub async fn compile_workflow(source: &str) -> Result<CompileResponse, String> {
    let req = CompileRequest {
        source: source.to_string(),
    };
    let res = authorized_request(reqwest::Method::POST, format!("{}/workflows/compile", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<CompileResponse>()
            .await
            .map_err(|e| format!("Failed decoding response payload: {}", e))
    } else {
        Err(format!(
            "Compilation request failed with status: {}",
            res.status()
        ))
    }
}

/// Request to execute an instantiated workflow on the target OS bounds.
pub async fn run_workflow(req: &RunRequest) -> Result<RunResponse, String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/workflows/run", &*API_BASE))
        .json(req)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<RunResponse>()
            .await
            .map_err(|e| format!("Failed to parse execution JSON: {}", e))
    } else {
        Err(format!(
            "Execution request failed with status: {}",
            res.status()
        ))
    }
}

/// Fetch list of deployable workflows natively.
pub async fn list_workflows() -> Result<Vec<WorkflowSummary>, String> {
    #[derive(Deserialize)]
    struct Wrapper {
        workflows: Vec<WorkflowSummary>,
    }
    let res = authorized_request(reqwest::Method::GET, format!("{}/workflows", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        let wrapper = res
            .json::<Wrapper>()
            .await
            .map_err(|e| format!("Decoding failed: {}", e))?;
        Ok(wrapper.workflows)
    } else {
        Err(format!("Failed to list workflows: {}", res.status()))
    }
}

/// Resume a suspended execution with fresh inputs.
pub async fn resume_workflow(
    execution_id: &str,
    req: &ResumeRequest,
) -> Result<RunResponse, String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/executions/{}/resume", &*API_BASE, execution_id))
        .json(req)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<RunResponse>()
            .await
            .map_err(|e| format!("Failed to parse execution JSON: {}", e))
    } else {
        Err(format!(
            "Resumption request failed with status: {}",
            res.status()
        ))
    }
}

/// Fetch granular details for visualization endpoints safely.
pub async fn get_workflow_detail(name: &str, version: &str) -> Result<serde_json::Value, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/workflows/{}/{}", &*API_BASE, name, version))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse workflow details: {}", e))
    } else {
        Err(format!(
            "Failed pulling workflow detail payload: {}",
            res.status()
        ))
    }
}

/// Request AI to automatically generate RheLang workflow using the natural language prompt.
pub async fn generate_workflow(prompt: &str) -> Result<GenerateResponse, String> {
    let req = GenerateRequest {
        prompt: prompt.to_string(),
    };
    let res = authorized_request(reqwest::Method::POST, format!("{}/workflows/generate", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<GenerateResponse>()
            .await
            .map_err(|e| format!("Failed parsing generation payload: {}", e))
    } else {
        Err(format!(
            "Generation request failed natively with HTTP {}",
            res.status()
        ))
    }
}

/// Response from PDF extraction containing both raw text and deconstructed workflows.
#[derive(Debug, Deserialize, Clone)]
pub struct PdfExtractResponse {
    pub raw_text: String,
    pub deconstructed_workflows: String,
}

/// Response from deconstructing policy text into workflow intents.
#[derive(Debug, Deserialize)]
pub struct DeconstructResponse {
    pub deconstructed_workflows: String,
    pub error: Option<String>,
}

/// Request to deconstruct policy text.
#[derive(Debug, Serialize)]
pub struct DeconstructRequest {
    pub policy_text: String,
}

/// Extract text from a PDF document and deconstruct into workflow intents.
pub async fn extract_pdf(bytes: Vec<u8>) -> Result<PdfExtractResponse, String> {
    let form = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(bytes).file_name("policy.pdf"),
    );

    let res = authorized_request(reqwest::Method::POST, format!("{}/workflows/extract-pdf", &*API_BASE))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<PdfExtractResponse>()
            .await
            .map_err(|e| format!("Failed to parse PDF extraction result: {}", e))
    } else {
        Err(format!("PDF extraction failed with HTTP {}", res.status()))
    }
}

/// Deconstruct policy text into workflow intents.
pub async fn deconstruct_policy(policy_text: &str) -> Result<DeconstructResponse, String> {
    let req = DeconstructRequest {
        policy_text: policy_text.to_string(),
    };
    let res = authorized_request(reqwest::Method::POST, format!("{}/workflows/deconstruct", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<DeconstructResponse>()
            .await
            .map_err(|e| format!("Failed parsing deconstruction response: {}", e))
    } else {
        Err(format!(
            "Deconstruction request failed with HTTP {}",
            res.status()
        ))
    }
}

/// Get dashboard stats dynamically.
pub async fn get_stats() -> Result<StatsResponse, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/stats", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to backend: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Server returned HTTP {}", res.status()));
    }

    res.json::<StatsResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

/// Retrieve the full details for a specific execution.
pub async fn get_execution_detail(execution_id: &str) -> Result<serde_json::Value, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/executions/{}", &*API_BASE, execution_id))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse execution JSON: {}", e))
    } else {
        Err(format!("Execution request failed with status: {}", res.status()))
    }
}

/// Run a what-if simulation for an execution.
pub async fn simulate_execution(execution_id: &str) -> Result<serde_json::Value, String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/executions/{}/simulate", &*API_BASE, execution_id))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse simulation result: {}", e))
    } else {
        Err(format!("Simulation failed with status: {}", res.status()))
    }
}

/// List available integration services with structured data.
pub async fn get_integrations() -> Result<Vec<IntegrationSummary>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/integrations", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<Vec<IntegrationSummary>>().await.map_err(|e| format!("Failed to parse integrations: {}", e))
    } else {
        Err(format!("Integrations request failed with HTTP {}", res.status()))
    }
}

/// Get action schemas for all providers.
pub async fn get_action_schemas() -> Result<Vec<ProviderMetadata>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/actions/schema", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<Vec<ProviderMetadata>>().await.map_err(|e| format!("Failed to parse action schemas: {}", e))
    } else {
        Err(format!("Action schemas request failed with HTTP {}", res.status()))
    }
}

/// Update integration credentials and configuration.
pub async fn update_integration(name: &str, api_key: &str, config: serde_json::Value) -> Result<(), String> {
    let payload = serde_json::json!({
        "name": name,
        "api_key": api_key,
        "config": config,
    });

    let res = authorized_request(reqwest::Method::POST, format!("{}/integrations", &*API_BASE))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Integration update failed with HTTP {}", res.status()))
    }
}

/// Webhook Management
pub async fn list_webhooks() -> Result<Vec<serde_json::Value>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/webhooks", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<Vec<serde_json::Value>>().await.map_err(|e| format!("Parse failed: {}", e))
    } else {
        Err(format!("Failed to list webhooks: {}", res.status()))
    }
}

pub async fn create_webhook(name: &str, workflow_name: &str, version: &str) -> Result<String, String> {
    let payload = serde_json::json!({
        "name": name,
        "workflow_name": workflow_name,
        "version": version,
    });
    let res = authorized_request(reqwest::Method::POST, format!("{}/webhooks", &*API_BASE))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        let json = res.json::<serde_json::Value>().await.map_err(|e| format!("Parse failed: {}", e))?;
        Ok(json["webhook_id"].as_str().unwrap_or_default().to_string())
    } else {
        Err(format!("Failed to create webhook: {}", res.status()))
    }
}

pub async fn delete_webhook(id: &str) -> Result<(), String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/webhooks/{}", &*API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to delete webhook: {}", res.status()))
    }
}

/// Request to login to an existing account.
#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Request to create a new user account.
#[derive(Debug, Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

/// Response containing the auth token and user profile.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub mfa_required: Option<bool>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ExecutionSummary {
    pub execution_id: String,
    pub workflow_name: String,
    pub version: String,
    pub status: String,
    pub execution_mode: String,
    pub created_at: String,
    pub current_step: String,
}

/// Login with email and password.
pub async fn login(req: &LoginRequest) -> Result<AuthResponse, String> {
    let res = client()
        .post(format!("{}/auth/login", &*API_BASE))
        .json(req)
        .send()
        .await
        .map_err(|e| format!("Login connection failed: {}", e))?;

    res.json::<AuthResponse>()
        .await
        .map_err(|e| format!("Failed to parse login response: {}", e))
}

/// Signup for a new account.
pub async fn signup(req: &SignupRequest) -> Result<AuthResponse, String> {
    let res = client()
        .post(format!("{}/auth/signup", &*API_BASE))
        .json(req)
        .send()
        .await
        .map_err(|e| format!("Signup connection failed: {}", e))?;

    res.json::<AuthResponse>()
        .await
        .map_err(|e| format!("Failed to parse signup response: {}", e))
}

/// List recent workflow executions.
pub async fn list_recent_executions() -> Result<Vec<ExecutionSummary>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/executions", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    res.json::<Vec<ExecutionSummary>>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

/// Audit Log types.
#[derive(Debug, Deserialize, Clone)]
pub struct AuditLog {
    pub user_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub request_id: Option<String>,
    pub result: Option<String>,
    pub error_message: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: String,
}

/// Query parameters for audit log filtering.
#[derive(Debug, Clone, Default)]
pub struct AuditLogQueryParams {
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub result: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Response containing audit logs with pagination info.
#[derive(Debug, Deserialize)]
pub struct AuditLogListResponse {
    pub logs: Vec<AuditLog>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// Query audit logs with optional filters.
pub async fn get_audit_logs(params: &AuditLogQueryParams) -> Result<AuditLogListResponse, String> {
    let mut url = format!("{}/audit-logs", &*API_BASE);
    
    let mut query_parts = Vec::new();
    if let Some(ref action) = params.action {
        query_parts.push(format!("action={}", urlencoding::encode(action)));
    }
    if let Some(ref resource_type) = params.resource_type {
        query_parts.push(format!("resource_type={}", urlencoding::encode(resource_type)));
    }
    if let Some(ref resource_id) = params.resource_id {
        query_parts.push(format!("resource_id={}", urlencoding::encode(resource_id)));
    }
    if let Some(ref result) = params.result {
        query_parts.push(format!("result={}", urlencoding::encode(result)));
    }
    if let Some(ref start_date) = params.start_date {
        query_parts.push(format!("start_date={}", urlencoding::encode(start_date)));
    }
    if let Some(ref end_date) = params.end_date {
        query_parts.push(format!("end_date={}", urlencoding::encode(end_date)));
    }
    if let Some(limit) = params.limit {
        query_parts.push(format!("limit={}", limit));
    }
    if let Some(offset) = params.offset {
        query_parts.push(format!("offset={}", offset));
    }
    
    if !query_parts.is_empty() {
        url.push('?');
        url.push_str(&query_parts.join("&"));
    }
    
    let res = authorized_request(reqwest::Method::GET, url)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<AuditLogListResponse>()
            .await
            .map_err(|e| format!("Failed to parse audit logs response: {}", e))
    } else {
        Err(format!("Failed to fetch audit logs: HTTP {}", res.status()))
    }
}

// MFA API Types and Functions

/// MFA Setup Response.
#[derive(Debug, Deserialize, Clone)]
pub struct MfaSetupResponse {
    pub success: bool,
    pub secret: Option<String>,
    pub qr_code_uri: Option<String>,
    pub qr_code_svg: Option<String>,
    pub error: Option<String>,
}

/// MFA Verify Setup Response.
#[derive(Debug, Deserialize, Clone)]
pub struct MfaVerifySetupResponse {
    pub success: bool,
    pub backup_codes: Option<Vec<String>>,
    pub error: Option<String>,
}

/// MFA Verify Response (for login).
#[derive(Debug, Deserialize, Clone)]
pub struct MfaVerifyResponse {
    pub success: bool,
    pub token: Option<String>,
    pub error: Option<String>,
}

/// MFA Status Response.
#[derive(Debug, Deserialize, Clone)]
pub struct MfaStatusResponse {
    pub enabled: bool,
}

/// MFA Backup Codes Response.
#[derive(Debug, Deserialize, Clone)]
pub struct MfaBackupCodesResponse {
    pub success: bool,
    pub backup_codes: Option<Vec<String>>,
    pub error: Option<String>,
}

/// Change type for diff entries.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}

/// Change to an input field.
#[derive(Debug, Deserialize, Clone)]
pub struct InputChange {
    pub change_type: ChangeType,
    pub field_name: String,
    pub old_type: Option<String>,
    pub new_type: Option<String>,
}

/// Change to a step.
#[derive(Debug, Deserialize, Clone)]
pub struct StepChange {
    pub change_type: ChangeType,
    pub step_name: String,
    pub old_body: Option<Vec<serde_json::Value>>,
    pub new_body: Option<Vec<serde_json::Value>>,
    pub body_diff_summary: Option<String>,
}

/// Change to an action.
#[derive(Debug, Deserialize, Clone)]
pub struct ActionChange {
    pub change_type: ChangeType,
    pub action_name: String,
    pub step_name: String,
    pub old_args: Option<serde_json::Value>,
    pub new_args: Option<serde_json::Value>,
}

/// Workflow diff response.
#[derive(Debug, Deserialize, Clone)]
pub struct WorkflowDiff {
    pub input_changes: Vec<InputChange>,
    pub step_changes: Vec<StepChange>,
    pub action_changes: Vec<ActionChange>,
    pub breaking_changes: bool,
    pub summary: String,
}

/// Workflow history entry.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WorkflowHistoryEntry {
    pub version: String,
    pub previous_version: Option<String>,
    pub created_at: String,
    pub created_by: String,
    pub changelog: String,
    pub is_breaking_change: bool,
}

/// MFA Disable Response.
#[derive(Debug, Deserialize, Clone)]
pub struct MfaDisableResponse {
    pub success: bool,
    pub error: Option<String>,
}

/// MFA Verify Setup Request.
#[derive(Debug, Serialize)]
pub struct MfaVerifySetupRequest {
    pub code: String,
}

/// MFA Verify Request (for login).
#[derive(Debug, Serialize)]
pub struct MfaVerifyRequest {
    pub user_id: String,
    pub code: String,
}

/// MFA Disable Request.
#[derive(Debug, Serialize)]
pub struct MfaDisableRequest {
    pub code: String,
}

/// MFA Backup Verify Request.
#[derive(Debug, Serialize)]
pub struct MfaBackupVerifyRequest {
    pub user_id: String,
    pub code: String,
}

/// Get MFA setup (TOTP secret and QR code).
pub async fn mfa_setup() -> Result<MfaSetupResponse, String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/auth/mfa/setup", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<MfaSetupResponse>()
            .await
            .map_err(|e| format!("Failed to parse MFA setup response: {}", e))
    } else {
        Err(format!("MFA setup failed: HTTP {}", res.status()))
    }
}

/// Verify MFA setup code.
pub async fn mfa_verify_setup(code: &str) -> Result<MfaVerifySetupResponse, String> {
    let req = MfaVerifySetupRequest {
        code: code.to_string(),
    };
    let res = authorized_request(reqwest::Method::POST, format!("{}/auth/mfa/verify-setup", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<MfaVerifySetupResponse>()
            .await
            .map_err(|e| format!("Failed to parse MFA verify response: {}", e))
    } else {
        Err(format!("MFA verify failed: HTTP {}", res.status()))
    }
}

/// Verify TOTP during login.
pub async fn mfa_verify(user_id: &str, code: &str) -> Result<MfaVerifyResponse, String> {
    let req = MfaVerifyRequest {
        user_id: user_id.to_string(),
        code: code.to_string(),
    };
    let res = client()
        .post(format!("{}/auth/mfa/verify", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<MfaVerifyResponse>()
            .await
            .map_err(|e| format!("Failed to parse MFA verify response: {}", e))
    } else {
        Err(format!("MFA verify failed: HTTP {}", res.status()))
    }
}

/// Get MFA status.
pub async fn mfa_status() -> Result<MfaStatusResponse, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/auth/mfa/status", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<MfaStatusResponse>()
            .await
            .map_err(|e| format!("Failed to parse MFA status response: {}", e))
    } else {
        Err(format!("MFA status failed: HTTP {}", res.status()))
    }
}

/// Disable MFA.
pub async fn mfa_disable(code: &str) -> Result<MfaDisableResponse, String> {
    let req = MfaDisableRequest {
        code: code.to_string(),
    };
    let res = authorized_request(reqwest::Method::POST, format!("{}/auth/mfa/disable", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<MfaDisableResponse>()
            .await
            .map_err(|e| format!("Failed to parse MFA disable response: {}", e))
    } else {
        Err(format!("MFA disable failed: HTTP {}", res.status()))
    }
}

/// Generate backup codes.
pub async fn mfa_generate_backup_codes() -> Result<MfaBackupCodesResponse, String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/auth/mfa/backup-codes", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<MfaBackupCodesResponse>()
            .await
            .map_err(|e| format!("Failed to parse backup codes response: {}", e))
    } else {
        Err(format!("Backup codes generation failed: HTTP {}", res.status()))
    }
}

/// Get workflow version history.
pub async fn get_workflow_history(name: &str) -> Result<Vec<WorkflowHistoryEntry>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/workflows/{}/history", &*API_BASE, name))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<Vec<WorkflowHistoryEntry>>()
            .await
            .map_err(|e| format!("Failed to parse history response: {}", e))
    } else {
        Err(format!("Failed to fetch workflow history: {}", res.status()))
    }
}

/// Compare two workflow versions.
pub async fn compare_workflows(name: &str, v1: &str, v2: &str) -> Result<WorkflowDiff, String> {
    let url = format!("{}/workflows/{}/diff?from={}&to={}", &*API_BASE, name, v1, v2);
    let res = authorized_request(reqwest::Method::GET, url)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<WorkflowDiff>()
            .await
            .map_err(|e| format!("Failed to parse diff response: {}", e))
    } else {
        Err(format!("Failed to compare workflows: {}", res.status()))
    }
}

/// Verify backup code during login.
pub async fn mfa_verify_backup(user_id: &str, code: &str) -> Result<MfaVerifyResponse, String> {
    let req = MfaBackupVerifyRequest {
        user_id: user_id.to_string(),
        code: code.to_string(),
    };
    let res = client()
        .post(format!("{}/auth/mfa/backup-verify", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<MfaVerifyResponse>()
            .await
            .map_err(|e| format!("Failed to parse backup verify response: {}", e))
    } else {
        Err(format!("Backup code verification failed: HTTP {}", res.status()))
    }
}
