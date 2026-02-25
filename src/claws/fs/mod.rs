use anyhow::Result;
use serde_json::{Value, json};
use std::fs;
use crate::core::Claw;

pub struct FsClaw;

impl Claw for FsClaw {
    fn name(&self) -> &'static str { "fs" }

    fn description(&self) -> &'static str {
        "Filesystem read operations"
    }

    fn execute(&self, args: Value) -> Result<Value> {
        let path = args["path"].as_str().ok_or_else(|| anyhow::anyhow!("missing path"))?;
        let content = fs::read_to_string(path)?;
        Ok(json!({ "content": content }))
    }
}
