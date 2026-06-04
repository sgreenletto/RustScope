use std::path::Path;

use crate::{
    model::{FunctionComplexity, LineMetrics},
    parser::{extract_ident_after_keyword, find_keyword, strip_comments},
};

pub fn calculate_line_metrics(content: &str) -> LineMetrics {
    let mut metrics = LineMetrics::default();

    for line in content.lines() {
        metrics.total_lines += 1;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            metrics.blank_lines += 1;
        } else if trimmed.starts_with("//") {
            metrics.comment_lines += 1;
        } else {
            metrics.code_lines += 1;
        }
    }

    metrics
}

pub fn calculate_function_complexities(content: &str, file: &Path) -> Vec<FunctionComplexity> {
    let mut complexities = Vec::new();
    let mut in_block_comment = false;
    let mut current: Option<FunctionState> = None;

    for (index, line) in content.lines().enumerate() {
        let line_number = index + 1;
        let searchable = strip_comments(line, &mut in_block_comment);

        if let Some(function) = current.as_mut() {
            function.complexity += count_complexity_markers(&searchable);
            if !function.started && searchable.contains('{') {
                function.started = true;
            }
            function.brace_depth += brace_delta(&searchable);

            if !function.started && searchable.contains(';') {
                complexities.push(function.to_complexity(file));
                current = None;
                continue;
            }

            if function.started && function.brace_depth <= 0 {
                complexities.push(function.to_complexity(file));
                current = None;
            }
            continue;
        }

        if find_keyword(&searchable, "fn").is_some()
            && let Some(name) = extract_ident_after_keyword(&searchable, "fn")
        {
            let function = FunctionState {
                name,
                line: line_number,
                complexity: 1 + count_complexity_markers(&searchable),
                brace_depth: brace_delta(&searchable),
                started: searchable.contains('{'),
            };

            if !function.started && searchable.contains(';') {
                complexities.push(function.to_complexity(file));
                continue;
            }

            if function.started && function.brace_depth <= 0 {
                complexities.push(function.to_complexity(file));
            } else {
                current = Some(function);
            }
        }
    }

    if let Some(function) = current {
        complexities.push(function.to_complexity(file));
    }

    complexities
}

#[derive(Debug)]
struct FunctionState {
    name: String,
    line: usize,
    complexity: usize,
    brace_depth: isize,
    started: bool,
}

impl FunctionState {
    fn to_complexity(&self, file: &Path) -> FunctionComplexity {
        FunctionComplexity {
            name: self.name.clone(),
            file: file.to_path_buf(),
            line: self.line,
            complexity: self.complexity,
        }
    }
}

fn count_complexity_markers(line: &str) -> usize {
    count_keyword(line, "if")
        + count_keyword(line, "match")
        + count_keyword(line, "for")
        + count_keyword(line, "while")
        + count_keyword(line, "loop")
        + line.matches('?').count()
}

fn count_keyword(line: &str, keyword: &str) -> usize {
    let mut count = 0;
    let mut offset = 0;

    while let Some(position) = find_keyword(&line[offset..], keyword) {
        count += 1;
        offset += position + keyword.len();
    }

    count
}

fn brace_delta(line: &str) -> isize {
    let open = line.matches('{').count() as isize;
    let close = line.matches('}').count() as isize;
    open - close
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn counts_blank_comment_and_code_lines() {
        let content = "\n// comment\nlet value = 1;\n   \n  // another comment\nfn main() {}\n";
        let metrics = calculate_line_metrics(content);

        assert_eq!(metrics.total_lines, 6);
        assert_eq!(metrics.blank_lines, 2);
        assert_eq!(metrics.comment_lines, 2);
        assert_eq!(metrics.code_lines, 2);
    }

    #[test]
    fn calculates_simple_function_complexity() {
        let content = r#"
fn analyze() -> Result<(), Error> {
    if ready {
        for item in items {
            handle(item)?;
        }
    } else if fallback {
        while retry {
            match state {
                State::Done => break,
                State::Loop => loop { break; },
            }
        }
    }
    Ok(())
}
"#;
        let file = PathBuf::from("src/analyzer.rs");
        let complexities = calculate_function_complexities(content, &file);

        assert_eq!(complexities.len(), 1);
        assert_eq!(complexities[0].name, "analyze");
        assert_eq!(complexities[0].complexity, 8);
    }
}
