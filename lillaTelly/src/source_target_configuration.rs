use serde::Deserialize;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceTargetConfiguration {
    pub source: String,
    pub target: String,
}
