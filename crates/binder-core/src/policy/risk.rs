//! 风险等级判定模块
//!
//! Phase 1: 直接使用 Capability 定义的 risk_level，不做额外判定。
//! Phase 2+: 可根据上下文（网络区域、设备信任等级、MFA状态等）动态调整。