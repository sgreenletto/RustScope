use std::collections::BTreeMap;

use crate::{
    error::RustScopeError,
    model::{DependencyEdge, FunctionComplexity, ItemKind, ProjectAnalysis},
};

pub trait ReportGenerator {
    fn generate(&self, analysis: &ProjectAnalysis) -> Result<String, RustScopeError>;
}

pub struct TerminalReportGenerator;

pub struct MarkdownReportGenerator;

pub struct HtmlReportGenerator;

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
            output.push_str("None detected.\n\n");
        } else {
            output.push_str("| From | To |\n");
            output.push_str("|---|---|\n");
            for dependency in &analysis.dependencies {
                output.push_str(&format!("| {} | {} |\n", dependency.from, dependency.to));
            }
            output.push('\n');
        }

        output.push_str("## Mermaid Dependency Graph\n\n");
        if analysis.dependencies.is_empty() {
            output.push_str("None detected.\n");
        } else {
            output.push_str("```mermaid\n");
            output.push_str(&generate_mermaid_graph(&analysis.dependencies));
            output.push_str("```\n");
        }

        Ok(output)
    }
}

impl ReportGenerator for HtmlReportGenerator {
    fn generate(&self, analysis: &ProjectAnalysis) -> Result<String, RustScopeError> {
        validate_analysis(analysis)?;

        let top_functions = top_complex_functions(&analysis.function_complexities, 5);
        let mut output = String::new();

        output.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n");
        output.push_str("<meta charset=\"utf-8\">\n");
        output
            .push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
        output.push_str("<title>RustScope Analysis Report</title>\n");
        output.push_str("<style>\n");
        output.push_str("body{font-family:Arial,sans-serif;margin:0;background:#f6f8fb;color:#202733;}main{max-width:1100px;margin:0 auto;padding:32px;}h1{margin:0 0 8px;}section{background:#fff;border:1px solid #d9e0ea;border-radius:8px;margin:18px 0;padding:18px;}table{width:100%;border-collapse:collapse;}th,td{border-bottom:1px solid #e5e9f0;padding:8px;text-align:left;}th{background:#eef3f8;}td.num,th.num{text-align:right;}pre{background:#111827;color:#e5e7eb;padding:16px;border-radius:8px;overflow:auto;}.summary{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px}.pill{background:#eef3f8;border-radius:6px;padding:10px;}\n");
        output.push_str("</style>\n</head>\n<body>\n<main>\n");
        output.push_str("<h1>RustScope Analysis Report</h1>\n");

        output.push_str("<section><h2>Project Summary</h2><div class=\"summary\">");
        output.push_str(&format!(
            "<div class=\"pill\"><strong>Project Path</strong><br>{}</div>",
            html_escape(&analysis.project_path.display().to_string())
        ));
        output.push_str(&format!(
            "<div class=\"pill\"><strong>Files Analyzed</strong><br>{}</div>",
            analysis.files_analyzed
        ));
        output.push_str(&format!(
            "<div class=\"pill\"><strong>Parallel Analysis</strong><br>{}</div>",
            if analysis.parallel_enabled {
                "enabled"
            } else {
                "disabled"
            }
        ));
        output.push_str(&format!(
            "<div class=\"pill\"><strong>Worker Threads</strong><br>{}</div>",
            analysis.worker_threads
        ));
        output.push_str("</div></section>\n");

        output.push_str("<section><h2>Line Metrics</h2><table><thead><tr><th>Metric</th><th class=\"num\">Count</th></tr></thead><tbody>");
        html_metric_row(
            &mut output,
            "Total Lines",
            analysis.line_metrics.total_lines,
        );
        html_metric_row(&mut output, "Code Lines", analysis.line_metrics.code_lines);
        html_metric_row(
            &mut output,
            "Comment Lines",
            analysis.line_metrics.comment_lines,
        );
        html_metric_row(
            &mut output,
            "Blank Lines",
            analysis.line_metrics.blank_lines,
        );
        output.push_str("</tbody></table></section>\n");

        output.push_str("<section><h2>Code Items</h2><table><thead><tr><th>Item</th><th class=\"num\">Count</th></tr></thead><tbody>");
        html_metric_row(
            &mut output,
            "Functions",
            count_items(analysis, ItemKind::Function),
        );
        html_metric_row(
            &mut output,
            "Structs",
            count_items(analysis, ItemKind::Struct),
        );
        html_metric_row(&mut output, "Enums", count_items(analysis, ItemKind::Enum));
        html_metric_row(
            &mut output,
            "Traits",
            count_items(analysis, ItemKind::Trait),
        );
        html_metric_row(
            &mut output,
            "Modules",
            count_items(analysis, ItemKind::Module),
        );
        html_metric_row(
            &mut output,
            "Impl Blocks",
            count_items(analysis, ItemKind::Impl),
        );
        output.push_str("</tbody></table></section>\n");

        output.push_str("<section><h2>Top Complex Functions</h2>");
        if top_functions.is_empty() {
            output.push_str("<p>None detected.</p>");
        } else {
            output.push_str("<table><thead><tr><th class=\"num\">Rank</th><th>Function</th><th>File</th><th class=\"num\">Line</th><th class=\"num\">Complexity</th></tr></thead><tbody>");
            for (index, function) in top_functions.iter().enumerate() {
                output.push_str(&format!(
                    "<tr><td class=\"num\">{}</td><td>{}</td><td>{}</td><td class=\"num\">{}</td><td class=\"num\">{}</td></tr>",
                    index + 1,
                    html_escape(&function.name),
                    html_escape(&function.file.display().to_string()),
                    function.line,
                    function.complexity
                ));
            }
            output.push_str("</tbody></table>");
        }
        output.push_str("</section>\n");

        output.push_str("<section><h2>Module Dependencies</h2>");
        if analysis.dependencies.is_empty() {
            output.push_str("<p>None detected.</p>");
        } else {
            output.push_str("<table><thead><tr><th>From</th><th>To</th></tr></thead><tbody>");
            for dependency in &analysis.dependencies {
                output.push_str(&format!(
                    "<tr><td>{}</td><td>{}</td></tr>",
                    html_escape(&dependency.from),
                    html_escape(&dependency.to)
                ));
            }
            output.push_str("</tbody></table>");
        }
        output.push_str("</section>\n");

        output.push_str("<section><h2>Mermaid Dependency Graph</h2>");
        if analysis.dependencies.is_empty() {
            output.push_str("<p>None detected.</p>");
        } else {
            output.push_str("<pre class=\"mermaid\"><code>");
            output.push_str(&html_escape(&generate_mermaid_graph(
                &analysis.dependencies,
            )));
            output.push_str("</code></pre>");
        }
        output.push_str("</section>\n");

        output.push_str("</main>\n</body>\n</html>\n");
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
    output.push_str(&format!(
        "Parallel analysis: {}\n",
        if analysis.parallel_enabled {
            "enabled"
        } else {
            "disabled"
        }
    ));
    output.push_str(&format!("Worker threads: {}\n\n", analysis.worker_threads));
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

pub(crate) fn count_code_items(analysis: &ProjectAnalysis, kind: ItemKind) -> usize {
    count_items(analysis, kind)
}

pub(crate) fn top_complex_functions(
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

pub fn generate_mermaid_graph(dependencies: &[DependencyEdge]) -> String {
    if dependencies.is_empty() {
        return "None detected.\n".to_string();
    }

    let mut node_ids = BTreeMap::new();
    let mut next_id = 0usize;
    let mut edges = dependencies.to_vec();
    edges.sort_by(|left, right| {
        left.from
            .cmp(&right.from)
            .then_with(|| left.to.cmp(&right.to))
    });

    for dependency in &edges {
        for label in [&dependency.from, &dependency.to] {
            if !node_ids.contains_key(label) {
                node_ids.insert(label.clone(), format!("node{next_id}"));
                next_id += 1;
            }
        }
    }

    let mut output = String::from("graph TD\n");
    for (label, id) in &node_ids {
        output.push_str(&format!("    {id}[\"{}\"]\n", mermaid_label_escape(label)));
    }
    for dependency in &edges {
        if let (Some(from), Some(to)) =
            (node_ids.get(&dependency.from), node_ids.get(&dependency.to))
        {
            output.push_str(&format!("    {from} --> {to}\n"));
        }
    }
    output
}

fn mermaid_label_escape(value: &str) -> String {
    value
        .replace('\\', "/")
        .replace('"', "'")
        .replace(['\n', '\r'], " ")
}

fn html_metric_row(output: &mut String, label: &str, value: usize) {
    output.push_str(&format!(
        "<tr><td>{}</td><td class=\"num\">{}</td></tr>",
        html_escape(label),
        value
    ));
}

fn html_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
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
        assert!(report.contains("```mermaid"));
        Ok(())
    }

    #[test]
    fn html_report_generator_outputs_document_sections() -> Result<(), RustScopeError> {
        let analysis = sample_analysis();
        let generator = HtmlReportGenerator;
        let report = generator.generate(&analysis)?;

        assert!(report.contains("<html"));
        assert!(report.contains("Project Summary"));
        assert!(report.contains("Line Metrics"));
        Ok(())
    }

    #[test]
    fn mermaid_graph_uses_safe_node_ids() {
        let dependencies = vec![DependencyEdge {
            from: "src/main.rs".to_string(),
            to: "crate::parser::parse_file".to_string(),
        }];

        let graph = generate_mermaid_graph(&dependencies);

        assert!(graph.contains("graph TD"));
        assert!(graph.contains("node0"));
        assert!(graph.contains("node1"));
        assert!(graph.contains("src/main.rs"));
        assert!(graph.contains("crate::parser::parse_file"));
    }

    #[test]
    fn mermaid_graph_escapes_special_labels() {
        let dependencies = vec![DependencyEdge {
            from: "src\\main.rs".to_string(),
            to: "crate::module \"quoted\"".to_string(),
        }];

        let graph = generate_mermaid_graph(&dependencies);

        assert!(graph.contains("src/main.rs"));
        assert!(graph.contains("crate::module 'quoted'"));
        assert!(!graph.contains("\"quoted\""));
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
