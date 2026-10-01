pub mod args;
pub mod errors;
pub mod interactive;
pub mod parser;

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::Utc;
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use errors::{CriticalError, ErrorReport};
use parser::VetoParser;

pub fn scan_errors(
    target_path: &Path,
    skip: Option<&Path>,
    excludes: &[String],
) -> Result<Vec<CriticalError>, Box<dyn std::error::Error>> {
    let files = collect_files(target_path, skip, excludes);

    let mut all_errors: Vec<CriticalError> = files
        .par_iter()
        .flat_map(|path| VetoParser::check_file(path).unwrap_or_default())
        .collect();

    all_errors.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then(left.line.cmp(&right.line))
            .then(left.column.cmp(&right.column))
    });

    Ok(all_errors)
}

pub fn run_analysis(
    target_path: &Path,
    output_path: &Path,
    excludes: &[String],
) -> Result<ErrorReport, Box<dyn std::error::Error>> {
    let errors = scan_errors(target_path, Some(output_path), excludes)?;

    let report = ErrorReport {
        total_critical_bugs: errors.len(),
        scanned_at: Utc::now().to_rfc3339(),
        errors,
    };

    write_report(&report, output_path)?;

    Ok(report)
}

fn collect_files(target_path: &Path, skip: Option<&Path>, excludes: &[String]) -> Vec<PathBuf> {
    if target_path.is_file() {
        return vec![target_path.to_path_buf()];
    }

    WalkDir::new(target_path)
        .into_iter()
        .filter_entry(|entry| !is_excluded(entry, excludes))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| match skip {
            Some(skip_path) => !paths_equal(path, skip_path),
            None => true,
        })
        .collect()
}

fn is_excluded(entry: &DirEntry, excludes: &[String]) -> bool {
    let name = entry.file_name().to_string_lossy();
    excludes.iter().any(|pattern| pattern == name.as_ref())
}

fn paths_equal(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }

    match (fs::canonicalize(left), fs::canonicalize(right)) {
        (Ok(lhs), Ok(rhs)) => lhs == rhs,
        _ => false,
    }
}

fn write_report(report: &ErrorReport, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let json_data = serde_json::to_string_pretty(report)?;
    let mut file = File::create(output_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
