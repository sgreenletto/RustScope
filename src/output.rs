use std::{fs, path::Path};

use crate::error::RustScopeError;

pub fn write_report(path: &Path, report: &str) -> Result<(), RustScopeError> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|error| RustScopeError::output_write(path, error))?;
    }

    fs::write(path, report).map_err(|error| RustScopeError::output_write(path, error))
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    #[test]
    fn writes_report_and_creates_parent_directories() -> Result<(), RustScopeError> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("rustscope_output_{nanos}"));
        let output = root.join("reports/report.html");

        write_report(&output, "<html></html>")?;

        assert_eq!(fs::read_to_string(&output)?, "<html></html>");
        fs::remove_dir_all(root)?;
        Ok(())
    }
}
