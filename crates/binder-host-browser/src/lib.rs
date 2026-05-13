//! binder-host-browser: Browser Host Adapter
//!
//! 通过 Chromium CDP (Chrome DevTools Protocol) 控制浏览器。
//! Phase 1: url alias 导航、文本验证、已知元素点击、已知字段填写。
//! Phase 2: 使用 chromiumoxide 实现完整的 Playwright-like API。

use async_trait::async_trait;
use binder_rpc::{HostAdapter, HostInfo, AdapterError, ExecuteResult};

pub struct BrowserAdapter;

#[async_trait]
impl HostAdapter for BrowserAdapter {
    async fn describe_host(&self) -> Result<HostInfo, AdapterError> {
        Ok(HostInfo {
            hostname: "browser".into(),
            os: "browser".into(),
            arch: "any".into(),
            os_version: "chromium".into(),
            adapter_version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    async fn list_capabilities(&self) -> Result<Vec<String>, AdapterError> {
        Ok(vec![
            "browser.open_url".into(),
            "browser.verify_text".into(),
            "browser.click_known_element".into(),
            "browser.fill_known_field".into(),
        ])
    }

    async fn precheck(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::PrecheckResult, AdapterError> {
        todo!("Browser precheck")
    }

    async fn snapshot(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::Snapshot, AdapterError> {
        todo!("Browser snapshot")
    }

    async fn dry_run(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<binder_rpc::DryRunResult, AdapterError> {
        todo!("Browser dry_run")
    }

    async fn execute(&self, _capability_id: &str, _params: &serde_json::Value) -> Result<ExecuteResult, AdapterError> {
        todo!("Browser execute")
    }

    async fn verify(&self, _capability_id: &str, _params: &serde_json::Value, _result: &ExecuteResult) -> Result<binder_rpc::VerifyResult, AdapterError> {
        todo!("Browser verify")
    }

    async fn rollback(&self, _capability_id: &str, _snapshot: &binder_rpc::Snapshot) -> Result<binder_rpc::RollbackResult, AdapterError> {
        todo!("Browser rollback")
    }
}