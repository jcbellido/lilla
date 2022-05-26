use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

use regex::{Captures, Regex};

use super::task_tv_show::{TaskError, VALID_EXTENSIONS};

#[derive(Debug, Default)]
pub struct Season {
    number: u32,
    entries: Vec<SeasonEntry>,
}

#[derive(Debug)]
pub struct SeasonEntry {
    pub full_file_name: String,
    pub season_number: u32,
    pub episode_number: u32,
    pub sanitized_file_name: String,
    pub full_path: PathBuf,
    pub target_dir: PathBuf,
}

#[derive(Debug, Default)]
pub struct TargetTVShow {
    seasons: Vec<Season>,
    root_dir: PathBuf,
}

impl TargetTVShow {
    pub fn total_len(&self) -> usize {
        self.seasons.iter().map(|s| s.entries.len()).sum()
    }
    pub fn contains(&self, source_file: &str) -> bool {
        for season in &self.seasons {
            if season
                .entries
                .iter()
                .any(|target_file| target_file.sanitized_file_name == source_file)
            {
                return true;
            }
        }
        false
    }

    pub fn first_available_entry(&self, default_season: u32, default_episode: u32) -> (u32, u32) {
        if self.seasons.is_empty() {
            return (default_season, default_episode);
        }
        let last_season = self.seasons.last().unwrap();

        if last_season.number < default_season {
            return (default_season, default_episode);
        }

        if last_season.number == default_season {
            return match last_season.entries.last() {
                Some(episode) => {
                    if episode.episode_number >= default_episode {
                        (default_season, episode.episode_number + 1)
                    } else {
                        (default_season, default_episode)
                    }
                }
                None => (default_season, default_episode),
            };
        }

        // Last season is "greater" than default season number
        match last_season.entries.last() {
            Some(episode) => (last_season.number, episode.episode_number + 1),
            None => (last_season.number, 1),
        }
    }

    pub fn construct_season_entry(
        &self,
        file_name: &str,
        mut suggested_season: u32,
        mut suggested_episode: u32,
    ) -> SeasonEntry {
        if suggested_episode > 99 {
            suggested_season += 1;
            suggested_episode = 1;
        }

        if let Some(season) = self.get_season(suggested_season) {
            if season
                .entries
                .iter()
                .any(|se| se.episode_number == suggested_episode)
            {
                return self.construct_season_entry(
                    file_name,
                    suggested_season,
                    suggested_episode + 1,
                );
            }
        }

        let season_dir_name = format!("Season {:02}", suggested_season);
        let full_file_name = format!(
            "S{:02}E{:02}-{}",
            suggested_season, suggested_episode, file_name
        );
        SeasonEntry {
            full_file_name: full_file_name.clone(),
            season_number: suggested_season,
            episode_number: suggested_episode,
            sanitized_file_name: file_name.into(),
            full_path: self
                .root_dir
                .join(season_dir_name.clone())
                .join(full_file_name),
            target_dir: self.root_dir.join(season_dir_name),
        }
    }

    fn get_season(&self, season: u32) -> Option<&Season> {
        self.seasons.iter().find(|s| s.number == season)
    }
}

fn captures_to_season_entry(captures: Captures, season_dir: PathBuf) -> SeasonEntry {
    let season_number = captures.get(1).unwrap().as_str().parse().unwrap();
    let episode_number = captures.get(2).unwrap().as_str().parse().unwrap();

    let sanitized_file_name = format!(
        "{}.{}",
        captures.get(3).unwrap().as_str(),
        captures.get(4).unwrap().as_str()
    );

    let full_file_name = captures.get(0).unwrap().as_str();

    SeasonEntry {
        full_file_name: full_file_name.to_string(),
        season_number,
        episode_number,
        sanitized_file_name,
        full_path: season_dir.join(full_file_name),
        target_dir: season_dir,
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
            entries: vec![],
        };

        let season_file_entries = season_dir.path().read_dir()?;

        let extensions = VALID_EXTENSIONS.join("|");
        let regex_episode =
            Regex::new(format!("^[Ss](\\d+)[Ee](\\d+)-(.*)\\.({extensions})$").as_str())
                .expect("Season regex has been impossible to build (?)");

        for season_episode in season_file_entries {
            let episode = season_episode?;
            if !episode.path().is_file() {
                continue;
            }
            if let Some(captures) =
                regex_episode.captures(episode.file_name().to_str().unwrap_or_default())
            {
                season
                    .entries
                    .push(captures_to_season_entry(captures, season_dir.path()));
            }
        }
        season
            .entries
            .sort_by(|a, b| a.episode_number.partial_cmp(&b.episode_number).unwrap());

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
                if let Some(s) = try_parse_season(season)? {
                    target_tv_show.seasons.push(s)
                }
            }
            Err(err) => {
                return Err(TaskError::TargetPathReading(
                    conf_target_dir.clone(),
                    err.to_string(),
                ));
            }
        };
    }

    target_tv_show
        .seasons
        .sort_by(|a, b| a.number.partial_cmp(&b.number).unwrap());
    target_tv_show.root_dir = target_path.to_path_buf();

    log::trace!("TargetTVShow: {:#?}", target_tv_show);

    Ok(target_tv_show)
}

#[cfg(test)]
mod tests {
    use super::construct_tv_show;

    #[test]
    fn map_target_season() {
        let v = construct_tv_show("./test_data/target_a".into()).unwrap();
        assert_eq!(v.seasons.len(), 1);
        assert_eq!(v.seasons[0].entries.len(), 2);
    }

    #[test]
    fn next_available_entry() {
        let tv_show = construct_tv_show("./test_data/target_a".into()).unwrap();
        let (season, episode) = tv_show.first_available_entry(66, 7);
        assert_eq!(season, 66);
        assert_eq!(episode, 7);
    }

    #[test]
    fn construct_season_entry_jump_season() {
        let tv_show = construct_tv_show("./test_data/target_a".into()).unwrap();
        let new_season_entry = tv_show.construct_season_entry("foo_bar_bar.mp4", 30, 100);
        assert_eq!(new_season_entry.season_number, 31);
        assert_eq!(new_season_entry.episode_number, 1);
        assert_eq!(new_season_entry.sanitized_file_name, "foo_bar_bar.mp4");
        assert_eq!(new_season_entry.full_file_name, "S31E01-foo_bar_bar.mp4");
    }

    #[test]
    fn construct_season_entry_jump_next() {
        let tv_show = construct_tv_show("./test_data/target_a".into()).unwrap();
        let new_season_entry = tv_show.construct_season_entry("foo_bar_bar.mp4", 30, 2);
        assert_eq!(new_season_entry.season_number, 30);
        assert_eq!(new_season_entry.episode_number, 3);
        assert_eq!(new_season_entry.sanitized_file_name, "foo_bar_bar.mp4");
        assert_eq!(new_season_entry.full_file_name, "S30E03-foo_bar_bar.mp4");
    }
}
