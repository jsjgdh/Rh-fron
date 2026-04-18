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
}

/// A step generated locally for trace tracking
#[derive(Debug, Deserialize, Clone)]
pub struct TraceStepResponse {
    pub step_name: String,
    pub action: Option<String>,
    pub timestamp_us: u64,
}

/// Response from execution.
#[derive(Debug, Deserialize, Clone)]
pub struct RunResponse {
    pub success: bool,
    pub execution_id: Option<String>,
    pub status: String,
    pub final_step: String,
    pub actions: Vec<String>,
    pub trace: Vec<TraceStepResponse>,
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

pub fn logout() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("rhexiom_token");
                let _ = storage.remove_item("rhexiom_email");
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
    let res = client()
        .post(format!("{}/workflows/generate", &*API_BASE))
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

/// Extract text from a PDF document.
pub async fn extract_pdf(bytes: Vec<u8>) -> Result<String, String> {
    let form = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(bytes).file_name("policy.pdf"),
    );

    let res = client()
        .post(format!("{}/workflows/extract-pdf", &*API_BASE))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        let json = res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse PDF extraction result: {}", e))?;
        Ok(json["text"].as_str().unwrap_or_default().to_string())
    } else {
        Err(format!("PDF extraction failed with HTTP {}", res.status()))
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

/// List available integration services.
pub async fn get_integrations() -> Result<Vec<String>, String> {
    let res = authorized_request(reqwest::Method::GET, format!("{}/integrations", &*API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network connection failed: {}", e))?;

    if res.status().is_success() {
        res.json::<Vec<String>>().await.map_err(|e| format!("Failed to parse integrations: {}", e))
    } else {
        Err(format!("Integrations request failed with HTTP {}", res.status()))
    }
}

/// Update integration credentials.
pub async fn update_integration(name: &str, api_key: &str) -> Result<(), String> {
    let payload = serde_json::json!({
        "name": name,
        "api_key": api_key,
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
}

/// Response containing the auth token and user profile.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub user_id: Option<String>,
    pub email: Option<String>,
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
