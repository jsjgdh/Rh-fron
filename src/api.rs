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
    pub span_start: usize,
    pub span_end: usize,
    pub line: usize,
    pub column: usize,
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
    pub last_updated: String,
    pub compliance_frameworks: Option<Vec<String>>,
    pub region: String,
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

/// Store refresh token for token rotation.
pub fn set_refresh_token(token: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("rhexiom_refresh_token", token);
            }
        }
    }
}

/// Get stored refresh token.
pub fn get_refresh_token() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                return storage.get_item("rhexiom_refresh_token").ok().flatten();
            }
        }
    }
    None
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
                let _ = storage.remove_item("rhexiom_refresh_token");
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

/// Refresh the access token using the refresh token.
pub async fn refresh_access_token() -> Result<(), String> {
    let refresh_token = get_refresh_token().ok_or("No refresh token available")?;

    #[derive(Serialize)]
    struct RefreshRequest<'a> {
        refresh_token: &'a str,
    }

    #[derive(Deserialize)]
    struct RefreshResponse {
        success: bool,
        token: Option<String>,
        error: Option<String>,
    }

    let res = client()
        .request(reqwest::Method::POST, format!("{}/auth/refresh", &*API_BASE))
        .json(&RefreshRequest { refresh_token: &refresh_token })
        .send()
        .await
        .map_err(|e| format!("Network error during token refresh: {}", e))?;

    let response: RefreshResponse = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse refresh response: {}", e))?;

    if response.success {
        if let Some(token) = response.token {
            set_token(&token);
        }
        Ok(())
    } else {
        Err(response.error.unwrap_or_else(|| "Token refresh failed".to_string()))
    }
}

/// Make an authorized request that automatically handles 401 by refreshing the token once.
pub async fn authorized_request_with_refresh(
    method: reqwest::Method,
    url: String,
) -> Result<reqwest::Response, String> {
    let res = authorized_request(method.clone(), url.clone())
        .send()
        .await
        .map_err(|e| format!("Network request failed: {}", e))?;

    if res.status() == reqwest::StatusCode::UNAUTHORIZED {
        if refresh_access_token().await.is_ok() {
            let res = authorized_request(method, url)
                .send()
                .await
                .map_err(|e| format!("Network request failed after token refresh: {}", e))?;
            Ok(res)
        } else {
            logout();
            Err("Session expired. Please sign in again.".to_string())
        }
    } else {
        Ok(res)
    }
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
pub async fn list_workflows(query: Option<&str>) -> Result<Vec<WorkflowSummary>, String> {
    #[derive(Deserialize)]
    struct Wrapper {
        workflows: Vec<WorkflowSummary>,
    }
    let mut url = format!("{}/workflows", &*API_BASE);
    if let Some(q) = query {
        url = format!("{}?q={}", url, urlencoding::encode(q));
    }
    let res = authorized_request(reqwest::Method::GET, url)
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

/// A system alert for the dashboard.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SystemAlert {
    pub id: String,
    pub alert_type: String,
    pub title: String,
    pub message: String,
    pub severity: String,
}

/// Response containing system alerts.
#[derive(Debug, Deserialize)]
pub struct AlertsResponse {
    pub alerts: Vec<SystemAlert>,
}

/// Get system alerts for the dashboard.
pub async fn get_alerts() -> Result<Vec<SystemAlert>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/alerts", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to backend: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Server returned HTTP {}", res.status()));
    }

    res.json::<AlertsResponse>()
        .await
        .map(|r| r.alerts)
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

/// Simulation mode - Quick (first decision path) or Full (all branches).
#[derive(Debug, Clone, PartialEq)]
pub enum SimulationMode {
    Quick,
    Full,
}

impl SimulationMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            SimulationMode::Quick => "quick",
            SimulationMode::Full => "full",
        }
    }
}

/// Request to run a workflow simulation.
#[derive(Debug, Serialize)]
pub struct SimulateRequest {
    pub workflow_name: String,
    pub version: String,
    pub input: HashMap<String, serde_json::Value>,
    pub mode: String,
    pub enable_external_calls: bool,
}

