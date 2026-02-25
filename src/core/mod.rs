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
        Self { claws: HashMap::new() }
    }

    pub fn register<C: Claw + 'static>(&mut self, claw: C) {
        self.claws.insert(claw.name(), Arc::new(claw));
    }

    pub fn execute(&self, name: &str, args: Value) -> Result<Value> {
        let claw = self.claws.get(name)
            .ok_or_else(|| anyhow::anyhow!("claw not found"))?;
        claw.execute(args)
    }
}
