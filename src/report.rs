use crate::{
    error::RustScopeError,
    model::{FunctionComplexity, ItemKind, ProjectAnalysis},
};

pub trait ReportGenerator {
    fn generate(&self, analysis: &ProjectAnalysis) -> Result<String, RustScopeError>;
}

pub struct TerminalReportGenerator;

pub struct MarkdownReportGenerator;

impl ReportGenerator for TerminalReportGenerator {
    fn generate(&self, analysis: &ProjectAnalysis) -> Result<String, RustScopeError> {
        validate_analysis(analysis)?;

        Ok(generate_terminal_report(analysis))
    }
}

impl ReportGenerator for MarkdownReportGenerator {
    fn generate(&self, analysis: &ProjectAnalysis) -> Result<String, RustScopeError> {
        validate_analysis(analysis)?;

        let mut output = String::new();
        let top_functions = top_complex_functions(&analysis.function_complexities, 5);

        output.push_str("# RustScope Analysis Report\n\n");
        output.push_str("## Project Summary\n\n");
        output.push_str(&format!(
            "- Project Path: {}\n",
            analysis.project_path.display()
        ));
        output.push_str(&format!(
            "- Files Analyzed: {}\n\n",
            analysis.files_analyzed
        ));

        output.push_str("## Line Metrics\n\n");
        output.push_str("| Metric | Count |\n");
        output.push_str("|---|---:|\n");
        output.push_str(&format!(
            "| Total Lines | {} |\n",
            analysis.line_metrics.total_lines
        ));
        output.push_str(&format!(
            "| Code Lines | {} |\n",
            analysis.line_metrics.code_lines
        ));
        output.push_str(&format!(
            "| Comment Lines | {} |\n",
            analysis.line_metrics.comment_lines
        ));
        output.push_str(&format!(
            "| Blank Lines | {} |\n\n",
            analysis.line_metrics.blank_lines
        ));

        output.push_str("## Code Items\n\n");
        output.push_str("| Item | Count |\n");
        output.push_str("|---|---:|\n");
        output.push_str(&format!(
            "| Functions | {} |\n",
            count_items(analysis, ItemKind::Function)
        ));
        output.push_str(&format!(
            "| Structs | {} |\n",
            count_items(analysis, ItemKind::Struct)
        ));
        output.push_str(&format!(
            "| Enums | {} |\n",
            count_items(analysis, ItemKind::Enum)
        ));
        output.push_str(&format!(
            "| Traits | {} |\n",
            count_items(analysis, ItemKind::Trait)
        ));
        output.push_str(&format!(
            "| Modules | {} |\n",
            count_items(analysis, ItemKind::Module)
        ));
        output.push_str(&format!(
            "| Impl Blocks | {} |\n\n",
            count_items(analysis, ItemKind::Impl)
        ));

        output.push_str("## Top Complex Functions\n\n");
        if top_functions.is_empty() {
            output.push_str("None detected.\n\n");
        } else {
            output.push_str("| Rank | Function | File | Line | Complexity |\n");
            output.push_str("|---:|---|---|---:|---:|\n");
            for (index, function) in top_functions.iter().enumerate() {
                output.push_str(&format!(
                    "| {} | {} | {} | {} | {} |\n",
                    index + 1,
                    function.name,
                    function.file.display(),
                    function.line,
                    function.complexity
                ));
            }
            output.push('\n');
        }

        output.push_str("## Module Dependencies\n\n");
        if analysis.dependencies.is_empty() {
            output.push_str("None detected.\n");
        } else {
            output.push_str("| From | To |\n");
            output.push_str("|---|---|\n");
            for dependency in &analysis.dependencies {
                output.push_str(&format!("| {} | {} |\n", dependency.from, dependency.to));
            }
        }

        Ok(output)
    }
}

fn validate_analysis(analysis: &ProjectAnalysis) -> Result<(), RustScopeError> {
    if analysis.project_path.as_os_str().is_empty() {
        return Err(RustScopeError::ReportGeneration(
            "project path is empty".to_string(),
        ));
    }

    if analysis
        .files
        .iter()
        .any(|file| file.path.as_os_str().is_empty())
    {
        return Err(RustScopeError::ReportGeneration(
            "analysis contains an empty file path".to_string(),
        ));
    }

    Ok(())
}

