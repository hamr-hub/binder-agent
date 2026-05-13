//! Capability Registry：管理所有注册的能力定义

use binder_schemas::capability::{Capability, InMemoryRegistry};
use std::collections::HashMap;

pub struct Registry {
    inner: InMemoryRegistry,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            inner: InMemoryRegistry::new(),
        }
    }

    /// 从 YAML 目录加载所有 Capability
    pub fn load_from_dir(&mut self, dir: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;
        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "yaml" || ext == "yml"))
        {
            let content = std::fs::read_to_string(entry.path())?;
            self.load_from_yaml(&content)?;
            count += 1;
        }
        Ok(count)
    }

    /// 从 YAML 内容（可能包含多个文档）加载 Capability
    pub fn load_from_yaml(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        for doc in serde_yaml::Deserializer::from_str(content) {
            let capability: Capability = serde_yaml::from_value(
                serde_yaml::Value::deserialize(doc)?
            )?;
            self.inner.register(capability)?;
        }
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Capability> {
        self.inner.get(id)
    }

    pub fn list_all(&self) -> Vec<&Capability> {
        self.inner.list_all()
    }

    pub fn list_by_platform(&self, platform: binder_schemas::Platform) -> Vec<&Capability> {
        self.inner.list_by_platform(platform)
    }
}

pub mod resolver {
    //! Capability Resolver：根据 intent_type + platform 解析具体执行绑定
}