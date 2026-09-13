use crate::core::models::{AuthMode, PlanType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthTokens {
    pub id_token: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthFile {
    pub auth_mode: Option<String>,
    pub openai_api_key: Option<String>,
    #[serde(default)]
    pub tokens: AuthTokens,
    pub last_refresh: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApiRequestContext {
    pub access_token: String,
    pub account_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthSnapshot {
    pub account_key: String,
    pub email: String,
    pub account_name: Option<String>,
    pub workspace_name: Option<String>,
    pub profile_name: Option<String>,
    pub plan: PlanType,
    pub auth_mode: AuthMode,
    pub created_at: i64,
}

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default()
}

pub fn load_auth_file(path: &Path) -> Result<AuthFile, std::io::Error> {
    let raw = std::fs::read_to_string(path)?;
    serde_json::from_str(&raw).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn make_api_request_context(auth: &AuthFile) -> Option<ApiRequestContext> {
    auth.tokens
        .access_token
        .clone()
        .or_else(|| auth.openai_api_key.clone())
        .map(|access_token| ApiRequestContext {
            access_token,
            account_id: auth.tokens.account_id.clone(),
        })
}

pub fn make_auth_snapshot(auth: &AuthFile, path: &Path) -> Result<AuthSnapshot, crate::core::models::CoreError> {
    let token = auth
        .tokens
        .id_token
        .as_deref()
        .or(auth.tokens.access_token.as_deref());
    let claims = token.and_then(decode_jwt_payload);
    let email = claims
        .as_ref()
        .and_then(|v| v.get("email").and_then(Value::as_str))
        .unwrap_or("unknown")
        .to_string();
    let account_key = claims
        .as_ref()
        .and_then(|v| v.get("sub").and_then(Value::as_str))
        .or(auth.tokens.account_id.as_deref())
        .unwrap_or(&email)
        .to_string();
    let auth_mode = if auth.auth_mode.as_deref().unwrap_or("chatgpt").eq_ignore_ascii_case("apikey") {
        AuthMode::Apikey
    } else {
        AuthMode::Chatgpt
    };
    let plan = claims
        .as_ref()
        .and_then(|v| v.get("plan_type").or_else(|| v.get("plan")))
        .and_then(Value::as_str)
        .map(parse_plan)
        .unwrap_or(PlanType::Unknown);
    let created_at = std::fs::metadata(path)
        .and_then(|m| m.created())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or_else(current_timestamp);
    Ok(AuthSnapshot {
        account_key,
        email,
        account_name: claims.as_ref().and_then(|v| v.get("name").and_then(Value::as_str)).map(str::to_owned),
        workspace_name: claims.as_ref().and_then(|v| v.get("workspace_name").and_then(Value::as_str)).map(str::to_owned),
        profile_name: claims.as_ref().and_then(|v| v.get("profile_name").and_then(Value::as_str)).map(str::to_owned),
        plan,
        auth_mode,
        created_at,
    })
}

fn parse_plan(value: &str) -> PlanType {
    match value.to_ascii_lowercase().as_str() {
        "free" => PlanType::Free,
        "plus" => PlanType::Plus,
        "pro5x" | "pro_5x" | "pro-5x" => PlanType::Pro5x,
        "pro20x" | "pro_20x" | "pro-20x" => PlanType::Pro20x,
        "team" => PlanType::Team,
        "business" => PlanType::Business,
        "enterprise" => PlanType::Enterprise,
        "edu" => PlanType::Edu,
        _ => PlanType::Unknown,
    }
}

fn decode_jwt_payload(token: &str) -> Option<Value> {
    let payload = token.split('.').nth(1)?;
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, payload)
        .or_else(|_| base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE, payload))
        .ok()?;
    serde_json::from_slice(&bytes).ok()
}
