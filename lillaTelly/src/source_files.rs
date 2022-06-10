use std::path::Path;

use super::task_tv_show::{TaskError, VALID_EXTENSIONS};

pub fn gather_source_files(source_dir: String) -> Result<Vec<String>, TaskError> {
    let source_path = Path::new(&source_dir);

    if !source_path.exists() || !source_path.is_dir() {
        return Err(TaskError::SourcePath(source_dir.clone()));
    }

    let mut source_files = vec![];

    let dir_entries = source_path.read_dir()?;

    for dir_entry in dir_entries {
        match dir_entry {
            Ok(de) => {
                if !de.path().is_file() {
                    continue;
                }
                if let Some(extension) = de.path().extension() {
                    if VALID_EXTENSIONS.contains(&extension.to_str().unwrap_or_default()) {
                        source_files.push(de.file_name().to_str().unwrap().to_string());
                    }
                }
            }
            Err(e) => {
                return Err(TaskError::SourcePathReading(
                    source_dir.clone(),
                    e.to_string(),
                ))
            }
        };
    }
    source_files.sort();
    Ok(source_files)
}
