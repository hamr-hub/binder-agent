//! binder-host-android-bin: Android Host Adapter Service

use async_trait::async_trait;
use binder_host_android::AndroidAdapter;
use binder_rpc::{
    AdapterError, DryRunResult, ExecuteResult, HostAdapter, HostAdapterRpcServer, HostInfo,
    PrecheckResult, RollbackResult, Snapshot, VerifyResult,
};
use jsonrpsee::server::ServerBuilder;
use std::net::SocketAddr;
use tracing::info;

struct RpcServer {
    adapter: AndroidAdapter,
}

#[async_trait]
impl HostAdapterRpcServer for RpcServer {
    async fn describe_host(&self) -> Result<HostInfo, AdapterError> {
        self.adapter.describe_host().await
    }

    async fn list_capabilities(&self) -> Result<Vec<String>, AdapterError> {
        self.adapter.list_capabilities().await
    }

    async fn precheck(
        &self,
        capability_id: String,
        params: serde_json::Value,
    ) -> Result<PrecheckResult, AdapterError> {
        self.adapter.precheck(&capability_id, &params).await
    }

    async fn snapshot(
        &self,
        capability_id: String,
        params: serde_json::Value,
    ) -> Result<Snapshot, AdapterError> {
        self.adapter.snapshot(&capability_id, &params).await
    }

    async fn dry_run(
        &self,
        capability_id: String,
        params: serde_json::Value,
    ) -> Result<DryRunResult, AdapterError> {
        self.adapter.dry_run(&capability_id, &params).await
    }

    async fn execute(
        &self,
        capability_id: String,
        params: serde_json::Value,
    ) -> Result<ExecuteResult, AdapterError> {
        self.adapter.execute(&capability_id, &params).await
    }

    async fn verify(
        &self,
        capability_id: String,
        params: serde_json::Value,
        result: ExecuteResult,
    ) -> Result<VerifyResult, AdapterError> {
        self.adapter.verify(&capability_id, &params, &result).await
    }

    async fn rollback(
        &self,
        capability_id: String,
        snapshot: Snapshot,
    ) -> Result<RollbackResult, AdapterError> {
        self.adapter.rollback(&capability_id, &snapshot).await
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 5001));
    let server = ServerBuilder::default().build(addr).await?;

    let adapter = AndroidAdapter::new(None);
    let rpc_server = RpcServer { adapter };

    info!("Starting Android Host Adapter RPC server on {}", addr);

    let handle = server.start(rpc_server.into_rpc());
    
    // 等待 Ctrl-C
    tokio::signal::ctrl_c().await?;
    handle.stop()?;

    Ok(())
}
