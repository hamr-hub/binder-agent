//! 执行状态机
//!
//! 定义从 Intent 接收到审计提交的完整状态转换。
//!
//! RECEIVED → INTENT_SCHEMA_VALIDATED → CAPABILITY_RESOLVED →
//! PLATFORM_SUPPORTED → POLICY_CHECKED → PRECONDITION_CHECKED →
//! SNAPSHOT_TAKEN → DRY_RUN_COMPLETED → CONFIRMATION_CHECKED →
//! EXECUTING → VERIFYING → SUCCEEDED/FAILED →
//! ROLLBACK_IF_NEEDED → AUDIT_COMMITTED

/// 执行状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionState {
    Received,
    IntentSchemaValidated,
    CapabilityResolved,
    PlatformSupported,
    PolicyChecked,
    PreconditionChecked,
    SnapshotTaken,
    DryRunCompleted,
    ConfirmationChecked,
    Executing,
    Verifying,
    Succeeded,
    Failed,
    RollbackIfNeeded,
    AuditCommitted,
}

impl ExecutionState {
    /// 获取下一个状态
    pub fn next(self) -> Option<ExecutionState> {
        match self {
            ExecutionState::Received => Some(ExecutionState::IntentSchemaValidated),
            ExecutionState::IntentSchemaValidated => Some(ExecutionState::CapabilityResolved),
            ExecutionState::CapabilityResolved => Some(ExecutionState::PlatformSupported),
            ExecutionState::PlatformSupported => Some(ExecutionState::PolicyChecked),
            ExecutionState::PolicyChecked => Some(ExecutionState::PreconditionChecked),
            ExecutionState::PreconditionChecked => Some(ExecutionState::SnapshotTaken),
            ExecutionState::SnapshotTaken => Some(ExecutionState::DryRunCompleted),
            ExecutionState::DryRunCompleted => Some(ExecutionState::ConfirmationChecked),
            ExecutionState::ConfirmationChecked => Some(ExecutionState::Executing),
            ExecutionState::Executing => Some(ExecutionState::Verifying),
            ExecutionState::Verifying => None, // 需要根据结果分支
            ExecutionState::Succeeded => Some(ExecutionState::AuditCommitted),
            ExecutionState::Failed => Some(ExecutionState::RollbackIfNeeded),
            ExecutionState::RollbackIfNeeded => Some(ExecutionState::AuditCommitted),
            ExecutionState::AuditCommitted => None, // 终点
        }
    }

    /// 是否为终态
    pub fn is_terminal(self) -> bool {
        matches!(self, ExecutionState::AuditCommitted)
    }

    /// 是否为失败状态
    pub fn is_failure(self) -> bool {
        matches!(self, ExecutionState::Failed)
    }

    pub fn name(&self) -> &'static str {
        match self {
            ExecutionState::Received => "RECEIVED",
            ExecutionState::IntentSchemaValidated => "INTENT_SCHEMA_VALIDATED",
            ExecutionState::CapabilityResolved => "CAPABILITY_RESOLVED",
            ExecutionState::PlatformSupported => "PLATFORM_SUPPORTED",
            ExecutionState::PolicyChecked => "POLICY_CHECKED",
            ExecutionState::PreconditionChecked => "PRECONDITION_CHECKED",
            ExecutionState::SnapshotTaken => "SNAPSHOT_TAKEN",
            ExecutionState::DryRunCompleted => "DRY_RUN_COMPLETED",
            ExecutionState::ConfirmationChecked => "CONFIRMATION_CHECKED",
            ExecutionState::Executing => "EXECUTING",
            ExecutionState::Verifying => "VERIFYING",
            ExecutionState::Succeeded => "SUCCEEDED",
            ExecutionState::Failed => "FAILED",
            ExecutionState::RollbackIfNeeded => "ROLLBACK_IF_NEEDED",
            ExecutionState::AuditCommitted => "AUDIT_COMMITTED",
        }
    }
}