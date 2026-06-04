use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use crate::error::RustScopeError;

pub fn scan_rust_files(root: &Path) -> Result<Vec<PathBuf>, RustScopeError> {
    if !root.exists() {
        return Err(RustScopeError::InvalidPath(format!(
            "Project path does not exist: {}",
            root.display()
        )));
    }

    if !root.is_dir() {
        return Err(RustScopeError::InvalidPath(format!(
            "Project path is not a directory: {}",
            root.display()
        )));
    }

    let mut files = Vec::new();
    scan_dir(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn scan_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), RustScopeError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if should_ignore_dir(&path) {
                continue;
            }
            scan_dir(&path, files)?;
        } else if is_rust_file(&path) {
            files.push(path);
        }
    }

    Ok(())
}

fn should_ignore_dir(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|name| name == OsStr::new("target") || name == OsStr::new(".git"))
}

fn is_rust_file(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "rs")
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    #[test]
    fn scans_rs_files_and_ignores_target_and_git() -> Result<(), RustScopeError> {
        let root = unique_test_dir("scanner")?;
        fs::create_dir_all(root.join("src"))?;
        fs::create_dir_all(root.join("target/debug"))?;
        fs::create_dir_all(root.join(".git/hooks"))?;

        File::create(root.join("src/main.rs"))?;
        File::create(root.join("src/lib.txt"))?;
        File::create(root.join("target/debug/generated.rs"))?;
        File::create(root.join(".git/hooks/hook.rs"))?;

        let files = scan_rust_files(&root)?;

        assert_eq!(files.len(), 1);
        assert_eq!(files[0], root.join("src/main.rs"));

        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[test]
    fn rejects_invalid_project_path() {
        let path = std::env::temp_dir().join("rustscope_missing_project_path");

        assert!(matches!(
            scan_rust_files(&path),
            Err(RustScopeError::InvalidPath(_))
        ));
    }

    fn unique_test_dir(name: &str) -> Result<PathBuf, RustScopeError> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("rustscope_{name}_{nanos}"));
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}
