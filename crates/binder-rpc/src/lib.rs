//! binder-rpc: Binder Core 与 Host Adapter 之间的 RPC 接口定义
//!
//! 所有平台 Host Adapter 必须实现 HostAdapter trait。

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Host 环境信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostInfo {
    pub hostname: String,
    pub os: String,
    pub arch: String,
    pub os_version: String,
    pub adapter_version: String,
}

/// 预检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecheckResult {
    pub ok: bool,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

/// 快照数据（用于回滚）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub timestamp: String,
    pub capability_id: String,
    pub state: serde_json::Value,
}

/// 干运行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunResult {
    pub ok: bool,
    #[serde(default)]
    pub expected_effects: Vec<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

/// 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResult {
    pub ok: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub output: Option<serde_json::Value>,
    pub duration_ms: u64,
}

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResult {
    pub ok: bool,
    #[serde(default)]
    pub observed: Option<serde_json::Value>,
    #[serde(default)]
    pub diff: Option<String>,
}

/// 回滚结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackResult {
    pub ok: bool,
    #[serde(default)]
    pub message: Option<String>,
}

/// Adapter 错误类型
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("Capability '{0}' not found on this adapter")]
    CapabilityNotFound(String),

    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    #[error("Precondition not met: {0}")]
    PreconditionFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Rollback failed: {0}")]
    RollbackFailed(String),

    #[error("Not reversible: capability '{0}' does not support rollback")]
    NotReversible(String),

    #[error("Confirmation required for capability '{0}'")]
    ConfirmationRequired(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Timeout: {0}")]
    Timeout(String),
}

/// 所有平台 Host Adapter 必须实现的统一接口
#[async_trait]
pub trait HostAdapter: Send + Sync {
    /// 描述当前 Host 环境信息
    async fn describe_host(&self) -> Result<HostInfo, AdapterError>;

    /// 列出本 Adapter 支持的所有 Capability ID
    async fn list_capabilities(&self) -> Result<Vec<String>, AdapterError>;

    /// 前置条件检查
    async fn precheck(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<PrecheckResult, AdapterError>;

    /// 保存执行前快照
    async fn snapshot(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<Snapshot, AdapterError>;

    /// 干运行（不产生实际效果）
    async fn dry_run(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<DryRunResult, AdapterError>;

    /// 执行 Capability 对应的实际操作
    async fn execute(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<ExecuteResult, AdapterError>;

    /// 验证执行结果
    async fn verify(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
        result: &ExecuteResult,
    ) -> Result<VerifyResult, AdapterError>;

    /// 回滚操作
    async fn rollback(
        &self,
        capability_id: &str,
        snapshot: &Snapshot,
    ) -> Result<RollbackResult, AdapterError>;
}