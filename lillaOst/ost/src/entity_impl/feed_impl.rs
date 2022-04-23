use std::{cell::RefCell, rc::Rc};

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{
    event_base::EventBase, event_key::EventType, event_key::OstEventKey, feed::Feed, person::Person,
};

use super::person_impl::PersonImpl;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeedPersistence {
    pub id: u32,
    pub breast_milk: u32,
    pub formula: u32,
    pub solids: u32,
    pub time_stamp: DateTime<Utc>,
    pub person_id: u32,
}

#[derive(Clone, Debug)]
pub struct FeedImpl {
    pub id: u32,
    pub breast_milk: u32,
    pub formula: u32,
    pub solids: u32,
    pub time_stamp: DateTime<Utc>,
    pub person: Rc<RefCell<PersonImpl>>,
}

impl Feed for FeedImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn breast_milk(&self) -> u32 {
        self.breast_milk
    }

    fn formula(&self) -> u32 {
        self.formula
    }

    fn solids(&self) -> u32 {
        self.solids
    }

    fn modify_feed(
        &mut self,
        breast_milk: u32,
        formula: u32,
        solids: u32,
        time_stamp: DateTime<Utc>,
    ) {
        self.breast_milk = breast_milk;
        self.formula = formula;
        self.solids = solids;
        self.time_stamp = time_stamp;
    }

    fn serialize(&self) -> String {
        serde_json::to_string(&self.to_persistence()).unwrap()
    }
}

impl EventBase for FeedImpl {
    fn person_name(&self) -> String {
        self.person.borrow().name().to_string()
    }

    fn time_stamp(&self) -> &DateTime<Utc> {
        &self.time_stamp
    }

    fn summary(&self) -> String {
        format!(
            "BM: {} F: {} Solids: {}",
            self.breast_milk, self.formula, self.solids
        )
    }

    fn is_person_active(&self) -> bool {
        self.person.borrow().is_active()
    }

    fn key(&self) -> OstEventKey {
        OstEventKey {
            t: EventType::Feed,
            id: self.id,
        }
    }
}

impl FeedImpl {
    pub fn new(
        person: Rc<RefCell<PersonImpl>>,
        id: u32,
        breast_milk: u32,
        formula: u32,
        solids: u32,
    ) -> Self {
        FeedImpl {
            breast_milk,
            formula,
            solids,
            time_stamp: Utc::now(),
            person,
            id,
        }
    }

    pub fn from_persistence(
        persisted: &FeedPersistence,
        persons: &[Rc<RefCell<PersonImpl>>],
    ) -> Self {
        let existing_person = persons
        .iter()
        .find(|p| {
            let unpacked_p = &***p;
            unpacked_p.borrow().id() == persisted.person_id
        })
        .expect(format!("broken reference persisted feeding points to person {} that can't be found in provided persons", persisted.person_id ).as_str());

        FeedImpl {
            breast_milk: persisted.breast_milk,
            formula: persisted.formula,
            solids: persisted.solids,
            time_stamp: persisted.time_stamp,
            person: existing_person.clone(),
            id: persisted.id,
        }
    }

    pub fn to_persistence(&self) -> FeedPersistence {
        let person = &*self.person;
        let person_id = person.borrow().id();

        FeedPersistence {
            breast_milk: self.breast_milk,
            formula: self.formula,
            solids: self.solids,
            time_stamp: self.time_stamp,
            person_id: person_id,
            id: self.id,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn from_feed(feed: &Box<dyn Feed>) -> &FeedImpl {
        feed.as_any()
            .downcast_ref::<FeedImpl>()
            .expect("wrong feed type")
    }
}
