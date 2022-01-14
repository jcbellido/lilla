use chrono::prelude::*;

use crate::entity_impl::person_impl::PersonImpl;
use crate::event_base::EventBase;
use crate::event_key::{EventType, OstEventKey};
use crate::expulsion::{Expulsion, ExpulsionDegree};

#[derive(Clone, Debug)]
pub struct RemoteExpulsionImpl {
    pub id: u32,
    pub degree: ExpulsionDegree,
    pub time_stamp: DateTime<Utc>,
    pub person: PersonImpl,
}

impl Expulsion for RemoteExpulsionImpl {
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
        panic!("Serializing remote expulsion impl is not supported ... yet (??)")
    }
}

impl EventBase for RemoteExpulsionImpl {
    fn person_name(&self) -> String {
        self.person.name.clone()
    }

    fn time_stamp(&self) -> &DateTime<Utc> {
        &self.time_stamp
    }

    fn summary(&self) -> String {
        format!("{:#?}", self.degree)
    }

    fn is_person_active(&self) -> bool {
        self.person.is_active
    }

    fn key(&self) -> OstEventKey {
        OstEventKey {
            t: EventType::Expulsion,
            id: self.id,
        }
    }
}
