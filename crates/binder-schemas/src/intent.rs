//! Intent Schema 定义与校验

use super::Platform;
use serde::{Deserialize, Serialize};

/// LLM 输出的意图结构
///
/// LLM 只能按此结构输出 Intent，不能包含任何可执行命令。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub intent_id: String,
    pub intent_type: String,
    pub target: Target,
    pub parameters: serde_json::Value,
    pub risk_hint: RiskHint,
    pub confidence: f64,
    #[serde(default)]
    pub needs_clarification: bool,
    #[serde(default)]
    pub clarification_question: Option<String>,
    #[serde(default)]
    pub natural_language_reason: Option<String>,
}

/// 意图目标（平台 + 设备）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub platform: Platform,
    pub device_id: String,
}

/// LLM 对风险的初步判断（仅供参考，不决定最终授权）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskHint {
    ReadOnly,
    Low,
    Medium,
    High,
    Critical,
}

/// 多意图拆解计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub plan_id: String,
    pub intents: Vec<Intent>,
}

/// Intent 校验器
pub struct IntentValidator {
    schema: jsonschema::JSONSchema,
}

impl IntentValidator {
    /// 创建 Intent 校验器，使用定义的 JSON Schema
    pub fn new() -> Result<Self, crate::capability::SchemaError> {
        let schema_json = include_str!("../../../schemas/intent.schema.json");
        let schema_value: serde_json::Value =
            serde_json::from_str(schema_json).map_err(|e| {
                crate::capability::SchemaError::LoadFailed(format!(
                    "Failed to parse intent schema: {}",
                    e
                ))
            })?;

        let schema = jsonschema::JSONSchema::compile(&schema_value).map_err(|e| {
            crate::capability::SchemaError::LoadFailed(format!(
                "Failed to compile intent schema: {}",
                e
            ))
        })?;

        Ok(Self { schema })
    }

    /// 校验 Intent JSON
    pub fn validate(&self, intent: &serde_json::Value) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        for error in self.schema.iter_errors(intent) {
            errors.push(format!("{}: {}", error.instance_path, error));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 额外安全校验：拒绝包含危险字段的 parameters
    pub fn security_check(&self, intent: &Intent) -> Result<(), String> {
        let dangerous_keys = [
            "cmd", "shell", "command", "code", "script", "exec", "eval", "run",
            "powershell", "bash", "sh", "python", "ruby", "perl", "php",
            "os_system", "subprocess", "popen", "spawn", "exec_shell",
        ];

        if let serde_json::Value::Object(params) = &intent.parameters {
            for key in params.keys() {
                let lower = key.to_lowercase();
                if dangerous_keys.iter().any(|dk| lower.contains(dk)) {
                    return Err(format!(
                        "Security rejection: parameters contain dangerous key '{}'",
                        key
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_intent() {
        let validator = IntentValidator::new().unwrap();
        let intent = json!({
            "intent_id": "test-001",
            "intent_type": "system.info.get",
            "target": {
                "platform": "linux",
                "device_id": "local"
            },
            "parameters": {},
            "risk_hint": "read_only",
            "confidence": 0.95
        });
        assert!(validator.validate(&intent).is_ok());
    }

    #[test]
    fn test_reject_extra_fields() {
        let validator = IntentValidator::new().unwrap();
        let intent = json!({
            "intent_id": "test-001",
            "intent_type": "system.info.get",
            "target": {
                "platform": "linux",
                "device_id": "local"
            },
            "parameters": {},
            "risk_hint": "read_only",
            "confidence": 0.95,
            "forbidden_field": "should_be_rejected"
        });
        assert!(validator.validate(&intent).is_err());
    }

    #[test]
    fn test_reject_unknown_intent_type() {
        let validator = IntentValidator::new().unwrap();
        let intent = json!({
            "intent_id": "test-001",
            "intent_type": "hack.rm_rf",
            "target": {
                "platform": "linux",
                "device_id": "local"
            },
            "parameters": {},
            "risk_hint": "read_only",
            "confidence": 0.95
        });
        assert!(validator.validate(&intent).is_err());
    }
}