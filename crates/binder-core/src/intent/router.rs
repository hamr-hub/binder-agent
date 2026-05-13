//! Intent 路由器：根据 intent_type + platform 路由到对应 Capability

use binder_schemas::capability::Capability;
use binder_schemas::Platform;

pub struct Router;

impl Router {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve<'a>(
        &self,
        intent_type: &str,
        platform: Platform,
        capabilities: &'a [Capability],
    ) -> Option<&'a Capability> {
        capabilities.iter().find(|c| {
            c.id == intent_type
                && c.supported_platforms.iter().any(|ps| ps.os == platform)
        })
    }
}