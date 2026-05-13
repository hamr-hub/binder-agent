//! binder-host-linux: Linux Host Adapter
//!
//! 通过 /proc、/sys、systemd DBus 等 Linux 原生机制
//! 提供系统信息、网络状态、服务状态、进程列表、受限文件访问等能力。

use binder_rpc::{HostAdapter, HostInfo, AdapterError, ExecuteResult};
use async_trait::async_trait;

pub struct LinuxAdapter;

#[async_trait]
impl HostAdapter for LinuxAdapter {
    async fn describe_host(&self) -> Result<HostInfo, AdapterError> {
        todo!("Linux describe_host")
    }

    async fn list_capabilities(&self) -> Result<Vec<String>, AdapterError> {
        Ok(vec![
            "system.info.get".into(),
            "system.network.status".into(),
            "system.service.status".into(),
            "system.process.list".into(),
            "file.read_confined".into(),
            "file.write_confined".into(),
        ])
    }

    async fn precheck(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::PrecheckResult, AdapterError> {
        todo!("Linux precheck")
    }

    async fn snapshot(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::Snapshot, AdapterError> {
        todo!("Linux snapshot")
    }

    async fn dry_run(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::DryRunResult, AdapterError> {
        todo!("Linux dry_run")
    }

    async fn execute(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<ExecuteResult, AdapterError> {
        todo!("Linux execute")
    }

    async fn verify(&self, _capability_id: &str, _params: &serde_json::Value, _result: &ExecuteResult) -> Result<binder_rpc::VerifyResult, AdapterError> {
        todo!("Linux verify")
    }

    async fn rollback(&self, _capability_id: &str, _snapshot: &binder_rpc::Snapshot) -> Result<binder_rpc::RollbackResult, AdapterError> {
        todo!("Linux rollback")
    }
}