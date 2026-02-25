use anyhow::Result;
use serde_json::{Value, json};
use crate::core::Claw;

pub struct NetClaw;

impl Claw for NetClaw {
    fn name(&self) -> &'static str { "net" }

    fn description(&self) -> &'static str {
        "Network operations abstraction"
    }

    fn execute(&self, args: Value) -> Result<Value> {
        let url = args["url"].as_str().ok_or_else(|| anyhow::anyhow!("missing url"))?;
        Ok(json!({ "requested": url }))
    }
}
