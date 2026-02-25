use anyhow::Result;
use serde_json::{Value, json};
use std::process::Command;
use crate::core::Claw;

pub struct ProcessClaw;

impl Claw for ProcessClaw {
    fn name(&self) -> &'static str { "process" }

    fn description(&self) -> &'static str {
        "Process execution and inspection"
    }

    fn execute(&self, args: Value) -> Result<Value> {
        let program = args["program"].as_str().ok_or_else(|| anyhow::anyhow!("missing program"))?;
        let output = Command::new(program).output()?;
        Ok(json!({
            "stdout": String::from_utf8_lossy(&output.stdout),
            "status": output.status.code()
        }))
    }
}
