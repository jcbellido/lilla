use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceTargetConfiguration {
    /// path to a dir. I'm expecting the files to be side-by-side here
    pub source: String,
    /// path to a `plex - friendly` dir. Inside there should be a structure of directories Season XX
    /// and inside those dirs Files should be named SXXEYY-[original name]
    pub target: String,
}
