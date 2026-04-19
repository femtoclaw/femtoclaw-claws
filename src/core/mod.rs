use anyhow::Result;
use serde_json::Value;

pub trait Claw: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, args: Value) -> Result<Value>;
}

use std::collections::HashMap;
use std::sync::Arc;

pub struct ClawRegistry {
    claws: HashMap<&'static str, Arc<dyn Claw>>,
}

impl ClawRegistry {
    pub fn new() -> Self {
        Self {
            claws: HashMap::new(),
        }
    }

    pub fn register<C: Claw + 'static>(&mut self, claw: C) {
        self.claws.insert(claw.name(), Arc::new(claw));
    }

    pub fn execute(&self, name: &str, args: Value) -> Result<Value> {
        let claw = self
            .claws
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("claw not found"))?;
        claw.execute(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claws::shell::ShellClaw;
    use serde_json::json;

    #[test]
    fn test_registry_register_and_execute() {
        let mut registry = ClawRegistry::new();
        registry.register(ShellClaw);

        // Execute shell successfully
        let res = registry
            .execute("shell", json!({"bin":"echo","argv":["hi"]}))
            .unwrap();
        assert!(res
            .get("stdout")
            .and_then(|v| v.as_str())
            .unwrap()
            .contains("hi"));

        // Unknown claw
        assert!(registry.execute("unknown", json!({})).is_err());
    }
}
