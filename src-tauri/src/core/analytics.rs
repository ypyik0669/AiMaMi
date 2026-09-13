use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageAnalyticsPayload {
    pub total_tokens: i64,
    pub avg_per_session: f64,
    pub input_pct: f64,
    pub output_pct: f64,
    pub reasoning_pct: f64,
    pub input_total: i64,
    pub output_total: i64,
    pub reasoning_total: i64,
    pub series: Vec<crate::core::models::TokenDaySeries>,
}
