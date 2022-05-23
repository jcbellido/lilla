use std::{fs::DirEntry, path::Path};

use regex::Regex;

use super::task_tv_show::{TaskError, VALID_EXTENSIONS};

fn parse_season(season_dir: DirEntry, target_files: &mut Vec<String>) -> Result<(), TaskError> {
    assert!(season_dir.path().is_dir());

    let regex_season =
        Regex::new(r"^[Ss]eason (\d+)$").expect("Season regex has been impossible to build (?)");

    if regex_season.is_match(season_dir.file_name().to_str().unwrap_or_default()) {
        let season_file_entries = season_dir.path().read_dir()?;

        let extensions = VALID_EXTENSIONS.join("|");
        let regex_episode = Regex::new(format!("^S(\\d+)E(\\d)+-.*({extensions})$").as_str())
            .expect("Season regex has been impossible to build (?)");

        for season_episode in season_file_entries {
            let episode = season_episode?;
            if !episode.path().is_file() {
                continue;
            }
            if regex_episode.is_match(episode.file_name().to_str().unwrap_or_default()) {
                target_files.push(episode.file_name().to_str().unwrap().into());
            }
        }
    }
    Ok(())
}

pub fn gather_target_files(conf_target_dir: String) -> Result<Vec<String>, TaskError> {
    let target_path = Path::new(&conf_target_dir);

    if !target_path.exists() || !target_path.is_dir() {
        return Err(TaskError::TargetPath(conf_target_dir.clone()));
    }

    let target_dir_entries = target_path.read_dir()?;

    let mut target_files = vec![];

    for season_entry in target_dir_entries {
        match season_entry {
            Ok(season) => {
                if !season.path().is_dir() {
                    continue;
                }
                parse_season(season, &mut target_files)?;
            }
            Err(err) => {
                return Err(TaskError::TargetPathReading(
                    conf_target_dir.clone(),
                    err.to_string(),
                ));
            }
        };
    }

    Ok(target_files)
}
