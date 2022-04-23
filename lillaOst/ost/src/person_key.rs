use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct OstPersonKey {
    pub id: u32,
}
