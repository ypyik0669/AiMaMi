use crate::core::models::{RateLimitWindow, UsageSource};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuotaStoreFile {
    pub schema_version: i32,
    pub updated_at: i64,
    pub items: Vec<QuotaStoreItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaStoreItem {
    pub account_key: String,
    pub captured_at: i64,
    pub usage_source: UsageSource,
    pub primary_window: Option<RateLimitWindow>,
    pub secondary_window: Option<RateLimitWindow>,
    pub token_status: Option<Value>,
}

pub fn load_or_default(path: &Path) -> QuotaStoreFile {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_else(|| QuotaStoreFile { schema_version: 1, ..Default::default() })
}

pub fn save(path: &Path, value: &QuotaStoreFile) -> Result<(), crate::core::models::CoreError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(value)?)?;
    Ok(())
}

pub fn find_item<'a>(store: &'a QuotaStoreFile, account_key: &str) -> Option<&'a QuotaStoreItem> {
    store.items.iter().find(|item| item.account_key == account_key)
}

pub fn upsert_item(store: &mut QuotaStoreFile, item: QuotaStoreItem, updated_at: i64) -> bool {
    if let Some(existing) = store.items.iter_mut().find(|entry| entry.account_key == item.account_key) {
        *existing = item;
    } else {
        store.items.push(item);
    }
    store.updated_at = updated_at;
    true
}
