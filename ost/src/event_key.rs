use std::fmt;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Event,
    Expulsion,
    Feed,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Event => write!(f, "{}", r#"Event"#),
            EventType::Expulsion => write!(f, "{}", r#"Expulsion"#),
            EventType::Feed => write!(f, "{}", r#"Feed"#),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OstEventKey {
    pub t: EventType,
    pub id: u32,
}
