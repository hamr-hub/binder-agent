//! binder-llm: LLM Intent Agent
//!
//! 使用 OpenAI 兼容 API 将自然语言转换为结构化 Intent。
//! LLM 只输出 Intent，不输出可执行命令。

pub mod client;
pub mod extractor;
pub mod prompts;