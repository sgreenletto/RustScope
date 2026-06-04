use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
}

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
pub struct FileAnalysis {
    pub path: PathBuf,
    pub line_metrics: LineMetrics,
    pub items: Vec<CodeItem>,
    pub function_complexities: Vec<FunctionComplexity>,
    pub dependencies: Vec<DependencyEdge>,
}

#[derive(Debug, Clone)]
pub struct ProjectAnalysis {
    pub project_path: PathBuf,
    pub files_analyzed: usize,
    pub files: Vec<FileAnalysis>,
    pub line_metrics: LineMetrics,
    pub items: Vec<CodeItem>,
    pub function_complexities: Vec<FunctionComplexity>,
    pub dependencies: Vec<DependencyEdge>,
}

impl ProjectAnalysis {
    pub fn new(project_path: PathBuf, files_analyzed: usize) -> Self {
        Self {
            project_path,
            files_analyzed,
            files: Vec::new(),
            line_metrics: LineMetrics::default(),
            items: Vec::new(),
            function_complexities: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    pub fn add_file_analysis(&mut self, file: FileAnalysis) {
        self.line_metrics.add(&file.line_metrics);
        self.items.extend(file.items.iter().cloned());
        self.function_complexities
            .extend(file.function_complexities.iter().cloned());
        self.dependencies.extend(file.dependencies.iter().cloned());
        self.files.push(file);
    }
}
