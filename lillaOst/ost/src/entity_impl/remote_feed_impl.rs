use chrono::prelude::*;

use super::person_impl::PersonImpl;

use crate::event_base::EventBase;
use crate::event_key::{EventType, OstEventKey};
use crate::feed::Feed;

#[derive(Clone, Debug)]
pub struct RemoteFeedImpl {
    pub id: u32,
    pub breast_milk: u32,
    pub formula: u32,
    pub solids: u32,
    pub time_stamp: DateTime<Utc>,
    pub person: PersonImpl,
}

impl Feed for RemoteFeedImpl {
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
        panic!("Serializing remote feed impl is not supported ... yet (??)")
    }
}

impl EventBase for RemoteFeedImpl {
    fn person_name(&self) -> String {
        self.person.name.clone()
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
        self.person.is_active
    }

    fn key(&self) -> OstEventKey {
        OstEventKey {
            t: EventType::Feed,
            id: self.id,
        }
    }
}
