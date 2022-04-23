use std::{cell::RefCell, rc::Rc};

use chrono::prelude::*;

use serde_derive::{Deserialize, Serialize};

use crate::entity_impl::person_impl::PersonImpl;
use crate::event::{Event, EventType};
use crate::event_base::EventBase;
use crate::event_key::{EventType as context_EventType, OstEventKey};
use crate::person::Person;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventPersistence {
    pub id: u32,
    pub time_stamp: DateTime<Utc>,
    pub person_id: u32,
    pub event: EventType,
}

#[derive(Clone, Debug)]
pub struct EventImpl {
    pub id: u32,
    pub time_stamp: DateTime<Utc>,
    pub person: Rc<RefCell<PersonImpl>>,
    pub event: EventType,
}

impl Event for EventImpl {
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
        serde_json::to_string(&self.to_persistence()).unwrap()
    }
}

impl EventBase for EventImpl {
    fn person_name(&self) -> String {
        self.person.borrow().name().to_string()
    }

    fn is_person_active(&self) -> bool {
        self.person.borrow().is_active()
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

impl EventImpl {
    pub fn new(person: Rc<RefCell<PersonImpl>>, id: u32, event: EventType) -> Self {
        EventImpl {
            id,
            time_stamp: Utc::now(),
            person,
            event,
        }
    }

    pub fn from_persistence(
        persisted: &EventPersistence,
        persons: &[Rc<RefCell<PersonImpl>>],
    ) -> Self {
        let existing_person = persons
        .iter()
        .find(|p| {
            let unpacked_p = &***p;
            unpacked_p.borrow().id() == persisted.person_id
        })
        .unwrap_or_else(|| panic!("broken reference persisted feeding points to person {} that can't be found in provided persons", persisted.person_id));
        EventImpl {
            id: persisted.id,
            event: persisted.event.clone(),
            time_stamp: persisted.time_stamp,
            person: existing_person.clone(),
        }
    }

    pub fn to_persistence(&self) -> EventPersistence {
        let person = &*self.person;
        let person_id = person.borrow().id();
        EventPersistence {
            id: self.id,
            time_stamp: self.time_stamp,
            person_id,
            event: self.event.clone(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn from_event(event: &Box<dyn Event>) -> &EventImpl {
        event
            .as_any()
            .downcast_ref::<EventImpl>()
            .expect("wrong event type")
    }
}