pub fn generate_terminal_report(analysis: &ProjectAnalysis) -> String {
    let mut output = String::new();
    let top_functions = top_complex_functions(&analysis.function_complexities, 5);

    output.push_str("RustScope Analysis Report\n");
    output.push_str("=========================\n\n");
    output.push_str(&format!("Project: {}\n\n", analysis.project_path.display()));
    output.push_str(&format!("Files analyzed: {}\n\n", analysis.files_analyzed));
    output.push_str("Line Metrics:\n");
    output.push_str(&format!(
        "- Total lines: {}\n",
        analysis.line_metrics.total_lines
    ));
    output.push_str(&format!(
        "- Code lines: {}\n",
        analysis.line_metrics.code_lines
    ));
    output.push_str(&format!(
        "- Comment lines: {}\n",
        analysis.line_metrics.comment_lines
    ));
    output.push_str(&format!(
        "- Blank lines: {}\n\n",
        analysis.line_metrics.blank_lines
    ));
    output.push_str("Code Items:\n");
    output.push_str(&format!(
        "- Functions: {}\n",
        count_items(analysis, ItemKind::Function)
    ));
    output.push_str(&format!(
        "- Structs: {}\n",
        count_items(analysis, ItemKind::Struct)
    ));
    output.push_str(&format!(
        "- Enums: {}\n",
        count_items(analysis, ItemKind::Enum)
    ));
    output.push_str(&format!(
        "- Traits: {}\n",
        count_items(analysis, ItemKind::Trait)
    ));
    output.push_str(&format!(
        "- Modules: {}\n",
        count_items(analysis, ItemKind::Module)
    ));
    output.push_str(&format!(
        "- Impl blocks: {}\n\n",
        count_items(analysis, ItemKind::Impl)
    ));
    output.push_str("Top Complex Functions:\n");

    if top_functions.is_empty() {
        output.push_str("No functions found.\n");
    } else {
        for (index, function) in top_functions.iter().enumerate() {
            output.push_str(&format!(
                "{}. {:<20} {}:{}    complexity: {}\n",
                index + 1,
                function.name,
                function.file.display(),
                function.line,
                function.complexity
            ));
        }
    }
    output.push('\n');
    output.push_str("Module Dependencies:\n");
    if analysis.dependencies.is_empty() {
        output.push_str("None detected.\n");
    } else {
        for dependency in &analysis.dependencies {
            output.push_str(&format!("{} -> {}\n", dependency.from, dependency.to));
        }
    }

    output
}

fn count_items(analysis: &ProjectAnalysis, kind: ItemKind) -> usize {
    analysis
        .items
        .iter()
        .filter(|item| item.kind == kind)
        .count()
}

fn top_complex_functions(
    functions: &[FunctionComplexity],
    limit: usize,
) -> Vec<FunctionComplexity> {
    let mut functions = functions.to_vec();
    functions.sort_by(|left, right| {
        right
            .complexity
            .cmp(&left.complexity)
            .then_with(|| left.file.cmp(&right.file))
            .then_with(|| left.line.cmp(&right.line))
            .then_with(|| left.name.cmp(&right.name))
    });
    functions.truncate(limit);
    functions
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::model::{CodeItem, DependencyEdge, LineMetrics};

    use super::*;

    #[test]
    fn terminal_report_generator_outputs_report() -> Result<(), RustScopeError> {
        let analysis = sample_analysis();
        let generator = TerminalReportGenerator;
        let report = generator.generate(&analysis)?;

        assert!(report.contains("RustScope Analysis Report"));
        assert!(report.contains("Files analyzed: 1"));
        assert!(report.contains("Module Dependencies:"));
        Ok(())
    }

    #[test]
    fn markdown_report_generator_outputs_title_and_tables() -> Result<(), RustScopeError> {
        let analysis = sample_analysis();
        let generator = MarkdownReportGenerator;
        let report = generator.generate(&analysis)?;

        assert!(report.contains("# RustScope Analysis Report"));
        assert!(report.contains("| Metric | Count |"));
        assert!(report.contains("| Rank | Function | File | Line | Complexity |"));
        assert!(report.contains("| From | To |"));
        Ok(())
    }

    fn sample_analysis() -> ProjectAnalysis {
        let mut analysis = ProjectAnalysis::new(PathBuf::from("demo"), 1);
        analysis.line_metrics = LineMetrics {
            total_lines: 10,
            code_lines: 8,
            comment_lines: 1,
            blank_lines: 1,
        };
        analysis.items.push(CodeItem {
            name: "main".to_string(),
            kind: ItemKind::Function,
            file: PathBuf::from("src/main.rs"),
            line: 1,
        });
        analysis.function_complexities.push(FunctionComplexity {
            name: "main".to_string(),
            file: PathBuf::from("src/main.rs"),
            line: 1,
            complexity: 1,
        });
        analysis.dependencies.push(DependencyEdge {
            from: "src/main.rs".to_string(),
            to: "crate::parser".to_string(),
        });
        analysis
    }
}
