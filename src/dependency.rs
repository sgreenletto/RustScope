use std::path::Path;

use crate::{error::RustScopeError, model::DependencyEdge, parser::strip_comments};

pub fn parse_use_dependencies(
    content: &str,
    file: &Path,
) -> Result<Vec<DependencyEdge>, RustScopeError> {
    let mut dependencies = Vec::new();
    let mut in_block_comment = false;
    let from = module_name_from_path(file);

    for line in content.lines() {
        let searchable = strip_comments(line, &mut in_block_comment);
        let trimmed = searchable.trim_start();

        if let Some(path) = extract_use_path(trimmed)? {
            dependencies.push(DependencyEdge {
                from: from.clone(),
                to: path,
            });
        }
    }

    Ok(dependencies)
}

fn extract_use_path(line: &str) -> Result<Option<String>, RustScopeError> {
    let line = if let Some(stripped) = line.strip_prefix("pub(crate) ") {
        stripped
    } else if let Some(stripped) = line.strip_prefix("pub(super) ") {
        stripped
    } else if let Some(stripped) = line.strip_prefix("pub ") {
        stripped
    } else {
        line
    };

    let Some(after_use) = line.strip_prefix("use ") else {
        return Ok(None);
    };

    let before_semicolon = match after_use.split(';').next() {
        Some(value) => value.trim(),
        None => "",
    };

    if before_semicolon.is_empty() {
        return Err(RustScopeError::Parse(
            "empty use dependency path".to_string(),
        ));
    }

    Ok(Some(before_semicolon.to_string()))
}

fn module_name_from_path(file: &Path) -> String {
    file.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn recognizes_crate_super_and_std_use_dependencies() -> Result<(), RustScopeError> {
        let file = PathBuf::from("src/parser.rs");
        let content = r#"
use crate::scanner;
use crate::parser::parse_file;
use super::model::ProjectAnalysis;
use std::collections::HashMap;
"#;

        let dependencies = parse_use_dependencies(content, &file)?;
        let targets: Vec<&str> = dependencies
            .iter()
            .map(|dependency| dependency.to.as_str())
            .collect();

        assert_eq!(
            targets,
            vec![
                "crate::scanner",
                "crate::parser::parse_file",
                "super::model::ProjectAnalysis",
                "std::collections::HashMap",
            ]
        );
        assert!(dependencies.iter().all(|edge| edge.from == "src/parser.rs"));

        Ok(())
    }

    #[test]
    fn ignores_use_statements_in_comments() -> Result<(), RustScopeError> {
        let file = PathBuf::from("src/lib.rs");
        let content = r#"
// use crate::ignored;
/* use std::ignored::Thing; */
use crate::real;
"#;

        let dependencies = parse_use_dependencies(content, &file)?;

        assert_eq!(dependencies.len(), 1);
        assert_eq!(dependencies[0].to, "crate::real");
        Ok(())
    }
}
