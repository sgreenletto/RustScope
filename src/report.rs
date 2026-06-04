use crate::model::{AnalysisReport, FunctionComplexity, ItemKind};

pub fn generate_terminal_report(analysis: &AnalysisReport) -> String {
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

    output
}

fn count_items(analysis: &AnalysisReport, kind: ItemKind) -> usize {
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
