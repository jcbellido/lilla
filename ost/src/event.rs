use std::any::Any;
use std::fmt;

use fake::{Dummy, Fake};

use chrono::prelude::*;

use serde_derive::{Deserialize, Serialize};

use crate::event_base::EventBase;

#[derive(Clone, Debug, Dummy, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    Bath,
    Medicine(String),
    Sleep,
    Awake,
    Note(String),
    Temperature(f64),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Bath => write!(f, "{}", r#"Bath"#),
            EventType::Medicine(medicine) => write!(f, "Medicine {}", medicine),
            EventType::Sleep => write!(f, "{}", r#"Sleep"#),
            EventType::Awake => write!(f, "{}", r#"Awake"#),
            EventType::Note(note) => write!(f, "Note {}", note),
            EventType::Temperature(_temp) => write!(f, "Temperature"),
        }
    }
}

pub trait Event: EventBase {
    fn event(&self) -> EventType;
    fn modify_event(&mut self, time_stamp: DateTime<Utc>, event: EventType);

    // https://bennetthardwick.com/rust/downcast-trait-object/
    fn as_any(&self) -> &dyn Any;

    fn serialize(&self) -> String;
}
