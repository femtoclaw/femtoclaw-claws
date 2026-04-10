use anyhow::Result;
use serde_json::{Value, json};
use std::process::Command;
use crate::core::Claw;

/// ShellClaw — synchronous command execution with bin + argv.
///
/// Accepts arguments in any of these shapes:
/// - { "bin": "ls", "argv": ["-la"] }
/// - { "command": "ls -la" }  (auto-split)
/// - { "cmd": "echo", "args": ["hi"] }
///
/// Returns:
/// { "stdout": "...", "stderr": "...", "exit_code": 0 }
pub struct ShellClaw;

impl Claw for ShellClaw {
    fn name(&self) -> &'static str { "shell" }

    fn description(&self) -> &'static str {
        "Allowlisted argv process execution (no shell interpolation)."
    }

    fn execute(&self, args: Value) -> Result<Value> {
        // Extract bin (with alias support).
        let raw_bin = args
            .get("bin")
            .or_else(|| args.get("command"))
            .or_else(|| args.get("cmd"))
            .or_else(|| args.get("executable"))
            .or_else(|| args.get("program"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("shell: missing bin/command field"))?;

        // Extract argv list if provided.
        let raw_argv: Vec<String> = args
            .get("argv")
            .or_else(|| args.get("args"))
            .or_else(|| args.get("arguments"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Normalize into (bin, argv).
        let (bin, argv) = if raw_bin.chars().any(char::is_whitespace) {
            // Split the raw_bin string on whitespace.
            let mut parts: Vec<String> = raw_bin.split_whitespace().map(String::from).collect();
            if !raw_argv.is_empty() {
                parts.append(&mut raw_argv.clone());
            }
            if parts.is_empty() {
                return Err(anyhow::anyhow!("shell: empty command"));
            }
            let bin = parts.remove(0);
            (bin, parts)
        } else {
            let bin = raw_bin.to_string();
            let argv = raw_argv.clone();
            (bin, argv)
        };

        // Basic sanity checks.
        if bin.is_empty() {
            return Err(anyhow::anyhow!("shell: bin cannot be empty"));
        }
        if argv.len() > 16 {
            return Err(anyhow::anyhow!("shell: too many argv items (max 16)"));
        }
        for arg in &argv {
            if arg.len() > 4096 {
                return Err(anyhow::anyhow!("shell: argument too long (max 4096)"));
            }
        }

        // Execute.
        let output = Command::new(&bin).args(&argv).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // Trim output if excessively large.
        let max_out = 32768;
        let mut out = stdout;
        if out.len() > max_out {
            out.truncate(max_out);
            out.push_str("\n...(truncated)...");
        }
        let mut err = stderr;
        if err.len() > max_out {
            err.truncate(max_out);
            err.push_str("\n...(truncated)...");
        }

        Ok(json!({
            "stdout": out,
            "stderr": err,
            "exit_code": output.status.code(),
        }))
    }
}
