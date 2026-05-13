//! binder-host-android: Android Host Adapter
//!
//! 通过 ADB (Android Debug Bridge) 提供 Android 设备控制能力。
//! 支持能力: android.device.info, android.app.launch, android.screen.click

use async_trait::async_trait;
use binder_rpc::{
    AdapterError, DryRunResult, ExecuteResult, HostAdapter, HostInfo, PrecheckResult,
    RollbackResult, Snapshot, VerifyResult,
};
use serde_json::json;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, error, info};

pub struct AndroidAdapter {
    device_id: Option<String>,
}

impl AndroidAdapter {
    pub fn new(device_id: Option<String>) -> Self {
        Self { device_id }
    }

    async fn run_adb(&self, args: &[&str]) -> Result<String, AdapterError> {
        let mut cmd = Command::new("adb");
        if let Some(ref id) = self.device_id {
            cmd.arg("-s").arg(id);
        }
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        debug!("Running adb command: {:?}", cmd);

        let output = cmd.spawn().map_err(|e| {
            AdapterError::Internal(format!("Failed to spawn adb: {}", e))
        })?.wait_with_output().await.map_err(|e| {
            AdapterError::Internal(format!("Failed to wait for adb: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(AdapterError::ExecutionFailed(format!(
                "adb command failed: {}",
                stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    async fn run_adb_shell(&self, shell_cmd: &str) -> Result<String, AdapterError> {
        self.run_adb(&["shell", shell_cmd]).await
    }
}

#[async_trait]
impl HostAdapter for AndroidAdapter {
    async fn describe_host(&self) -> Result<HostInfo, AdapterError> {
        let model = self.run_adb_shell("getprop ro.product.model").await.unwrap_or_else(|_| "unknown".into());
        let version = self.run_adb_shell("getprop ro.build.version.release").await.unwrap_or_else(|_| "unknown".into());
        
        Ok(HostInfo {
            hostname: format!("android-{}", model),
            os: "android".into(),
            arch: self.run_adb_shell("getprop ro.product.cpu.abi").await.unwrap_or_else(|_| "unknown".into()),
            os_version: version,
            adapter_version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    async fn list_capabilities(&self) -> Result<Vec<String>, AdapterError> {
        Ok(vec![
            "android.device.info".into(),
            "android.app.launch".into(),
            "android.screen.click".into(),
        ])
    }

    async fn precheck(
        &self,
        capability_id: &str,
        _params: &serde_json::Value,
    ) -> Result<PrecheckResult, AdapterError> {
        // 检查 adb 是否可用以及设备是否在线
        match self.run_adb(&["get-state"]).await {
            Ok(state) if state == "device" => Ok(PrecheckResult {
                ok: true,
                reason: None,
                warnings: vec![],
            }),
            Ok(state) => Ok(PrecheckResult {
                ok: false,
                reason: Some(format!("Device state is not 'device': {}", state)),
                warnings: vec![],
            }),
            Err(e) => Ok(PrecheckResult {
                ok: false,
                reason: Some(format!("Failed to check device state: {}", e)),
                warnings: vec![],
            }),
        }
    }

    async fn snapshot(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<Snapshot, AdapterError> {
        match capability_id {
            "android.app.launch" => {
                let package_name = params["package_name"].as_str().ok_or_else(|| {
                    AdapterError::Internal("Missing package_name in params".into())
                })?;
                // 记录应用当前是否正在运行
                let output = self.run_adb_shell(&format!("pidof {}", package_name)).await;
                let is_running = output.is_ok() && !output.unwrap().is_empty();
                
                Ok(Snapshot {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    capability_id: capability_id.into(),
                    state: json!({ "was_running": is_running, "package_name": package_name }),
                })
            }
            _ => Ok(Snapshot {
                timestamp: chrono::Utc::now().to_rfc3339(),
                capability_id: capability_id.into(),
                state: json!({}),
            }),
        }
    }

    async fn dry_run(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<DryRunResult, AdapterError> {
        match capability_id {
            "android.app.launch" => Ok(DryRunResult {
                ok: true,
                expected_effects: vec![format!("Launch app: {}", params["package_name"])],
                warnings: vec![],
            }),
            "android.screen.click" => Ok(DryRunResult {
                ok: true,
                expected_effects: vec![format!("Click screen at ({}, {})", params["x"], params["y"])],
                warnings: vec![],
            }),
            _ => Ok(DryRunResult {
                ok: true,
                expected_effects: vec![],
                warnings: vec![],
            }),
        }
    }

    async fn execute(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<ExecuteResult, AdapterError> {
        let start = std::time::Instant::now();
        match capability_id {
            "android.device.info" => {
                let model = self.run_adb_shell("getprop ro.product.model").await?;
                let version = self.run_adb_shell("getprop ro.build.version.release").await?;
                let battery = self.run_adb_shell("dumpsys battery | grep level").await?;
                
                Ok(ExecuteResult {
                    ok: true,
                    message: Some("Successfully retrieved device info".into()),
                    output: Some(json!({
                        "model": model,
                        "version": version,
                        "battery": battery,
                    })),
                    duration_ms: start.elapsed().as_millis() as u64,
                })
            }
            "android.app.launch" => {
                let package_name = params["package_name"].as_str().ok_or_else(|| {
                    AdapterError::Internal("Missing package_name".into())
                })?;
                
                self.run_adb_shell(&format!("monkey -p {} -c android.intent.category.LAUNCHER 1", package_name)).await?;
                
                Ok(ExecuteResult {
                    ok: true,
                    message: Some(format!("Launched app {}", package_name)),
                    output: Some(json!({ "launched": true })),
                    duration_ms: start.elapsed().as_millis() as u64,
                })
            }
            "android.screen.click" => {
                let x = params["x"].as_i64().ok_or_else(|| AdapterError::Internal("Missing x".into()))?;
                let y = params["y"].as_i64().ok_or_else(|| AdapterError::Internal("Missing y".into()))?;
                
                self.run_adb_shell(&format!("input tap {} {}", x, y)).await?;
                
                Ok(ExecuteResult {
                    ok: true,
                    message: Some(format!("Clicked at ({}, {})", x, y)),
                    output: Some(json!({ "clicked": true })),
                    duration_ms: start.elapsed().as_millis() as u64,
                })
            }
            _ => Err(AdapterError::CapabilityNotFound(capability_id.into())),
        }
    }

    async fn verify(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
        _result: &ExecuteResult,
    ) -> Result<VerifyResult, AdapterError> {
        match capability_id {
            "android.app.launch" => {
                let package_name = params["package_name"].as_str().ok_or_else(|| {
                    AdapterError::Internal("Missing package_name".into())
                })?;
                // 简单校验：通过 dumpsys window windows | grep mCurrentFocus 看看包名是否在前台
                let output = self.run_adb_shell("dumpsys window windows | grep mCurrentFocus").await?;
                let ok = output.contains(package_name);
                
                Ok(VerifyResult {
                    ok,
                    observed: Some(json!({ "current_focus": output })),
                    diff: None,
                })
            }
            "android.screen.click" => {
                // 点击很难验证结果，除非截图做图像对比。MVP 暂时认为点击即成功。
                Ok(VerifyResult {
                    ok: true,
                    observed: None,
                    diff: None,
                })
            }
            _ => Ok(VerifyResult {
                ok: true,
                observed: None,
                diff: None,
            }),
        }
    }

    async fn rollback(
        &self,
        capability_id: &str,
        snapshot: &Snapshot,
    ) -> Result<RollbackResult, AdapterError> {
        match capability_id {
            "android.app.launch" => {
                let was_running = snapshot.state["was_running"].as_bool().unwrap_or(false);
                let package_name = snapshot.state["package_name"].as_str().ok_or_else(|| {
                    AdapterError::Internal("Missing package_name in snapshot".into())
                })?;
                
                if !was_running {
                    info!("Rolling back android.app.launch: stopping app {}", package_name);
                    self.run_adb_shell(&format!("am force-stop {}", package_name)).await?;
                }
                
                Ok(RollbackResult {
                    ok: true,
                    message: Some(format!("App {} stopped if it wasn't running before", package_name)),
                })
            }
            _ => Ok(RollbackResult {
                ok: true,
                message: Some("No rollback needed".into()),
            }),
        }
    }
}
