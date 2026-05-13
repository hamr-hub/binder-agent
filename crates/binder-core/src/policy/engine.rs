//! Policy Engine：权限策略决策引擎
//!
//! 支持 RBAC（基于角色）和风险等级判定。
//! Phase 1: 内存 HashMap 存储策略规则。

use binder_schemas::capability::Capability;
use binder_schemas::{RiskLevel, Role};

/// 策略决策结果
#[derive(Debug)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub requires_confirmation: bool,
    pub reason: String,
}

/// Policy Engine
pub struct PolicyEngine {
    rules: Vec<PolicyRule>,
}

struct PolicyRule {
    role: Role,
    max_risk: RiskLevel,
    description: String,
}

impl PolicyEngine {
    pub fn new() -> Self {
        let mut engine = Self { rules: Vec::new() };
        engine.add_rule(Role::User, RiskLevel::Low, "用户只能执行低风险操作");
        engine.add_rule(Role::Operator, RiskLevel::Medium, "运维可执行中风险操作");
        engine.add_rule(Role::Admin, RiskLevel::High, "管理员可执行高风险操作");
        engine.add_rule(Role::DeviceOwner, RiskLevel::Critical, "设备所有者可执行所有操作");
        engine
    }

    fn add_rule(&mut self, role: Role, max_risk: RiskLevel, description: &str) {
        self.rules.push(PolicyRule { role, max_risk, description: description.to_string() });
    }

    pub fn evaluate(&self, capability: &Capability, role: Role) -> PolicyDecision {
        let max_risk = self.rules.iter()
            .find(|r| r.role == role)
            .map(|r| r.max_risk)
            .unwrap_or(RiskLevel::ReadOnly);

        if capability.risk_level > max_risk {
            return PolicyDecision {
                allowed: false,
                requires_confirmation: false,
                reason: format!("Role '{:?}' limited to '{:?}' risk, capability is '{:?}'",
                    role, max_risk, capability.risk_level),
            };
        }

        let requires_confirmation = capability.requires_confirmation
            || capability.risk_level.requires_confirmation();

        PolicyDecision {
            allowed: true,
            requires_confirmation,
            reason: if requires_confirmation {
                "Action requires user confirmation due to risk level".to_string()
            } else {
                "Action allowed by policy".to_string()
            },
        }
    }
}