use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemKind {
    Function,
    Struct,
    Enum,
    Trait,
    Module,
    Impl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeItem {
    pub name: String,
    pub kind: ItemKind,
    pub file: PathBuf,
    pub line: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LineMetrics {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
}

impl LineMetrics {
    pub fn add(&mut self, other: &Self) {
        self.total_lines += other.total_lines;
        self.code_lines += other.code_lines;
        self.comment_lines += other.comment_lines;
        self.blank_lines += other.blank_lines;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionComplexity {
    pub name: String,
    pub file: PathBuf,
    pub line: usize,
    pub complexity: usize,
}

#[derive(Debug, Clone)]
pub struct AnalysisReport {
    pub project_path: PathBuf,
    pub files_analyzed: usize,
    pub line_metrics: LineMetrics,
    pub items: Vec<CodeItem>,
    pub function_complexities: Vec<FunctionComplexity>,
}

impl AnalysisReport {
    pub fn new(project_path: PathBuf, files_analyzed: usize) -> Self {
        Self {
            project_path,
            files_analyzed,
            line_metrics: LineMetrics::default(),
            items: Vec::new(),
            function_complexities: Vec::new(),
        }
    }
}
