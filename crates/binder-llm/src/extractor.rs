//! Intent 提取器：调用 LLM 将自然语言转换为结构化 Intent

use crate::client::{ChatMessage, LlmClient};
use crate::prompts;
use binder_schemas::intent::Intent;
use binder_schemas::Platform;

pub struct IntentExtractor {
    client: LlmClient,
}

impl IntentExtractor {
    pub fn new() -> Self {
        Self {
            client: LlmClient::new(),
        }
    }

    /// 从自然语言提取 Intent
    ///
    /// # Arguments
    /// * `user_input` - 用户的自然语言输入
    /// * `platform` - 目标平台
    /// * `available_capabilities` - 当前可用的 Capability ID 列表（已过滤到 5-20 个）
    pub async fn extract(
        &self,
        user_input: &str,
        platform: Platform,
        available_capabilities: &[String],
    ) -> Result<Intent, Box<dyn std::error::Error>> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: prompts::get_system_prompt(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompts::build_user_message(user_input, available_capabilities),
            },
        ];

        let response = self.client.chat(messages, None).await?;

        // 解析 LLM 返回的 JSON 为 Intent
        let intent: Intent = serde_json::from_str(&response)?;

        // 校验 platform 一致性
        if intent.target.platform != platform {
            tracing::warn!(
                "LLM returned platform {:?} but expected {:?}",
                intent.target.platform, platform
            );
        }

        Ok(intent)
    }
}