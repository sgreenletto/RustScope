use std::io::{self, Write};

use crate::{
    error::RustScopeError,
    model::{ItemKind, ProjectAnalysis},
    report::{count_code_items, top_complex_functions},
};

pub fn run_dashboard(analysis: &ProjectAnalysis) -> Result<(), RustScopeError> {
    let mut stdout = io::stdout();
    write_dashboard(&mut stdout, analysis)?;
    wait_for_quit()
}

fn write_dashboard<W: Write>(
    writer: &mut W,
    analysis: &ProjectAnalysis,
) -> Result<(), RustScopeError> {
    writeln!(writer, "RustScope TUI Dashboard")?;
    writeln!(writer, "=======================")?;
    writeln!(writer, "Project: {}", analysis.project_path.display())?;
    writeln!(
        writer,
        "Parallel analysis: {} | Worker threads: {}",
        if analysis.parallel_enabled {
            "enabled"
        } else {
            "disabled"
        },
        analysis.worker_threads
    )?;
    writeln!(writer)?;
    writeln!(
        writer,
        "Files: {} | Total Lines: {} | Code Lines: {}",
        analysis.files_analyzed,
        analysis.line_metrics.total_lines,
        analysis.line_metrics.code_lines
    )?;
    writeln!(
        writer,
        "Comments: {} | Blank Lines: {}",
        analysis.line_metrics.comment_lines, analysis.line_metrics.blank_lines
    )?;
    writeln!(writer)?;
    writeln!(writer, "Code Items")?;
    writeln!(
        writer,
        "Functions: {} | Structs: {} | Enums: {} | Traits: {}",
        count_code_items(analysis, ItemKind::Function),
        count_code_items(analysis, ItemKind::Struct),
        count_code_items(analysis, ItemKind::Enum),
        count_code_items(analysis, ItemKind::Trait)
    )?;
    writeln!(
        writer,
        "Modules: {} | Impl Blocks: {}",
        count_code_items(analysis, ItemKind::Module),
        count_code_items(analysis, ItemKind::Impl)
    )?;
    writeln!(writer)?;
    writeln!(writer, "Top Complex Functions")?;
    let top_functions = top_complex_functions(&analysis.function_complexities, 5);
    if top_functions.is_empty() {
        writeln!(writer, "None detected.")?;
    } else {
        for (index, function) in top_functions.iter().enumerate() {
            writeln!(
                writer,
                "{}. {} | {}:{} | complexity: {}",
                index + 1,
                function.name,
                function.file.display(),
                function.line,
                function.complexity
            )?;
        }
    }
    writeln!(writer)?;
    writeln!(writer, "Module Dependencies")?;
    if analysis.dependencies.is_empty() {
        writeln!(writer, "None detected.")?;
    } else {
        for dependency in analysis.dependencies.iter().take(12) {
            writeln!(writer, "{} -> {}", dependency.from, dependency.to)?;
        }
        if analysis.dependencies.len() > 12 {
            writeln!(writer, "... {} more", analysis.dependencies.len() - 12)?;
        }
    }
    writeln!(writer)?;
    writeln!(writer, "Press q to quit.")?;
    writer.flush()?;
    Ok(())
}

fn wait_for_quit() -> Result<(), RustScopeError> {
    let mut input = String::new();
    loop {
        input.clear();
        let read = io::stdin().read_line(&mut input)?;
        if read == 0 || input.trim().eq_ignore_ascii_case("q") {
            return Ok(());
        }
        println!("Press q to quit.");
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::model::{FunctionComplexity, LineMetrics};

    use super::*;

    #[test]
    fn dashboard_contains_project_summary() -> Result<(), RustScopeError> {
        let mut analysis = ProjectAnalysis::new(PathBuf::from("demo"), 1);
        analysis.line_metrics = LineMetrics {
            total_lines: 12,
            code_lines: 10,
            comment_lines: 1,
            blank_lines: 1,
        };
        analysis.function_complexities.push(FunctionComplexity {
            name: "main".to_string(),
            file: PathBuf::from("src/main.rs"),
            line: 1,
            complexity: 1,
        });
        let mut output = Vec::new();

        write_dashboard(&mut output, &analysis)?;
        let rendered = String::from_utf8(output)
            .map_err(|error| RustScopeError::ReportGeneration(error.to_string()))?;

        assert!(rendered.contains("RustScope TUI Dashboard"));
        assert!(rendered.contains("Project: demo"));
        assert!(rendered.contains("Top Complex Functions"));
        Ok(())
    }
}
