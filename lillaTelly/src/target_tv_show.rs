use std::{fs::DirEntry, path::Path};

use regex::{Captures, Regex};

use super::task_tv_show::{TaskError, VALID_EXTENSIONS};

#[derive(Debug, Default)]
pub struct Season {
    number: u32,
    files: Vec<SeasonEntry>,
}

#[derive(Debug)]
pub struct SeasonEntry {
    full_file_name: String,
    season_number: u32,
    episode_number: u32,
    sanitized_file_name: String,
}

#[derive(Debug, Default)]
pub struct TargetTVShow {
    seasons: Vec<Season>,
}

impl TargetTVShow {
    pub fn total_len(&self) -> usize {
        self.seasons.iter().map(|s| s.files.len()).sum()
    }
}

fn captures_to_season_entry(captures: Captures) -> SeasonEntry {
    let season_number = captures.get(1).unwrap().as_str().parse().unwrap();
    let episode_number = captures.get(2).unwrap().as_str().parse().unwrap();

    let sanitized_file_name = format!(
        "{}.{}",
        captures.get(3).unwrap().as_str(),
        captures.get(4).unwrap().as_str()
    );

    SeasonEntry {
        full_file_name: captures.get(0).unwrap().as_str().into(),
        season_number,
        episode_number,
        sanitized_file_name,
    }
}

fn try_parse_season(season_dir: DirEntry) -> Result<Option<Season>, TaskError> {
    assert!(season_dir.path().is_dir());

    let regex_season = Regex::new(r"^[Ss]eason (?P<season_number>\d+)$")
        .expect("Season regex has been impossible to build (?)");

    if let Some(captures) =
        regex_season.captures(season_dir.file_name().to_str().unwrap_or_default())
    {
        let season_number = captures.get(1).unwrap().as_str().parse().unwrap();
        let mut season = Season {
            number: season_number,
            files: vec![],
        };

        let season_file_entries = season_dir.path().read_dir()?;

        let extensions = VALID_EXTENSIONS.join("|");
        let regex_episode = Regex::new(format!("^S(\\d+)E(\\d)+-(.*)\\.({extensions})$").as_str())
            .expect("Season regex has been impossible to build (?)");

        for season_episode in season_file_entries {
            let episode = season_episode?;
            if !episode.path().is_file() {
                continue;
            }
            if let Some(captures) =
                regex_episode.captures(episode.file_name().to_str().unwrap_or_default())
            {
                season.files.push(captures_to_season_entry(captures));
            }
        }
        Ok(Some(season))
    } else {
        Ok(None)
    }
}

pub fn construct_tv_show(conf_target_dir: String) -> Result<TargetTVShow, TaskError> {
    let target_path = Path::new(&conf_target_dir);

    if !target_path.exists() || !target_path.is_dir() {
        return Err(TaskError::TargetPath(conf_target_dir.clone()));
    }

    let target_dir_entries = target_path.read_dir()?;

    let mut target_tv_show = TargetTVShow::default();

    for season_entry in target_dir_entries {
        match season_entry {
            Ok(season) => {
                if !season.path().is_dir() {
                    continue;
                }
                try_parse_season(season)?.map(|s| target_tv_show.seasons.push(s));
            }
            Err(err) => {
                return Err(TaskError::TargetPathReading(
                    conf_target_dir.clone(),
                    err.to_string(),
                ));
            }
        };
    }

    Ok(target_tv_show)
}

#[cfg(test)]
mod tests {
    use super::construct_tv_show;

    #[test]
    fn map_target_season() {
        let v = construct_tv_show("./test_data/target_a".into()).unwrap();
        assert_eq!(v.seasons.len(), 1);
        assert_eq!(v.seasons[0].files.len(), 2);
    }
}
