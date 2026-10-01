use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use crate::errors::CriticalError;

const PROBE_BUFFER_SIZE: usize = 8192;

struct CriticalPattern {
    id: &'static str,
    needle: &'static str,
    message: &'static str,
}

const CRITICAL_PATTERNS: &[CriticalPattern] = &[
    CriticalPattern {
        id: "V001",
        needle: "<<<<<<<",
        message: "Unmerged git conflict marker detected.",
    },
    CriticalPattern {
        id: "V002",
        needle: ">>>>>>>",
        message: "Unmerged git conflict marker detected.",
    },
    CriticalPattern {
        id: "V003",
        needle: "|||||||",
        message: "Unmerged git conflict marker detected.",
    },
    CriticalPattern {
        id: "V004",
        needle: "TODO_CRITICAL",
        message: "Critical TODO marker present in source.",
    },
    CriticalPattern {
        id: "V005",
        needle: "FIXME_CRITICAL",
        message: "Critical FIXME marker present in source.",
    },
    CriticalPattern {
        id: "V006",
        needle: "XXX_CRITICAL",
        message: "Critical XXX marker present in source.",
    },
];

pub struct VetoParser;

impl VetoParser {
    pub fn check_file(path: &Path) -> Result<Vec<CriticalError>, std::io::Error> {
        if is_binary(path) {
            return Ok(Vec::new());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut found = Vec::new();

        for (index, line_result) in reader.lines().enumerate() {
            let line = match line_result {
                Ok(value) => value,
                Err(_) => continue,
            };

            for pattern in CRITICAL_PATTERNS {
                if let Some(column) = line.find(pattern.needle) {
                    found.push(CriticalError {
                        id: pattern.id.to_string(),
                        file: path.to_string_lossy().to_string(),
                        line: index + 1,
                        column: column + 1,
                        message: pattern.message.to_string(),
                        context: line.trim().to_string(),
                    });
                    break;
                }
            }
        }

        Ok(found)
    }
}

fn is_binary(path: &Path) -> bool {
    let mut buffer = [0u8; PROBE_BUFFER_SIZE];
    match File::open(path).and_then(|mut file| file.read(&mut buffer)) {
        Ok(bytes_read) => buffer[..bytes_read].contains(&0),
        Err(_) => true,
    }
}
