use crate::core::auth::ApiRequestContext;
use crate::core::models::{ApiProxyConfigPayload, ApiProxyMode, ApiProxyTestPayload, ApiReachabilityStatus};

pub fn sanitize_proxy_config(value: &ApiProxyConfigPayload) -> Result<ApiProxyConfigPayload, crate::core::models::CoreError> {
    match value.mode {
        ApiProxyMode::Direct => Ok(ApiProxyConfigPayload { mode: ApiProxyMode::Direct, url: None }),
        ApiProxyMode::Manual => {
            let url = value.url.as_deref().unwrap_or("").trim();
            if url.is_empty() {
                return Err(crate::core::models::CoreError::InvalidData("Manual proxy URL is required".into()));
            }
            reqwest::Url::parse(url).map_err(|_| crate::core::models::CoreError::InvalidData("Invalid proxy URL".into()))?;
            Ok(ApiProxyConfigPayload { mode: ApiProxyMode::Manual, url: Some(url.to_string()) })
        }
    }
}

pub fn test_api_connectivity(config: &ApiProxyConfigPayload, context: Option<&ApiRequestContext>) -> ApiProxyTestPayload {
    let client = match build_client(config) {
        Ok(c) => c,
        Err(e) => return ApiProxyTestPayload { code: "invalid_proxy".into(), reachable: false, status_code: None, message: e.to_string() },
    };
    let mut request = client.get("https://chatgpt.com/backend-api/accounts/check/v4-accounts");
    if let Some(ctx) = context {
        request = request.bearer_auth(&ctx.access_token);
    }
    match request.send() {
        Ok(response) => {
            let status = response.status();
            ApiProxyTestPayload { code: if status.is_success() { "reachable" } else { "http_error" }.into(), reachable: status.is_success(), status_code: Some(status.as_u16() as i32), message: status.to_string() }
        }
        Err(e) => ApiProxyTestPayload { code: "request_failed".into(), reachable: false, status_code: None, message: e.to_string() },
    }
}

pub fn detect_api_proxy_config(context: Option<&ApiRequestContext>) -> crate::core::models::ApiProxyDetectPayload {
    let direct = test_api_connectivity(&ApiProxyConfigPayload::default(), context);
    crate::core::models::ApiProxyDetectPayload {
        found: direct.reachable,
        mode: if direct.reachable { Some(ApiProxyMode::Direct) } else { None },
        url: None,
        probe: direct,
    }
}

fn build_client(config: &ApiProxyConfigPayload) -> Result<reqwest::blocking::Client, crate::core::models::CoreError> {
    let mut builder = reqwest::blocking::Client::builder();
    if let ApiProxyMode::Manual = config.mode {
        builder = builder.proxy(reqwest::Proxy::all(config.url.as_deref().unwrap_or(""))?);
    }
    Ok(builder.build()?)
}
