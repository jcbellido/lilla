use chrono::prelude::*;

use crate::entity_impl::person_impl::PersonImpl;
use crate::event::{Event, EventType};
use crate::event_base::EventBase;
use crate::event_key::{EventType as context_EventType, OstEventKey};

#[derive(Clone, Debug)]
pub struct RemoteEventImpl {
    pub id: u32,
    pub time_stamp: DateTime<Utc>,
    pub person: PersonImpl,
    pub event: EventType,
}

impl Event for RemoteEventImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn modify_event(&mut self, time_stamp: DateTime<Utc>, event: EventType) {
        self.time_stamp = time_stamp;
        self.event = event;
    }

    fn event(&self) -> EventType {
        self.event.clone()
    }

    fn serialize(&self) -> String {
        panic!("Serializing remote event impl is not supported ... yet (??)")
    }
}

impl EventBase for RemoteEventImpl {
    fn person_name(&self) -> String {
        self.person.name.clone()
    }

    fn is_person_active(&self) -> bool {
        self.person.is_active
    }

    fn time_stamp(&self) -> &DateTime<Utc> {
        &self.time_stamp
    }

    fn summary(&self) -> String {
        match &self.event {
            EventType::Bath => "Bath".to_string(),
            EventType::Medicine(m) => format!("Med: {:#?}", m),
            EventType::Sleep => "Sleep".to_string(),
            EventType::Awake => "Awake".to_string(),
            EventType::Note(n) => format!("Note: {:#?}", n),
            EventType::Temperature(t) => format!("Temp: {:#?}", t),
        }
    }

    fn key(&self) -> OstEventKey {
        OstEventKey {
            t: context_EventType::Event,
            id: self.id,
        }
    }
}
