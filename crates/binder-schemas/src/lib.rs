//! binder-schemas: AI Intent Binder 的公共 Schema 定义
//!
//! 包含 Intent、Capability、Audit 的 Rust 类型定义、JSON Schema 定义和校验逻辑。

pub mod intent;
pub mod capability;
pub mod audit;

/// 风险等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    ReadOnly,
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    /// R0 只读 - 自动允许
    pub fn is_read_only(&self) -> bool {
        matches!(self, RiskLevel::ReadOnly)
    }

    /// R0/R1 - 自动允许
    pub fn is_auto_allow(&self) -> bool {
        matches!(self, RiskLevel::ReadOnly | RiskLevel::Low)
    }

    /// R2+ - 需要确认
    pub fn requires_confirmation(&self) -> bool {
        matches!(self, RiskLevel::Medium | RiskLevel::High | RiskLevel::Critical)
    }

    /// R4 - 默认禁止
    pub fn is_default_deny(&self) -> bool {
        matches!(self, RiskLevel::Critical)
    }
}

/// 目标平台
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Android,
    Ios,
    Linux,
    Windows,
    Darwin,
    Browser,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Android => write!(f, "android"),
            Platform::Ios => write!(f, "ios"),
            Platform::Linux => write!(f, "linux"),
            Platform::Windows => write!(f, "windows"),
            Platform::Darwin => write!(f, "darwin"),
            Platform::Browser => write!(f, "browser"),
        }
    }
}

/// 用户角色 (RBAC)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Operator,
    Admin,
    DeviceOwner,
}