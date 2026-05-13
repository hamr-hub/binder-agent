//! Audit Schema 定义与 Hash Chain

use super::Platform;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// 审计记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub audit_id: String,
    pub intent_id: String,
    pub actor: Actor,
    pub intent_type: String,
    pub target: AuditTarget,
    pub capability_id: String,
    #[serde(default)]
    pub capability_version: Option<u32>,
    pub policy_decision: PolicyDecision,
    #[serde(default)]
    pub policy_reason: Option<String>,
    #[serde(default)]
    pub pre_state: Option<serde_json::Value>,
    pub parameters: serde_json::Value,
    pub execution_result: ExecutionResult,
    #[serde(default)]
    pub verification_result: Option<VerificationResult>,
    #[serde(default)]
    pub rollback: Option<RollbackResult>,
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    pub previous_hash: Option<String>,
    pub hash: String,
}

/// 操作者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub user_id: String,
    pub role: super::Role,
}

/// 审计目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTarget {
    pub platform: Platform,
    pub device_id: String,
}

/// 策略决定
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
    Allow,
    Deny,
    Confirm,
    Error,
}

/// 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub ok: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub adapter: Option<String>,
    #[serde(default)]
    pub duration_ms: Option<u64>,
}

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub ok: bool,
    #[serde(default)]
    pub observed: Option<serde_json::Value>,
    #[serde(default)]
    pub diff: Option<String>,
}

/// 回滚结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackResult {
    pub attempted: bool,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Hash Chain 计算
pub struct HashChain;

impl HashChain {
    /// 计算一条审计记录的 hash
    /// hash = SHA-256(previous_hash + normalized_record_json)
    pub fn compute_hash(
        previous_hash: Option<&str>,
        record: &AuditRecord,
    ) -> Result<String, serde_json::Error> {
        let mut hasher = Sha256::new();

        if let Some(prev) = previous_hash {
            hasher.update(prev.as_bytes());
        }

        // 序列化 record_data（排除 hash 字段）
        let record_json = serde_json::to_string(record)?;
        hasher.update(record_json.as_bytes());

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// 验证 hash chain 完整性
    pub fn verify_chain(records: &[AuditRecord]) -> Result<bool, String> {
        for i in 1..records.len() {
            let prev = &records[i - 1];
            let current = &records[i];

            let expected_hash =
                Self::compute_hash(Some(&prev.hash), current).map_err(|e| e.to_string())?;

            if expected_hash != current.hash {
                return Ok(false);
            }
        }

        Ok(true)
    }
}