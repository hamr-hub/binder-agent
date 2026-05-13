use binder_schemas::intent::{Intent, IntentValidator as SchemaValidator};

/// Intent 校验结果
#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Intent 校验器
pub struct Validator {
    schema_validator: SchemaValidator,
}

impl Validator {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            schema_validator: SchemaValidator::new()?,
        })
    }

    /// 对 Intent 进行完整校验
    pub fn validate(&self, intent_json: &serde_json::Value) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Step 1: JSON Schema 结构校验
        if let Err(schema_errors) = self.schema_validator.validate(intent_json) {
            errors.extend(schema_errors);
        }

        // Step 2: 反序列化为 Intent 类型
        match serde_json::from_value::<Intent>(intent_json.clone()) {
            Ok(intent) => {
                // Step 3: 安全校验
                if let Err(e) = self.schema_validator.security_check(&intent) {
                    errors.push(e);
                }

                // Step 4: 置信度范围
                if intent.confidence < 0.0 || intent.confidence > 1.0 {
                    errors.push(format!(
                        "confidence out of range [0,1]: {}",
                        intent.confidence
                    ));
                }
            }
            Err(e) => {
                errors.push(format!("Failed to deserialize intent: {}", e));
            }
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

// Placeholder files for normalizer and router