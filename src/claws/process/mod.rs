use crate::core::Claw;
use anyhow::Result;
use serde_json::{json, Value};
use std::process::Command;

pub struct ProcessClaw;

impl Claw for ProcessClaw {
    fn name(&self) -> &'static str {
        "process"
    }

    fn description(&self) -> &'static str {
        "Process execution and inspection"
    }

    fn execute(&self, args: Value) -> Result<Value> {
        let program = args["program"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing program"))?;
        let allowlist = process_allowlist();
        if !allowlist.iter().any(|allowed| allowed == program) {
            return Err(anyhow::anyhow!("program not allowed"));
        }

        // Optional "args" array; if absent, no arguments.
        let args_vec: Vec<String> = args["args"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        if args_vec.len() > 16 {
            return Err(anyhow::anyhow!("too many args"));
        }
        for arg in &args_vec {
            if arg.len() > 4096 {
                return Err(anyhow::anyhow!("arg too long"));
            }
        }

        let output = Command::new(program).args(args_vec).output()?;
        let stdout = truncate_utf8_lossy(&output.stdout, 32_768);
        let stderr = truncate_utf8_lossy(&output.stderr, 32_768);
        Ok(json!({
            "stdout": stdout,
            "stderr": stderr,
            "status": output.status.code(),
        }))
    }
}

fn process_allowlist() -> Vec<String> {
    std::env::var("FEMTO_PROCESS_ALLOWLIST")
        .ok()
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|entry| !entry.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .filter(|entries: &Vec<String>| !entries.is_empty())
        .unwrap_or_else(|| vec!["echo".to_string()])
}

fn truncate_utf8_lossy(bytes: &[u8], max_len: usize) -> String {
    let mut out = String::from_utf8_lossy(bytes).to_string();
    if out.len() > max_len {
        out.truncate(max_len);
        out.push_str("\n...(truncated)...");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::ProcessClaw;
    use crate::core::Claw;
    use serde_json::json;

    #[test]
    fn rejects_programs_not_in_allowlist() {
        let result = ProcessClaw.execute(json!({
            "program": "pwd",
            "args": []
        }));
        assert!(result.is_err());
    }
}
