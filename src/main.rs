use std::process;

use clap::Parser;

use veto::args::{default_excludes, CliArgs};
use veto::interactive;
use veto::run_analysis;

fn main() {
    let args = CliArgs::parse();

    match args.path.as_deref() {
        Some(path) => run_cli(path, &args.output, args.quiet),
        None => run_interactive_mode(),
    }
}

fn run_cli(path: &std::path::Path, output: &std::path::Path, quiet: bool) {
    let excludes = default_excludes();

    match run_analysis(path, output, &excludes) {
        Ok(report) => {
            if !quiet {
                if report.total_critical_bugs == 0 {
                    println!("Veto: no critical bugs found.");
                } else {
                    println!(
                        "Veto: {} critical bug(s) found. Report saved to {}",
                        report.total_critical_bugs,
                        output.display()
                    );
                }
            }

            if report.total_critical_bugs == 0 {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Veto: execution error: {}", err);
            process::exit(2);
        }
    }
}

fn run_interactive_mode() {
    if let Err(err) = interactive::run_interactive() {
        eprintln!("Veto: interactive error: {}", err);
        process::exit(2);
    }
}
