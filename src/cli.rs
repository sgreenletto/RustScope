use std::path::PathBuf;

use crate::error::RustScopeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Analyze(PathBuf),
}

pub fn parse_args<I, S>(args: I) -> Result<Command, RustScopeError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    match args.as_slice() {
        [command, path] if command == "analyze" => Ok(Command::Analyze(PathBuf::from(path))),
        [path] if path != "analyze" => Ok(Command::Analyze(PathBuf::from(path))),
        [] | [_] => Err(RustScopeError::Usage(usage_message())),
        _ => Err(RustScopeError::Usage(usage_message())),
    }
}

pub fn usage_message() -> String {
    "Usage:\n  rustscope analyze <project-path>\n  rustscope <project-path>".to_string()
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
            Command::Analyze(PathBuf::from("examples/demo_project"))
        );
    }

    #[test]
    fn rejects_missing_path() {
        assert!(matches!(
            parse_args(Vec::<String>::new()),
            Err(RustScopeError::Usage(_))
        ));
    }
}
