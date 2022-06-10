use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{
    source_target_configuration::SourceTargetConfiguration,
    target_tv_show::{SeasonEntry, TargetTVShow},
};

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

pub const VALID_EXTENSIONS: [&str; 3] = ["mp4", "webm", "m4v"];

#[derive(Debug)]
pub enum TaskAction {
    Copy(PathBuf, SeasonEntry),
}

impl Display for TaskAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskAction::Copy(source, target) => {
                write!(
                    f,
                    "src: {}\ntrg: {}",
                    source.to_str().unwrap_or("<wrong source?>"),
                    target.full_path.to_str().unwrap_or("<wrong target?>")
                )
            }
        }
    }
}

impl TaskAction {
    pub fn simplified_display(&self) -> String {
        match self {
            TaskAction::Copy(source, target) => {
                let source_file = source
                    .as_path()
                    .file_name()
                    .map_or("<wrong source?>", |s| s.to_str().unwrap());
                let target_file = target
                    .full_path
                    .as_path()
                    .file_name()
                    .map_or("<wrong target?>", |t| t.to_str().unwrap());
                format!("{}\n{}", source_file, target_file)
            }
        }
    }
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

    pub fn gather_actions_to_run(&self) -> Result<Vec<TaskAction>, TaskError> {
        if self.source_files.is_empty() {
            return Ok(vec![]);
        }

        let mut output = vec![];
        let (mut season, mut episode) = self.target_tv_show.first_available_entry(
            self.configuration.default_season,
            self.configuration.default_episode,
        );
        for source_file in self
            .source_files
            .iter()
            .filter(|source_file| !self.target_tv_show.contains(source_file))
        {
            let se = self
                .target_tv_show
                .construct_season_entry(source_file, season, episode);
            season = se.season_number;
            episode = se.episode_number + 1;
            output.push(TaskAction::Copy(
                Path::new(&self.configuration.source).join(source_file),
                se,
            ));
        }
        Ok(output)
    }

    /// Gathers actions and logs some reports mainly
    pub fn dry_run(&self) -> Result<(), TaskError> {
        log::info!(
            "Dry run: for: {} - {}",
            self.configuration.source,
            self.configuration.target
        );
        let tasks = self.gather_actions_to_run()?;

        if tasks.is_empty() {
            log::info!("Dry run: No new actions needed",);
            return Ok(());
        }
        log::info!(
            "Dry run: tasks found for source: {}",
            self.configuration.source
        );
        for task in tasks {
            log::info!("Dry run:\n{}", task.simplified_display());
        }
        Ok(())
    }

    pub fn run(&self) -> Result<(), TaskError> {
        log::info!(
            "Executing: {} - {}",
            self.configuration.source,
            self.configuration.target
        );
        let tasks = self.gather_actions_to_run()?;

        if tasks.is_empty() {
            log::info!("No new actions needed");
            return Ok(());
        }
        for task in tasks {
            log::info!("Executing {task}");
            #[allow(irrefutable_let_patterns)]
            if let TaskAction::Copy(source, target) = task {
                if !target.target_dir.exists() {
                    std::fs::create_dir(target.target_dir.clone())?;
                }
                let output = std::process::Command::new("/bin/cp")
                    .arg(source.to_str().unwrap())
                    .arg(target.full_path.to_str().unwrap())
                    .output()?;
                log::info!("{:#?}", output);
            } else {
                // This is essentially scaffolding as of now
                log::warn!("Action {:#?} not supported", task);
            }
        }
        Ok(())
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
                "target" : "./test_data/target_a",
                "default_season" : 30,
                "default_episode": 1
            }
        ]"#;
        let mut all_configurations =
            serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();
        let task = TaskTvShow::new(all_configurations.pop().unwrap()).unwrap();
        let dry_actions = task.gather_actions_to_run();
        assert!(dry_actions.is_ok());
        assert_eq!(dry_actions.unwrap().len(), 2);
    }

    #[test]
    fn count_source_files() {
        let source = r#"[
        {
            "source" : "./test_data/source_a",
            "target" : "./test_data/target_a",
            "default_season" : 30,
            "default_episode": 1
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
                "target" : "./test_data/target_a",
                "default_season" : 30,
                "default_episode": 1
            }
        ]"#;
        let mut all_configurations =
            serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();
        let task = TaskTvShow::new(all_configurations.pop().unwrap()).unwrap();
        assert_eq!(task.count_target_files(), 2);
    }
}
