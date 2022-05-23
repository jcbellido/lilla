use thiserror::Error;

use crate::{source_target_configuration::SourceTargetConfiguration, target_tv_show::TargetTVShow};

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
    #[error("Error in IO")]
    IoError(#[from] std::io::Error),
    #[error("Something else happened!")]
    UnknownError,
}

pub const VALID_EXTENSIONS: [&str; 2] = ["mp4", "webm"];

#[derive(Debug)]
pub enum TaskAction {
    Copy(String, String),
}

pub struct TaskTvShow {
    pub configuration: SourceTargetConfiguration,
    pub target_tv_show: TargetTVShow,
    pub source_files: Vec<String>,
}

impl TaskTvShow {
    pub fn new(configuration: SourceTargetConfiguration) -> Result<Self, TaskError> {
        let source_files = super::source_files::gather_source_files(configuration.source.clone())?;
        let target_tv_show =
            super::target_tv_show::construct_tv_show(configuration.target.clone())?;
        Ok(Self {
            configuration,
            target_tv_show,
            source_files,
        })
    }
}

impl TaskTvShow {
    pub fn count_target_files(&self) -> usize {
        self.target_tv_show.total_len()
    }

    pub fn count_source_files(&self) -> usize {
        self.source_files.len()
    }

    pub fn dry_run(&self) -> Result<Vec<()>, TaskError> {
        Ok(vec![(), ()])
    }
}

#[cfg(test)]
mod tests {
    use super::TaskTvShow;
    use crate::source_target_configuration::SourceTargetConfiguration;

    #[test]
    fn dry_run() {
        let source = r#"[
            {
                "source" : "./test_data/source_a",
                "target" : "./test_data/target_a"
            }
        ]"#;
        let mut all_configurations =
            serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();
        let task = TaskTvShow::new(all_configurations.pop().unwrap()).unwrap();
        // dry run should return the actions that this task will perform.
        let dry_actions = task.dry_run();
        assert!(dry_actions.is_ok());
        assert_eq!(dry_actions.unwrap().len(), 2);
    }

    #[test]
    fn count_source_files() {
        let source = r#"[
        {
            "source" : "./test_data/source_a",
            "target" : "./test_data/target_a"
        }
    ]"#;
        let mut all_configurations =
            serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();
        let task = TaskTvShow::new(all_configurations.pop().unwrap()).unwrap();
        assert_eq!(task.count_source_files(), 4);
    }

    #[test]
    fn count_target_files() {
        let source = r#"[
            {
                "source" : "./test_data/source_a",
                "target" : "./test_data/target_a"
            }
        ]"#;
        let mut all_configurations =
            serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();
        let task = TaskTvShow::new(all_configurations.pop().unwrap()).unwrap();
        assert_eq!(task.count_target_files(), 2);
    }
}
