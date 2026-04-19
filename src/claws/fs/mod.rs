use crate::core::Claw;
use anyhow::Result;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub struct FsClaw;

impl Claw for FsClaw {
    fn name(&self) -> &'static str {
        "fs"
    }

    fn description(&self) -> &'static str {
        "Filesystem read operations"
    }

    fn execute(&self, args: Value) -> Result<Value> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing path"))?;
        let allowed_root = allowed_root()?;
        let resolved_path = resolve_path(path)?;

        if !resolved_path.starts_with(&allowed_root) {
            return Err(anyhow::anyhow!("path escapes allowed root"));
        }
        if !resolved_path.is_file() {
            return Err(anyhow::anyhow!("path is not a file"));
        }

        let content = fs::read_to_string(&resolved_path)?;
        Ok(json!({ "content": content }))
    }
}

fn allowed_root() -> Result<PathBuf> {
    let root = std::env::var("FEMTO_FS_ROOT")
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?);
    canonicalize_existing(&root)
}

fn resolve_path(path: &str) -> Result<PathBuf> {
    let candidate = PathBuf::from(path);
    let absolute = if candidate.is_absolute() {
        candidate
    } else {
        std::env::current_dir()?.join(candidate)
    };
    canonicalize_existing(&absolute)
}

fn canonicalize_existing(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .map_err(|e| anyhow::anyhow!("invalid path '{}': {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::FsClaw;
    use crate::core::Claw;
    use serde_json::json;
    use std::fs;

    #[test]
    fn rejects_paths_outside_allowed_root() {
        let root = std::env::temp_dir().join(format!("femtoclaw-fs-root-{}", std::process::id()));
        let outside =
            std::env::temp_dir().join(format!("femtoclaw-fs-outside-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        fs::write(&outside, "outside").unwrap();
        std::env::set_var("FEMTO_FS_ROOT", &root);

        let result = FsClaw.execute(json!({ "path": outside.to_string_lossy() }));

        std::env::remove_var("FEMTO_FS_ROOT");
        let _ = fs::remove_file(&outside);
        let _ = fs::remove_dir_all(&root);
        assert!(result.is_err());
    }
}
