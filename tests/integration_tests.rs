use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use veto::run_analysis;

fn create_test_file(path: &Path, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[test]
fn test_no_errors_found() {
    let test_file = Path::new("tests/clean_example.txt");
    let output_json = Path::new("tests/clean_report.json");

    create_test_file(test_file, "fn main() {\n    println!(\"Hello World\");\n}").expect("Failed to create test file");

    let result = run_analysis(test_file, output_json, &[]);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert_eq!(report.total_critical_bugs, 0);
    assert!(report.errors.is_empty());
    assert!(output_json.exists(), "errors.json must be generated");

    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file(output_json);
}

#[test]
fn test_critical_bug_detected() {
    let test_file = Path::new("tests/broken_example.txt");
    let output_json = Path::new("tests/broken_report.json");

    let broken_code = "// Some comments\n\
                       let x = 10; // TODO_CRITICAL: Fix before release\n\
                       println!(\"{}\", x);";

    create_test_file(test_file, broken_code).expect("Failed to create test file");

    let result = run_analysis(test_file, output_json, &[]);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert_eq!(report.total_critical_bugs, 1);

    let detected = &report.errors[0];
    assert_eq!(detected.id, "V004");
    assert_eq!(detected.line, 2);
    assert!(detected.context.contains("TODO_CRITICAL"));
    assert!(output_json.exists(), "errors.json must be generated");

    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file(output_json);
}

#[test]
fn test_git_conflict_detected() {
    let test_file = Path::new("tests/conflict_example.txt");
    let output_json = Path::new("tests/conflict_report.json");

    let conflicted_code = "fn main() {\n\
                           <<<<<<< HEAD\n\
                           println!(\"ours\");\n\
                           =======\n\
                           println!(\"theirs\");\n\
                           >>>>>>> feature-branch\n\
                           }";

    create_test_file(test_file, conflicted_code).expect("Failed to create test file");

    let result = run_analysis(test_file, output_json, &[]);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert_eq!(report.total_critical_bugs, 2);

    let ids: Vec<&str> = report.errors.iter().map(|error| error.id.as_str()).collect();
    assert!(ids.contains(&"V001"));
    assert!(ids.contains(&"V002"));

    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file(output_json);
}

#[test]
fn test_binary_file_is_skipped() {
    let test_file = Path::new("tests/binary_example.bin");
    let output_json = Path::new("tests/binary_report.json");

    let binary_content: Vec<u8> = vec![0x00, 0xFF, 0x10, 0x00, 0xAB, 0xCD];
    File::create(test_file)
        .and_then(|mut file| file.write_all(&binary_content))
        .expect("Failed to create binary test file");

    let result = run_analysis(test_file, output_json, &[]);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert_eq!(report.total_critical_bugs, 0);

    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file(output_json);
}

#[test]
fn test_directory_scan_collects_all_files() {
    let scan_dir = Path::new("tests/dir_scan");
    let output_json = Path::new("tests/dir_scan_report.json");

    let _ = fs::remove_dir_all(scan_dir);
    fs::create_dir_all(scan_dir).expect("Failed to create scan directory");

    create_test_file(&scan_dir.join("first.txt"), "let a = 1; // TODO_CRITICAL: first\n")
        .expect("Failed to create first file");

    create_test_file(
        &scan_dir.join("second.txt"),
        "fn main() {}\n<<<<<<< HEAD\nfn conflict() {}\n",
    )
    .expect("Failed to create second file");

    let result = run_analysis(scan_dir, output_json, &[]);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert_eq!(report.total_critical_bugs, 2);

    let ids: Vec<&str> = report.errors.iter().map(|error| error.id.as_str()).collect();
    assert!(ids.contains(&"V001"));
    assert!(ids.contains(&"V004"));

    let _ = fs::remove_dir_all(scan_dir);
    let _ = fs::remove_file(output_json);
}

#[test]
fn test_exclude_filter_skips_matching_entries() {
    let scan_dir = Path::new("tests/exclude_scan");
    let keep_file = Path::new("tests/exclude_scan/keep.txt");
    let skip_dir = Path::new("tests/exclude_scan/skipme");
    let skip_file = Path::new("tests/exclude_scan/skipme/skip.txt");
    let output_json = Path::new("tests/exclude_report.json");

    let _ = fs::remove_dir_all(scan_dir);
    fs::create_dir_all(skip_dir).expect("Failed to create scan directory");

    create_test_file(keep_file, "// TODO_CRITICAL: keep me\n").expect("Failed to create keep file");
    create_test_file(skip_file, "// TODO_CRITICAL: skip me\n").expect("Failed to create skip file");

    let excludes = vec!["skipme".to_string()];
    let result = run_analysis(scan_dir, output_json, &excludes);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert_eq!(report.total_critical_bugs, 1);
    assert_eq!(report.errors[0].id, "V004");
    assert!(report.errors[0].file.contains("keep.txt"));

    let _ = fs::remove_dir_all(scan_dir);
    let _ = fs::remove_file(output_json);
}

#[test]
fn test_output_file_inside_target_is_ignored() {
    let scan_dir = Path::new("tests/output_scan");
    let source_file = Path::new("tests/output_scan/source.txt");
    let output_json = Path::new("tests/output_scan/report.json");

    let _ = fs::remove_dir_all(scan_dir);
    fs::create_dir_all(scan_dir).expect("Failed to create scan directory");

    create_test_file(source_file, "fn clean() {}\n").expect("Failed to create source file");
    create_test_file(output_json, "TODO_CRITICAL stale marker\n").expect("Failed to create output file");

    let result = run_analysis(scan_dir, output_json, &[]);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert_eq!(report.total_critical_bugs, 0);

    let _ = fs::remove_dir_all(scan_dir);
}
