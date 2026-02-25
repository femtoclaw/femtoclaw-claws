use anyhow::Result;
use serde_json::{Value, json};
use std::process::Command;
use crate::core::Claw;

pub struct ShellClaw;

impl Claw for ShellClaw {
    fn name(&self) -> &'static str { "shell" }

    fn description(&self) -> &'static str {
        "Execute authorized shell commands"
    }

    fn execute(&self, args: Value) -> Result<Value> {
        let bin = args["bin"].as_str().ok_or_else(|| anyhow::anyhow!("missing bin"))?;
        let argv = args["argv"].as_array().ok_or_else(|| anyhow::anyhow!("missing argv"))?;

        let mut cmd = Command::new(bin);
        for arg in argv {
            cmd.arg(arg.as_str().unwrap());
        }

        let output = cmd.output()?;

        Ok(json!({
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "status": output.status.code()
        }))
    }
}
