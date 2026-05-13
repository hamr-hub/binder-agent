use binder_host_android::AndroidAdapter;
use binder_rpc::HostAdapter;
use serde_json::json;

#[tokio::test]
async fn test_android_adapter_mock_e2e() {
    let adapter = AndroidAdapter::new_mock();
    
    // 1. Describe Host
    let host_info = adapter.describe_host().await.unwrap();
    assert_eq!(host_info.os, "android");
    assert_eq!(host_info.hostname, "android-Mock Device");

    // 2. List Capabilities
    let caps = adapter.list_capabilities().await.unwrap();
    assert!(caps.contains(&"android.device.info".to_string()));
    assert!(caps.contains(&"android.app.launch".to_string()));

    // 3. Precheck
    let precheck = adapter.precheck("android.device.info", &json!({})).await.unwrap();
    assert!(precheck.ok);

    // 4. Execute Get Info
    let result = adapter.execute("android.device.info", &json!({})).await.unwrap();
    assert!(result.ok);
    assert!(result.output.unwrap()["model"].as_str().unwrap().contains("Mock Device"));

    // 5. App Launch with Verification and Rollback
    let params = json!({ "package_name": "com.android.settings" });
    
    // Snapshot
    let snapshot = adapter.snapshot("android.app.launch", &params).await.unwrap();
    
    // Execute
    let launch_result = adapter.execute("android.app.launch", &params).await.unwrap();
    assert!(launch_result.ok);
    
    // Verify
    let verify = adapter.verify("android.app.launch", &params, &launch_result).await.unwrap();
    assert!(verify.ok);
    
    // Rollback (mocking that it wasn't running)
    let mut fake_snapshot = snapshot;
    fake_snapshot.state["was_running"] = json!(false);
    let rollback = adapter.rollback("android.app.launch", &fake_snapshot).await.unwrap();
    assert!(rollback.ok);
}
