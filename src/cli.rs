use std::path::PathBuf;

use crate::error::RustScopeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Analyze(AnalyzeOptions),
    Tui(PathBuf),
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
    Html,
}

pub fn parse_args<I, S>(args: I) -> Result<Command, RustScopeError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    match args.as_slice() {
        [] => Err(RustScopeError::Argument(usage_message())),
        [command] if command == "analyze" || command == "tui" => {
            Err(RustScopeError::Argument(usage_message()))
        }
        [command, path, rest @ ..] if command == "analyze" => {
            parse_analyze_options(path, rest).map(Command::Analyze)
        }
        [command, path] if command == "tui" => Ok(Command::Tui(PathBuf::from(path))),
        [command, _, ..] if command == "tui" => Err(RustScopeError::Argument(
            "unsupported argument for tui command".to_string(),
        )),
        [path, rest @ ..] if path != "analyze" => {
            parse_analyze_options(path, rest).map(Command::Analyze)
        }
        _ => Err(RustScopeError::Argument(usage_message())),
    }
}

pub fn usage_message() -> String {
    "Usage:\n  rustscope analyze <project-path> [--format terminal|markdown|html] [--output <file>]\n  rustscope <project-path> [--format terminal|markdown|html] [--output <file>]\n  rustscope tui <project-path>".to_string()
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
            "html" => Ok(Self::Html),
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
    fn parses_html_format() {
        let command = parse_args(["analyze", "examples/demo_project", "--format", "html"])
            .unwrap_or_else(|error| panic!("expected html format to parse, got {error}"));

        assert_eq!(
            command,
            Command::Analyze(AnalyzeOptions {
                project_path: PathBuf::from("examples/demo_project"),
                format: ReportFormat::Html,
                output: None,
            })
        );
    }

    #[test]
    fn parses_html_format_with_output_path() {
        let command = parse_args([
            "analyze",
            "examples/demo_project",
            "--format",
            "html",
            "--output",
            "reports/report.html",
        ])
        .unwrap_or_else(|error| panic!("expected html output options to parse, got {error}"));

        assert_eq!(
            command,
            Command::Analyze(AnalyzeOptions {
                project_path: PathBuf::from("examples/demo_project"),
                format: ReportFormat::Html,
                output: Some(PathBuf::from("reports/report.html")),
            })
        );
    }

    #[test]
    fn parses_tui_subcommand() {
        let command = parse_args(["tui", "examples/demo_project"])
            .unwrap_or_else(|error| panic!("expected tui command to parse, got {error}"));

        assert_eq!(
            command,
            Command::Tui(PathBuf::from("examples/demo_project"))
        );
    }

    #[test]
    fn rejects_unsupported_format() {
        let error = parse_args(["analyze", "examples/demo_project", "--format", "docx"])
            .expect_err("docx format should be rejected");

        assert_eq!(error.to_string(), "unsupported report format: docx");
    }

    #[test]
    fn rejects_missing_path() {
        assert!(matches!(
            parse_args(Vec::<String>::new()),
            Err(RustScopeError::Argument(_))
        ));
    }
}
