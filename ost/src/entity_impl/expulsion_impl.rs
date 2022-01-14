use std::{cell::RefCell, rc::Rc};

use chrono::prelude::*;

use serde_derive::{Deserialize, Serialize};

use crate::entity_impl::person_impl::PersonImpl;
use crate::event_base::EventBase;
use crate::event_key::{EventType, OstEventKey};
use crate::expulsion::{Expulsion, ExpulsionDegree};
use crate::person::Person;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExpulsionPersistence {
    pub id: u32,
    pub degree: ExpulsionDegree,
    pub time_stamp: DateTime<Utc>,
    pub person_id: u32,
}

#[derive(Clone, Debug)]
pub struct ExpulsionImpl {
    pub id: u32,
    pub degree: ExpulsionDegree,
    pub time_stamp: DateTime<Utc>,
    pub person: Rc<RefCell<PersonImpl>>,
}

impl Expulsion for ExpulsionImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn degree(&self) -> ExpulsionDegree {
        self.degree.clone()
    }

    fn modify_expulsion(&mut self, degree: ExpulsionDegree, time_stamp: DateTime<Utc>) {
        self.degree = degree;
        self.time_stamp = time_stamp;
    }

    fn serialize(&self) -> String {
        serde_json::to_string(&self.to_persistence()).unwrap()
    }
}

impl EventBase for ExpulsionImpl {
    fn person_name(&self) -> String {
        self.person.borrow().name().to_string()
    }

    fn time_stamp(&self) -> &DateTime<Utc> {
        &self.time_stamp
    }

    fn summary(&self) -> String {
        format!("{:#?}", self.degree)
    }

    fn is_person_active(&self) -> bool {
        self.person.borrow().is_active()
    }

    fn key(&self) -> OstEventKey {
        OstEventKey {
            t: EventType::Expulsion,
            id: self.id,
        }
    }
}

impl ExpulsionImpl {
    pub fn new(person: Rc<RefCell<PersonImpl>>, id: u32, degree: ExpulsionDegree) -> Self {
        ExpulsionImpl {
            id,
            degree,
            time_stamp: Utc::now(),
            person,
        }
    }

    pub fn from_persistence(
        persisted: &ExpulsionPersistence,
        persons: &[Rc<RefCell<PersonImpl>>],
    ) -> Self {
        let existing_person = persons
        .iter()
        .find(|p| {
            let unpacked_p = &***p;
            unpacked_p.borrow().id() == persisted.person_id
        })
        .unwrap_or_else(|| panic!("broken reference persisted feeding points to person {} that can't be found in provided persons", persisted.person_id));

        ExpulsionImpl {
            id: persisted.id,
            degree: persisted.degree.clone(),
            time_stamp: persisted.time_stamp,
            person: existing_person.clone(),
        }
    }

    pub fn to_persistence(&self) -> ExpulsionPersistence {
        let person = &*self.person;
        let person_id = person.borrow().id();

        ExpulsionPersistence {
            id: self.id,
            degree: self.degree.clone(),
            time_stamp: self.time_stamp,
            person_id,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn from_expulsion(expulsion: &Box<dyn Expulsion>) -> &ExpulsionImpl {
        expulsion
            .as_any()
            .downcast_ref::<ExpulsionImpl>()
            .expect("wrong feed type")
    }
}
