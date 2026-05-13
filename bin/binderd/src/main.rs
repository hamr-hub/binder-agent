//! binderd: Binder Core 守护进程
//!
//! 启动 Binder Core，加载 Capability，连接 Host Adapters，
//! 提供 JSON-RPC 服务供 binderctl 调用。

use binder_core::{
    audit::logger::AuditLogger,
    capability::registry::Registry,
    execution::state_machine::ExecutionState,
    intent::validator::Validator,
    policy::engine::PolicyEngine,
};
use binder_schemas::{Platform, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "binderd=info".into()),
        )
        .init();

    tracing::info!("Starting binderd...");

    // 初始化 Audit Logger (SQLite)
    let audit_db_path = std::env::var("BINDER_AUDIT_DB")
        .unwrap_or_else(|_| "/var/lib/binder/audit.db".to_string());
    // 确保目录存在
    if let Some(parent) = std::path::Path::new(&audit_db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    let audit_logger = AuditLogger::new(&audit_db_path)?;
    tracing::info!("Audit logger initialized at {}", audit_db_path);

    // 初始化 Capability Registry
    let mut registry = Registry::new();
    let cap_dir = std::env::var("BINDER_CAPABILITIES_DIR")
        .unwrap_or_else(|_| "capabilities".to_string());
    let count = registry.load_from_dir(&cap_dir)?;
    tracing::info!("Loaded {} capabilities from {}", count, cap_dir);

    // 初始化 Policy Engine
    let policy_engine = PolicyEngine::new();

    // 初始化 Intent Validator
    let validator = Validator::new()?;

    tracing::info!("binderd is ready. Waiting for intents...");

    // TODO: Phase 1.2 - 启动 JSON-RPC 服务
    // TODO: Phase 1.3 - 连接各 Host Adapter

    // 简单保持运行
    tokio::signal::ctrl_c().await?;
    tracing::info!("binderd shutting down.");

    Ok(())
}