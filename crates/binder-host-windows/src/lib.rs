//! binder-host-windows: Windows Host Adapter
//!
//! 通过 Windows UI Automation、Win32 API 等提供 Windows 平台能力。
//! Phase 1: app.open, app.close, ui.inspect, system.info.get

use async_trait::async_trait;
use binder_rpc::{HostAdapter, HostInfo, AdapterError, ExecuteResult};

pub struct WindowsAdapter;

#[async_trait]
impl HostAdapter for WindowsAdapter {
    async fn describe_host(&self) -> Result<HostInfo, AdapterError> {
        Ok(HostInfo {
            hostname: hostname::get()
                .map(|h| h.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
            os: "windows".into(),
            arch: std::env::consts::ARCH.to_string(),
            os_version: "unknown".into(),
            adapter_version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    async fn list_capabilities(&self) -> Result<Vec<String>, AdapterError> {
        Ok(vec![
            "system.info.get".into(),
            "app.open".into(),
            "app.close".into(),
            "ui.inspect".into(),
            "file.read_confined".into(),
        ])
    }

    async fn precheck(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::PrecheckResult, AdapterError> {
        todo!("Windows precheck")
    }

    async fn snapshot(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::Snapshot, AdapterError> {
        todo!("Windows snapshot")
    }

    async fn dry_run(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::DryRunResult, AdapterError> {
        todo!("Windows dry_run")
    }

    async fn execute(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<ExecuteResult, AdapterError> {
        todo!("Windows execute")
    }

    async fn verify(&self, _capability_id: &str, _params: &serde_json::Value, _result: &ExecuteResult) -> Result<binder_rpc::VerifyResult, AdapterError> {
        todo!("Windows verify")
    }

    async fn rollback(&self, _capability_id: &str, _snapshot: &binder_rpc::Snapshot) -> Result<binder_rpc::RollbackResult, AdapterError> {
        todo!("Windows rollback")
    }
}