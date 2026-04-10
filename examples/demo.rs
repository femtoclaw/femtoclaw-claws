use femtoclaw_claws::core::ClawRegistry;
use femtoclaw_claws::claws::shell::ShellClaw;
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let mut registry = ClawRegistry::new();
    registry.register(ShellClaw);

    let result = registry.execute("shell", json!({
        "bin": "echo",
        "argv": ["FemtoClaw operational"]
    }))?;

    println!("{}", result);
    Ok(())
}
