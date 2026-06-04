use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use crate::{
    dependency,
    error::RustScopeError,
    metrics,
    model::{FileAnalysis, ProjectAnalysis},
    parser, scanner,
};

pub fn analyze_project(root: &Path) -> Result<ProjectAnalysis, RustScopeError> {
    let files = scanner::scan_rust_files(root)?;
    analyze_files_parallel(root.to_path_buf(), files)
}

#[cfg(test)]
pub fn analyze_project_serial(root: &Path) -> Result<ProjectAnalysis, RustScopeError> {
    let files = scanner::scan_rust_files(root)?;
    let mut analysis = ProjectAnalysis::new(root.to_path_buf(), files.len());

    for file in files {
        analysis.add_file_analysis(analyze_file(file)?);
    }

    sort_analysis(&mut analysis);
    Ok(analysis)
}

fn analyze_files_parallel(
    project_path: PathBuf,
    files: Vec<PathBuf>,
) -> Result<ProjectAnalysis, RustScopeError> {
    let file_count = files.len();
    let worker_count = worker_count(file_count);
    let (job_sender, job_receiver) = mpsc::channel::<PathBuf>();
    let (result_sender, result_receiver) = mpsc::channel::<Result<FileAnalysis, RustScopeError>>();
    let job_receiver = Arc::new(Mutex::new(job_receiver));
    let mut handles = Vec::new();

    for _ in 0..worker_count {
        let job_receiver = Arc::clone(&job_receiver);
        let result_sender = result_sender.clone();
        handles.push(thread::spawn(move || {
            loop {
                let next_job = match job_receiver.lock() {
                    Ok(receiver) => receiver.recv(),
                    Err(_) => {
                        let _ = result_sender.send(Err(RustScopeError::Analysis(
                            "worker job queue lock was poisoned".to_string(),
                        )));
                        break;
                    }
                };

                match next_job {
                    Ok(path) => {
                        if result_sender.send(analyze_file(path)).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }));
    }

    drop(result_sender);

    for file in files {
        job_sender
            .send(file)
            .map_err(|_| RustScopeError::Analysis("failed to send analysis job".to_string()))?;
    }
    drop(job_sender);

    let mut file_results = Vec::with_capacity(file_count);
    for result in result_receiver {
        file_results.push(result?);
    }

    for handle in handles {
        handle
            .join()
            .map_err(|_| RustScopeError::Analysis("analysis worker panicked".to_string()))?;
    }

    if file_results.len() != file_count {
        return Err(RustScopeError::Analysis(format!(
            "expected {file_count} file results, received {}",
            file_results.len()
        )));
    }

    file_results.sort_by(|left, right| left.path.cmp(&right.path));
    let mut analysis = ProjectAnalysis::new(project_path, file_count);
    analysis.parallel_enabled = worker_count > 1;
    analysis.worker_threads = worker_count;

    for file in file_results {
        analysis.add_file_analysis(file);
    }

    sort_analysis(&mut analysis);
    Ok(analysis)
}

fn analyze_file(file: PathBuf) -> Result<FileAnalysis, RustScopeError> {
    let content = fs::read_to_string(&file)?;
    let line_metrics = metrics::calculate_line_metrics(&content);
    let items = parser::parse_code_items(&content, &file);
    let function_complexities = metrics::calculate_function_complexities(&content, &file);
    let dependencies = dependency::parse_use_dependencies(&content, &file)?;

    Ok(FileAnalysis {
        path: file,
        line_metrics,
        items,
        function_complexities,
        dependencies,
    })
}

fn worker_count(file_count: usize) -> usize {
    if file_count == 0 {
        return 1;
    }

    thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .min(file_count)
        .max(1)
}

fn sort_analysis(analysis: &mut ProjectAnalysis) {
    analysis
        .files
        .sort_by(|left, right| left.path.cmp(&right.path));
    analysis.items.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then_with(|| left.line.cmp(&right.line))
            .then_with(|| left.name.cmp(&right.name))
    });
    analysis.function_complexities.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then_with(|| left.line.cmp(&right.line))
            .then_with(|| left.name.cmp(&right.name))
    });
    analysis.dependencies.sort_by(|left, right| {
        left.from
            .cmp(&right.from)
            .then_with(|| left.to.cmp(&right.to))
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_analysis_matches_serial_key_metrics() -> Result<(), RustScopeError> {
        let root = Path::new("examples/demo_project");
        let parallel = analyze_project(root)?;
        let serial = analyze_project_serial(root)?;

        assert_eq!(parallel.files_analyzed, serial.files_analyzed);
        assert_eq!(parallel.line_metrics, serial.line_metrics);
        assert_eq!(parallel.items.len(), serial.items.len());
        assert_eq!(
            parallel.function_complexities.len(),
            serial.function_complexities.len()
        );
        assert_eq!(parallel.dependencies, serial.dependencies);
        Ok(())
    }
}
