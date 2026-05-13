//! 并发资源锁管理
//!
//! 确保同一资源不会被多个 Intent 同时操作。
//! Phase 1: 简单的内存 Mutex 实现。

use std::collections::HashMap;
use std::sync::Mutex;

/// 资源锁管理器
pub struct LockManager {
    /// 被锁定的资源: capability_id -> intent_id
    locks: Mutex<HashMap<String, String>>,
}

impl LockManager {
    pub fn new() -> Self {
        Self {
            locks: Mutex::new(HashMap::new()),
        }
    }

    /// 尝试锁定资源，成功返回 true
    pub fn try_lock(&self, capability_id: &str, intent_id: &str) -> bool {
        let mut locks = self.locks.lock().unwrap();
        if locks.contains_key(capability_id) {
            false
        } else {
            locks.insert(capability_id.to_string(), intent_id.to_string());
            true
        }
    }

    /// 解锁资源
    pub fn unlock(&self, capability_id: &str) {
        let mut locks = self.locks.lock().unwrap();
        locks.remove(capability_id);
    }

    /// 检查资源是否被锁定
    pub fn is_locked(&self, capability_id: &str) -> bool {
        let locks = self.locks.lock().unwrap();
        locks.contains_key(capability_id)
    }

    /// 获取锁定资源的 intent_id
    pub fn get_lock_owner(&self, capability_id: &str) -> Option<String> {
        let locks = self.locks.lock().unwrap();
        locks.get(capability_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_and_unlock() {
        let manager = LockManager::new();
        assert!(manager.try_lock("android.app.launch", "intent-001"));
        assert!(!manager.try_lock("android.app.launch", "intent-002")); // Already locked
        manager.unlock("android.app.launch");
        assert!(manager.try_lock("android.app.launch", "intent-003")); // Unlocked, can lock again
    }

    #[test]
    fn test_get_lock_owner() {
        let manager = LockManager::new();
        manager.try_lock("android.screen.click", "intent-001");
        assert_eq!(manager.get_lock_owner("android.screen.click"), Some("intent-001".to_string()));
        assert_eq!(manager.get_lock_owner("android.device.info"), None);
    }
}