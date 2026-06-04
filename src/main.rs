mod cli;
mod dependency;
mod error;
mod metrics;
mod model;
mod parser;
mod report;
mod scanner;

use std::{env, fs, process};

use cli::{Command, ReportFormat};
use error::RustScopeError;
use model::{FileAnalysis, ProjectAnalysis};
use report::{MarkdownReportGenerator, ReportGenerator, TerminalReportGenerator};

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
            let files = scanner::scan_rust_files(&options.project_path)?;
            let mut analysis = ProjectAnalysis::new(options.project_path, files.len());

            for file in files {
                let content = fs::read_to_string(&file)?;
                let line_metrics = metrics::calculate_line_metrics(&content);
                let items = parser::parse_code_items(&content, &file);
                let function_complexities =
                    metrics::calculate_function_complexities(&content, &file);
                let dependencies = dependency::parse_use_dependencies(&content, &file)?;

                analysis.add_file_analysis(FileAnalysis {
                    path: file,
                    line_metrics,
                    items,
                    function_complexities,
                    dependencies,
                });
            }

            let generator: Box<dyn ReportGenerator> = match options.format {
                ReportFormat::Terminal => Box::new(TerminalReportGenerator),
                ReportFormat::Markdown => Box::new(MarkdownReportGenerator),
            };
            let report = generator.generate(&analysis)?;

            if let Some(output_path) = options.output {
                fs::write(&output_path, report)
                    .map_err(|error| RustScopeError::output_write(&output_path, error))?;
                println!("Report written to {}", output_path.display());
            } else {
                println!("{report}");
            }

            Ok(())
        }
    }
}
