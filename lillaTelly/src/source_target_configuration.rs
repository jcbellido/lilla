use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceTargetConfiguration {
    /// path to a dir. I'm expecting the files to be side-by-side here
    pub source: String,
    /// path to a `plex - friendly` dir. Inside there should be a structure of directories Season XX
    /// and inside those dirs Files should be named SXXEYY-[original name]
    pub target: String,

    /// Which season should lillaTelly use as default? Our observation is that unless the order of the original sources is somehow preserved, Plex will fetch bogus summaries and miniatures from the server
    /// all the content handled by lillaTelly will have this season or greater
    pub default_season: u32,
    /// Which episode should be the first
    pub default_episode: u32,
}
