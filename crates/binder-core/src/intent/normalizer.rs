//! Intent 规范化器：补全默认值、标准化字段名
//!
//! 将 LLM 输出的 Intent 规范化，补全缺失字段、标准化命名。

use binder_schemas::intent::Intent;
use binder_schemas::Platform;
use uuid::Uuid;

/// 规范化 Intent，补全缺失字段
pub fn normalize(intent: &mut Intent, default_platform: Platform) {
    // 补全 intent_id（如果缺失或格式不对）
    if intent.intent_id.is_empty() {
        intent.intent_id = Uuid::new_v4().to_string();
    }

    // 补全 platform（如果缺失）
    if intent.target.device_id.is_empty() {
        intent.target.device_id = "local".to_string();
    }

    // 如果 LLM 返回了错误的 platform，使用默认值
    if intent.target.platform != default_platform {
        tracing::warn!(
            "Normalizing platform from {:?} to {:?}",
            intent.target.platform, default_platform
        );
        intent.target.platform = default_platform;
    }

    // 如果 confidence 超出范围，归一化到 0.5
    if intent.confidence < 0.0 || intent.confidence > 1.0 {
        tracing::warn!(
            "Normalizing confidence from {} to 0.5",
            intent.confidence
        );
        intent.confidence = 0.5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binder_schemas::intent::{RiskHint, Target};
    use serde_json::json;

    #[test]
    fn test_normalize_empty_intent_id() {
        let mut intent = Intent {
            intent_id: "".to_string(),
            intent_type: "android.device.info".to_string(),
            target: Target {
                platform: Platform::Android,
                device_id: "".to_string(),
            },
            parameters: json!({}),
            risk_hint: RiskHint::ReadOnly,
            confidence: 0.95,
            needs_clarification: false,
            clarification_question: None,
            natural_language_reason: None,
        };

        normalize(&mut intent, Platform::Android);
        assert!(!intent.intent_id.is_empty());
        assert_eq!(intent.target.device_id, "local");
    }

    #[test]
    fn test_normalize_confidence_out_of_range() {
        let mut intent = Intent {
            intent_id: "test-001".to_string(),
            intent_type: "android.device.info".to_string(),
            target: Target {
                platform: Platform::Linux,
                device_id: "local".to_string(),
            },
            parameters: json!({}),
            risk_hint: RiskHint::Low,
            confidence: 2.0,
            needs_clarification: false,
            clarification_question: None,
            natural_language_reason: None,
        };

        normalize(&mut intent, Platform::Android);
        assert_eq!(intent.confidence, 0.5);
        assert_eq!(intent.target.platform, Platform::Android);
    }
}