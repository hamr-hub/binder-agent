//! Capability Resolver：根据 intent_type + platform 解析具体执行绑定
//!
//! 从 Capability Registry 中查找匹配的 Capability，
//! 然后根据 platform 解析出对应的 Adapter 和方法绑定。

use binder_schemas::capability::{Capability, CapabilityBinding};
use binder_schemas::Platform;

/// 解析结果
#[derive(Debug)]
pub struct ResolvedBinding {
    pub capability: Capability,
    pub binding: CapabilityBinding,
}

/// Capability 解析器
pub struct Resolver;

impl Resolver {
    pub fn new() -> Self {
        Self
    }

    /// 根据 intent_type + platform 从 Capability 列表中解析绑定
    pub fn resolve<'a>(
        &self,
        intent_type: &str,
        platform: Platform,
        capabilities: &'a [Capability],
    ) -> Option<ResolvedBinding> {
        let capability = capabilities.iter().find(|c| {
            c.id == intent_type
                && c.supported_platforms.iter().any(|ps| ps.os == platform)
        })?;

        let platform_key = platform.to_string();
        let binding = capability.bindings.get(&platform_key)?;

        Some(ResolvedBinding {
            capability: capability.clone(),
            binding: binding.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binder_schemas::capability::*;
    use serde_json::json;

    #[test]
    fn test_resolve_android_capability() {
        let capabilities = vec![Capability {
            id: "android.app.launch".to_string(),
            version: 1,
            description: "Launch app".to_string(),
            risk_level: binder_schemas::RiskLevel::Low,
            reversible: true,
            requires_confirmation: false,
            input_schema: json!({}),
            supported_platforms: vec![PlatformSupport {
                os: Platform::Android,
                arch: Some(vec!["arm64".to_string()]),
            }],
            bindings: HashMap::from([(
                "android".to_string(),
                CapabilityBinding {
                    adapter: "binder-host-android".to_string(),
                    method: "app.launch".to_string(),
                    verifier: Some("app.is_running".to_string()),
                    rollback: Some("app.force_stop".to_string()),
                },
            )]),
            success_criteria: None,
            rollback: None,
        }];

        let resolver = Resolver::new();
        let result = resolver.resolve("android.app.launch", Platform::Android, &capabilities);
        assert!(result.is_some());
        let resolved = result.unwrap();
        assert_eq!(resolved.binding.adapter, "binder-host-android");
        assert_eq!(resolved.binding.method, "app.launch");
    }

    #[test]
    fn test_resolve_unknown_capability() {
        let capabilities = vec![];
        let resolver = Resolver::new();
        let result = resolver.resolve("unknown.cap", Platform::Android, &capabilities);
        assert!(result.is_none());
    }
}