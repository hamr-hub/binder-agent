//! Capability Schema 定义、Registry 和解析器

use super::{Platform, RiskLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 注册的能力定义（对应 capabilities/*.yaml 文件）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub version: u32,
    pub description: String,
    pub risk_level: RiskLevel,
    pub reversible: bool,
    pub requires_confirmation: bool,
    pub input_schema: serde_json::Value,
    pub supported_platforms: Vec<PlatformSupport>,
    #[serde(default)]
    pub bindings: HashMap<String, CapabilityBinding>,
    #[serde(default)]
    pub success_criteria: Option<Vec<SuccessCriterion>>,
    #[serde(default)]
    pub rollback: Option<RollbackConfig>,
}

/// 平台支持声明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSupport {
    pub os: Platform,
    #[serde(default)]
    pub arch: Option<Vec<String>>,
}

/// Capability 绑定（指定哪个 Adapter 的哪个方法）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityBinding {
    pub adapter: String,
    pub method: String,
    #[serde(default)]
    pub verifier: Option<String>,
    #[serde(default)]
    pub rollback: Option<String>,
}

/// 成功标准
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriterion {
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub equals_param: Option<String>,
    #[serde(default)]
    pub equals_value: Option<serde_json::Value>,
    #[serde(default)]
    pub contains: Option<String>,
    #[serde(default)]
    pub exists: Option<bool>,
}

/// 回滚配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    pub strategy: RollbackStrategy,
}

/// 回滚策略
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackStrategy {
    RestorePreviousValue,
    CompensatingAction,
    SnapshotRestore,
    None,
}

/// Schema 相关错误
#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    #[error("Failed to load schema: {0}")]
    LoadFailed(String),
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Capability Registry 接口
pub trait CapabilityRegistry: Send + Sync {
    /// 注册一个 Capability
    fn register(&mut self, capability: Capability) -> Result<(), SchemaError>;

    /// 根据 ID 查找 Capability
    fn get(&self, id: &str) -> Option<&Capability>;

    /// 列出所有 Capability
    fn list_all(&self) -> Vec<&Capability>;

    /// 根据平台过滤 Capability
    fn list_by_platform(&self, platform: Platform) -> Vec<&Capability>;

    /// 根据风险等级过滤
    fn list_by_risk(&self, risk_level: RiskLevel) -> Vec<&Capability>;
}

/// 内存实现的 CapabilityRegistry
#[derive(Debug, Default)]
pub struct InMemoryRegistry {
    capabilities: HashMap<String, Capability>,
}

impl InMemoryRegistry {
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
        }
    }
}

impl CapabilityRegistry for InMemoryRegistry {
    fn register(&mut self, capability: Capability) -> Result<(), SchemaError> {
        self.capabilities
            .insert(capability.id.clone(), capability);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<&Capability> {
        self.capabilities.get(id)
    }

    fn list_all(&self) -> Vec<&Capability> {
        self.capabilities.values().collect()
    }

    fn list_by_platform(&self, platform: Platform) -> Vec<&Capability> {
        self.capabilities
            .values()
            .filter(|c| {
                c.supported_platforms
                    .iter()
                    .any(|ps| ps.os == platform)
            })
            .collect()
    }

    fn list_by_risk(&self, risk_level: RiskLevel) -> Vec<&Capability> {
        self.capabilities
            .values()
            .filter(|c| c.risk_level == risk_level)
            .collect()
    }
}