mod cli;
mod error;
mod metrics;
mod model;
mod parser;
mod report;
mod scanner;

use std::{env, fs, process};

use cli::Command;
use error::RustScopeError;
use model::AnalysisReport;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(match error {
            RustScopeError::Usage(_) => 2,
            _ => 1,
        });
    }
}

fn run() -> Result<(), RustScopeError> {
    match cli::parse_args(env::args().skip(1))? {
        Command::Analyze(project_path) => {
            let files = scanner::scan_rust_files(&project_path)?;
            let mut analysis = AnalysisReport::new(project_path, files.len());

            for file in files {
                let content = fs::read_to_string(&file)?;
                let line_metrics = metrics::calculate_line_metrics(&content);
                analysis.line_metrics.add(&line_metrics);
                analysis
                    .items
                    .extend(parser::parse_code_items(&content, &file));
                analysis
                    .function_complexities
                    .extend(metrics::calculate_function_complexities(&content, &file));
            }

            println!("{}", report::generate_terminal_report(&analysis));
            Ok(())
        }
    }
}
