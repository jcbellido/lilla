use std::path::Path;

use thiserror::Error;

use crate::source_target_configuration::SourceTargetConfiguration;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Source path `{0}` is not a directory or can't be found")]
    SourcePath(String),
    #[error("Error reading source path `{0}` : `{1}`")]
    SourcePathReading(String, String),
    #[error("Target path `{0}` is not a directory or can't be found")]
    TargetPath(String),
    #[error("Error reading target path `{0}` : `{1}`")]
    TargetPathReading(String, String),
    #[error("Something else happened!")]
    UnknownError,
}

const VALID_EXTENSIONS: [&str; 2] = ["mp4", "webm"];

pub struct TaskTvShow {
    pub configuration: SourceTargetConfiguration,
}

impl TaskTvShow {
    fn count_target_files(&self) -> Result<u32, TaskError> {
        let target_path = Path::new(&self.configuration.target);

        if !target_path.exists() || !target_path.is_dir() {
            return Err(TaskError::TargetPath(self.configuration.target.clone()));
        }

        let target_dir_entries = match target_path.read_dir() {
            Ok(rd) => rd,
            Err(e) => {
                return Err(TaskError::TargetPathReading(
                    self.configuration.target.clone(),
                    e.to_string(),
                ))
            }
        };

        unimplemented!()
    }

    fn count_source_files(&self) -> Result<u32, TaskError> {
        let source_path = Path::new(&self.configuration.source);

        if !source_path.exists() || !source_path.is_dir() {
            return Err(TaskError::SourcePath(self.configuration.source.clone()));
        }

        let dir_entries = match source_path.read_dir() {
            Ok(de) => de,
            Err(err) => {
                return Err(TaskError::SourcePathReading(
                    self.configuration.source.clone(),
                    err.to_string(),
                ))
            }
        };

        let mut count_valid_files: u32 = 0;

        for dir_entry in dir_entries {
            match dir_entry {
                Ok(de) => {
                    if !de.path().is_file() {
                        continue;
                    }
                    if let Some(extension) = de.path().extension() {
                        if VALID_EXTENSIONS.contains(&extension.to_str().unwrap_or_default()) {
                            count_valid_files += 1;
                        }
                    }
                }
                Err(e) => {
                    return Err(TaskError::SourcePathReading(
                        self.configuration.source.clone(),
                        e.to_string(),
                    ))
                }
            };
        }

        Ok(count_valid_files)
    }
}

#[cfg(test)]
mod tests {
    use super::TaskTvShow;
    use crate::source_target_configuration::SourceTargetConfiguration;

    #[test]
    fn count_source_files() {
        let source = r#"[
        {
            "source" : "./test_data/source_a",
            "target" : "does not really matter much"
        }
    ]"#;
        let mut all_configurations =
            serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();
        let task = TaskTvShow {
            configuration: all_configurations.pop().unwrap(),
        };
        let count = task.count_source_files();
        assert!(count.is_ok());
        assert_eq!(count.unwrap(), 4);
    }

    #[test]
    fn count_target_files() {
        let source = r#"[
            {
                "source" : "should be inconsequential for this test",
                "target" : "./test_data/target_a"
            }
        ]"#;
        let mut all_configurations =
            serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();
        let task = TaskTvShow {
            configuration: all_configurations.pop().unwrap(),
        };
        let count = task.count_target_files();
        assert!(count.is_ok());
        assert_eq!(count.unwrap(), 2);
    }
}
