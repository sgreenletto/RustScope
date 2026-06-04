use std::path::PathBuf;

use crate::error::RustScopeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Analyze(AnalyzeOptions),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeOptions {
    pub project_path: PathBuf,
    pub format: ReportFormat,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Terminal,
    Markdown,
}

pub fn parse_args<I, S>(args: I) -> Result<Command, RustScopeError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    match args.as_slice() {
        [] => Err(RustScopeError::Argument(usage_message())),
        [command] if command == "analyze" => Err(RustScopeError::Argument(usage_message())),
        [command, path, rest @ ..] if command == "analyze" => {
            parse_analyze_options(path, rest).map(Command::Analyze)
        }
        [path, rest @ ..] if path != "analyze" => {
            parse_analyze_options(path, rest).map(Command::Analyze)
        }
        _ => Err(RustScopeError::Argument(usage_message())),
    }
}

pub fn usage_message() -> String {
    "Usage:\n  rustscope analyze <project-path> [--format terminal|markdown] [--output <file>]\n  rustscope <project-path> [--format terminal|markdown] [--output <file>]".to_string()
}

fn parse_analyze_options(path: &str, args: &[String]) -> Result<AnalyzeOptions, RustScopeError> {
    let mut format = ReportFormat::Terminal;
    let mut output = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    RustScopeError::Argument("missing value for --format".to_string())
                })?;
                format = ReportFormat::parse(value)?;
                index += 2;
            }
            "--output" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    RustScopeError::Argument("missing value for --output".to_string())
                })?;
                output = Some(PathBuf::from(value));
                index += 2;
            }
            unknown => {
                return Err(RustScopeError::Argument(format!(
                    "unsupported argument: {unknown}"
                )));
            }
        }
    }

    Ok(AnalyzeOptions {
        project_path: PathBuf::from(path),
        format,
        output,
    })
}

impl ReportFormat {
    fn parse(value: &str) -> Result<Self, RustScopeError> {
        match value {
            "terminal" => Ok(Self::Terminal),
            "markdown" => Ok(Self::Markdown),
            unsupported => Err(RustScopeError::Argument(format!(
                "unsupported report format: {unsupported}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_analyze_subcommand() {
        let command = parse_args(["analyze", "examples/demo_project"]).unwrap_or_else(|error| {
            panic!("expected analyze command to parse, got {error}");
        });

        assert_eq!(
            command,
            Command::Analyze(AnalyzeOptions {
                project_path: PathBuf::from("examples/demo_project"),
                format: ReportFormat::Terminal,
                output: None,
            })
        );
    }

    #[test]
    fn parses_markdown_format() {
        let command = parse_args(["analyze", "examples/demo_project", "--format", "markdown"])
            .unwrap_or_else(|error| panic!("expected markdown format to parse, got {error}"));

        assert_eq!(
            command,
            Command::Analyze(AnalyzeOptions {
                project_path: PathBuf::from("examples/demo_project"),
                format: ReportFormat::Markdown,
                output: None,
            })
        );
    }

    #[test]
    fn parses_output_path() {
        let command = parse_args([
            "analyze",
            "examples/demo_project",
            "--format",
            "markdown",
            "--output",
            "report.md",
        ])
        .unwrap_or_else(|error| panic!("expected output path to parse, got {error}"));

        assert_eq!(
            command,
            Command::Analyze(AnalyzeOptions {
                project_path: PathBuf::from("examples/demo_project"),
                format: ReportFormat::Markdown,
                output: Some(PathBuf::from("report.md")),
            })
        );
    }

    #[test]
    fn rejects_unsupported_format() {
        let error = parse_args(["analyze", "examples/demo_project", "--format", "html"])
            .expect_err("html format should be rejected");

        assert_eq!(error.to_string(), "unsupported report format: html");
    }

    #[test]
    fn rejects_missing_path() {
        assert!(matches!(
            parse_args(Vec::<String>::new()),
            Err(RustScopeError::Argument(_))
        ));
    }
}
