//! binder-core: Binder Core 核心库
//!
//! 包含 Intent Gateway、Capability Registry、Policy Engine、
//! Execution State Machine、Verifier、Rollback Manager、Audit Logger。

pub mod intent;
pub mod capability;
pub mod policy;
pub mod execution;
pub mod verifier;
pub mod rollback;
pub mod audit;