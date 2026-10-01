use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "veto",
    author = "datekt",
    version = "0.1.0",
    about = "Veto: Lightning-fast linter that stops your build only on critical, fatal bugs.",
    long_about = "Veto is a lightning-fast linter that detects only critical, fatal bugs.\n\n\
                  Run `veto` with no arguments to open the interactive menu, or pass a PATH \
                  to scan in CLI mode."
)]
pub struct CliArgs {
    #[arg(value_name = "PATH")]
    pub path: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE", default_value = "errors.json")]
    pub output: PathBuf,

    #[arg(short, long)]
    pub quiet: bool,
}

pub fn default_excludes() -> Vec<String> {
    vec![
        ".git".into(),
        "node_modules".into(),
        "target".into(),
        ".venv".into(),
        "dist".into(),
        "build".into(),
        "errors".into(),
    ]
}