/// A single step in the simulation trace.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SimulationStep {
    pub step_name: String,
    pub action: Option<String>,
    pub timestamp_us: u64,
    pub variables: HashMap<String, serde_json::Value>,
    pub duration_us: u64,
    pub shadowed: bool,
}

/// Represents a decision path taken during execution.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DecisionPath {
    pub step_name: String,
    pub condition: Option<String>,
    pub branch_taken: String,
    pub alternative_branches: Vec<String>,
}

/// Detailed simulation trace.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SimulationTrace {
    pub workflow_name: String,
    pub version: String,
    pub steps: Vec<SimulationStep>,
    pub total_duration_us: u64,
    pub decision_paths: Vec<DecisionPath>,
}

/// An action that would be executed during simulation.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SimulatedAction {
    pub name: String,
    pub step_name: String,
    pub executed: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// Execution timing information.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ExecutionTiming {
    pub total_duration_us: u64,
    pub execution_time_us: u64,
    pub external_call_time_us: u64,
    pub memory_usage_bytes: u64,
    pub step_timings: HashMap<String, u64>,
}

/// Simulation metadata.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SimulationMetadata {
    pub mode: String,
    pub external_calls_enabled: bool,
    pub started_at: String,
    pub steps_executed: usize,
    pub branches_explored: usize,
}

/// Full simulation result response.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SimulationResult {
    pub original_execution_id: Option<String>,
    pub simulation_execution_id: String,
    pub status: String,
    pub workflow_name: String,
    pub version: String,
    pub trace: SimulationTrace,
    pub actions: Vec<SimulatedAction>,
    pub error: Option<String>,
    pub timing: ExecutionTiming,
    pub metadata: SimulationMetadata,
}

/// Run a what-if simulation for a historical execution.
pub async fn simulate_execution(execution_id: &str) -> Result<SimulationResult, String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/executions/{}/simulate", &*API_BASE, execution_id))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<SimulationResult>()
            .await
            .map_err(|e| format!("Failed to parse simulation result: {}", e))
    } else {
        Err(format!("Simulation failed with status: {}", res.status()))
    }
}

/// Run a direct simulation for a workflow with given inputs.
pub async fn simulate_workflow(req: &SimulateRequest) -> Result<SimulationResult, String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/simulate", &*API_BASE))
        .json(req)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<SimulationResult>()
            .await
            .map_err(|e| format!("Failed to parse simulation result: {}", e))
    } else {
        let status = res.status();
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Simulation failed with status {}: {}", status, error_text))
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
pub async fn list_recent_executions(query: Option<&str>, cursor: Option<&str>) -> Result<Vec<ExecutionSummary>, String> {
    let mut url = format!("{}/executions", &*API_BASE);
    let mut params = Vec::new();
    if let Some(q) = query {
        params.push(format!("q={}", urlencoding::encode(q)));
    }
    if let Some(c) = cursor {
        params.push(format!("cursor={}", urlencoding::encode(c)));
    }
    if !params.is_empty() {
        url = format!("{}?{}", url, params.join("&"));
    }
    let res = authorized_request(reqwest::Method::GET, url)
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WorkflowAnalytics {
    pub name: String,
    pub execution_count: i64,
    pub success_rate: f64,
    pub avg_latency_ms: f64,
}

/// Fetch aggregated workflow analytics.
pub async fn get_analytics() -> Result<Vec<WorkflowAnalytics>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/analytics", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<Vec<WorkflowAnalytics>>()
            .await
            .map_err(|e| format!("Failed to parse analytics JSON: {}", e))
    } else {
        Err(format!("Analytics request failed with status: {}", res.status()))
    }
}

/// Promote a specific version of a workflow to latest (Rollback).
pub async fn promote_version(name: &str, version: &str) -> Result<String, String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/workflows/{}/{}/promote", &*API_BASE, name, version))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        let json = res.json::<serde_json::Value>().await.map_err(|e| format!("Decoding failed: {}", e))?;
        Ok(json["new_version"].as_str().unwrap_or_default().to_string())
    } else {
        Err(format!("Rollback failed: {}", res.status()))
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TemplateSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Option<Vec<String>>,
    pub compliance_frameworks: Option<Vec<String>>,
    pub usage_count: i32,
}

