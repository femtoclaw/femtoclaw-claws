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
        // Optional "args" array; if absent, no arguments.
        let args_vec: Vec<String> = args["args"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let output = Command::new(program).args(args_vec).output()?;
        Ok(json!({
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "status": output.status.code(),
        }))
    }
}
