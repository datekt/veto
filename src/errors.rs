use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorReport {
    pub total_critical_bugs: usize,
    pub scanned_at: String,
    pub errors: Vec<CriticalError>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CriticalError {
    pub id: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub context: String,
}

pub fn write_error_tree(
    errors: &[CriticalError],
    errors_root: &Path,
    strip_prefix: &Path,
    prepend: Option<&Path>,
) -> io::Result<()> {
    if errors_root.exists() {
        fs::remove_dir_all(errors_root)?;
    }
    fs::create_dir_all(errors_root)?;

    let mut by_file: BTreeMap<PathBuf, Vec<&CriticalError>> = BTreeMap::new();
    for err in errors {
        by_file.entry(PathBuf::from(&err.file)).or_default().push(err);
    }

    for (file_path, errs) in &by_file {
        let rel = strip_base(file_path, strip_prefix);
        let with_prefix = match prepend {
            Some(prefix) => prefix.join(&rel),
            None => rel,
        };

        let file_name = with_prefix
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let json_name = format!("{}.json", file_name);
        let parent_dir = with_prefix.parent().unwrap_or(Path::new(""));
        let out_path = errors_root.join(parent_dir).join(&json_name);

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json_data = serde_json::to_string_pretty(errs)?;
        fs::write(&out_path, json_data)?;
    }

    Ok(())
}

fn strip_base(file: &Path, base: &Path) -> PathBuf {
    if let Ok(rel) = file.strip_prefix(base) {
        return rel.to_path_buf();
    }

    if let (Ok(canon_file), Ok(canon_base)) = (fs::canonicalize(file), fs::canonicalize(base)) {
        if let Ok(rel) = canon_file.strip_prefix(&canon_base) {
            return rel.to_path_buf();
        }
    }

    if base == Path::new(".") {
        return file.to_path_buf();
    }

    file.to_path_buf()
}
