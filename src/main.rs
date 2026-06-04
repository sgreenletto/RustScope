mod analyzer;
mod cli;
mod dependency;
mod error;
mod metrics;
mod model;
mod output;
mod parser;
mod report;
mod scanner;
mod tui;

use std::{env, process};

use cli::{Command, ReportFormat};
use error::RustScopeError;
use report::{
    HtmlReportGenerator, MarkdownReportGenerator, ReportGenerator, TerminalReportGenerator,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(match error {
            RustScopeError::Argument(_) => 2,
            _ => 1,
        });
    }
}

fn run() -> Result<(), RustScopeError> {
    match cli::parse_args(env::args().skip(1))? {
        Command::Analyze(options) => {
            let analysis = analyzer::analyze_project(&options.project_path)?;

            let generator: Box<dyn ReportGenerator> = match options.format {
                ReportFormat::Terminal => Box::new(TerminalReportGenerator),
                ReportFormat::Markdown => Box::new(MarkdownReportGenerator),
                ReportFormat::Html => Box::new(HtmlReportGenerator),
            };
            let report = generator.generate(&analysis)?;

            if let Some(output_path) = options.output {
                output::write_report(&output_path, &report)?;
                println!("Report written to {}", output_path.display());
            } else {
                println!("{report}");
            }

            Ok(())
        }
        Command::Tui(project_path) => {
            let analysis = analyzer::analyze_project(&project_path)?;
            tui::run_dashboard(&analysis)
        }
    }
}