#[derive(Debug, Serialize)]
pub struct InstantiateTemplateRequest {
    pub workflow_name: String,
}

/// List all available workflow templates.
pub async fn list_templates() -> Result<Vec<TemplateSummary>, String> {
    #[derive(Deserialize)]
    struct Wrapper {
        templates: Vec<TemplateSummary>,
    }
    let res = authorized_request(reqwest::Method::GET, format!("{}/templates", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        let wrapper = res.json::<Wrapper>().await.map_err(|e| format!("Decoding failed: {}", e))?;
        Ok(wrapper.templates)
    } else {
        Err(format!("Failed to list templates: {}", res.status()))
    }
}

/// Instantiate a workflow from a template.
pub async fn instantiate_template(template_id: &str, workflow_name: &str) -> Result<CompileResponse, String> {
    let req = InstantiateTemplateRequest {
        workflow_name: workflow_name.to_string(),
    };
    let res = authorized_request(reqwest::Method::POST, format!("{}/templates/{}/instantiate", &*API_BASE, template_id))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<CompileResponse>()
            .await
            .map_err(|e| format!("Failed to parse compilation result: {}", e))
    } else {
        Err(format!("Instantiation failed: {}", res.status()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase 3: User Management
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct OrgMemberResponse {
    pub id: String,
    pub email: String,
    pub role: String,
    pub joined_at: String,
    pub status: String,
}

/// List all members of the current organization.
pub async fn list_org_members() -> Result<Vec<crate::components::users::OrgMember>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/orgs/members", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() {
        let raw: Vec<OrgMemberResponse> = res.json().await.map_err(|e| e.to_string())?;
        Ok(raw.into_iter().map(|m| crate::components::users::OrgMember {
            id: m.id,
            email: m.email,
            role: m.role,
            joined_at: m.joined_at,
            status: m.status,
        }).collect())
    } else {
        Err(format!("Failed to list members: {}", res.status()))
    }
}

#[derive(Debug, Serialize)]
struct InviteMemberRequest { email: String, role: String }

/// Invite a new member to the organization.
pub async fn invite_member(email: &str, role: &str) -> Result<(), String> {
    let req = InviteMemberRequest { email: email.to_string(), role: role.to_string() };
    let res = authorized_request(reqwest::Method::POST, format!("{}/orgs/members/invite", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Invite failed: {}", res.status())) }
}

#[derive(Debug, Serialize)]
struct UpdateRoleRequest { role: String }

/// Update a member's role.
pub async fn update_member_role(member_id: &str, role: &str) -> Result<(), String> {
    let req = UpdateRoleRequest { role: role.to_string() };
    let res = authorized_request(reqwest::Method::PATCH, format!("{}/orgs/members/{}/role", &*API_BASE, member_id))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Role update failed: {}", res.status())) }
}

/// Remove a member from the organization.
pub async fn remove_member(member_id: &str) -> Result<(), String> {
    let res = authorized_request(reqwest::Method::DELETE, format!("{}/orgs/members/{}", &*API_BASE, member_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Remove failed: {}", res.status())) }
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase 3: Policy Test Suite
// ─────────────────────────────────────────────────────────────────────────────

/// List all saved test cases for the current org.
pub async fn list_test_cases() -> Result<Vec<crate::components::test_suite::TestCase>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/test-cases", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() {
        res.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed: {}", res.status()))
    }
}

/// Save a test case to the backend.
pub async fn save_test_case(tc: &crate::components::test_suite::TestCase) -> Result<(), String> {
    let res = authorized_request(reqwest::Method::POST, format!("{}/test-cases", &*API_BASE))
        .json(tc)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Save failed: {}", res.status())) }
}

/// Run a single test case against the backend and return a RunResponse.
pub async fn run_test_case(tc: &crate::components::test_suite::TestCase) -> Result<RunResponse, String> {
    let req = RunRequest {
        workflow_name: tc.workflow_name.clone(),
        version: tc.version.clone(),
        input: tc.input.as_object().map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect()).unwrap_or_default(),
        execution_mode: "Live".to_string(),
    };
    run_workflow(&req).await
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase 3: Workflow Scheduling
// ─────────────────────────────────────────────────────────────────────────────

/// List all scheduled jobs for the current org.
pub async fn list_scheduled_jobs() -> Result<Vec<crate::components::schedule::ScheduledJob>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/schedules", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() {
        res.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed: {}", res.status()))
    }
}

#[derive(Debug, Serialize)]
struct CreateScheduleRequest {
    workflow_name: String,
    version: String,
    cron_expr: Option<String>,
    run_at: Option<String>,
}

/// Create a new workflow schedule.
pub async fn create_schedule(workflow: &str, version: &str, cron: Option<&str>, run_at: Option<&str>) -> Result<(), String> {
    let req = CreateScheduleRequest {
        workflow_name: workflow.to_string(),
        version: version.to_string(),
        cron_expr: cron.map(str::to_string),
        run_at: run_at.map(str::to_string),
    };
    let res = authorized_request(reqwest::Method::POST, format!("{}/schedules", &*API_BASE))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Create schedule failed: {}", res.status())) }
}

/// Delete a scheduled job.
pub async fn delete_schedule(job_id: &str) -> Result<(), String> {
    let res = authorized_request(reqwest::Method::DELETE, format!("{}/schedules/{}", &*API_BASE, job_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Delete failed: {}", res.status())) }
}

/// Toggle a schedule active/paused state.
pub async fn toggle_schedule(job_id: &str) -> Result<(), String> {
    let res = authorized_request(reqwest::Method::PATCH, format!("{}/schedules/{}/toggle", &*API_BASE, job_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Toggle failed: {}", res.status())) }
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase 3: Approval Chains
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ApprovalResponseItem {
    id: String,
    workflow_name: String,
    version: String,
    requested_by: String,
    requested_at: String,
    input_summary: String,
    status: String,
}

/// List approval requests for the current user.
pub async fn list_pending_approvals() -> Result<Vec<crate::components::approvals::ApprovalRequest>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/approvals", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() {
        let raw: Vec<ApprovalResponseItem> = res.json().await.map_err(|e| e.to_string())?;
        Ok(raw.into_iter().map(|a| {
            use crate::components::approvals::{ApprovalRequest, ApprovalStatus};
            ApprovalRequest {
                id: a.id,
                workflow_name: a.workflow_name,
                version: a.version,
                requested_by: a.requested_by,
                requested_at: a.requested_at,
                input_summary: a.input_summary,
                status: match a.status.as_str() {
                    "approved" => ApprovalStatus::Approved,
                    "rejected" => ApprovalStatus::Rejected,
                    _          => ApprovalStatus::Pending,
                },
            }
        }).collect())
    } else {
        Err(format!("Failed: {}", res.status()))
    }
}

#[derive(Debug, Serialize)]
struct ApprovalActionRequest { comment: String }

/// Approve a pending execution.
pub async fn approve_execution(approval_id: &str, comment: &str) -> Result<(), String> {
    let req = ApprovalActionRequest { comment: comment.to_string() };
    let res = authorized_request(reqwest::Method::POST, format!("{}/approvals/{}/approve", &*API_BASE, approval_id))
        .json(&req).send().await.map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Approve failed: {}", res.status())) }
}

/// Reject a pending execution.
pub async fn reject_execution(approval_id: &str, comment: &str) -> Result<(), String> {
    let req = ApprovalActionRequest { comment: comment.to_string() };
    let res = authorized_request(reqwest::Method::POST, format!("{}/approvals/{}/reject", &*API_BASE, approval_id))
        .json(&req).send().await.map_err(|e| format!("Network error: {}", e))?;
    if res.status().is_success() { Ok(()) } else { Err(format!("Reject failed: {}", res.status())) }
}

