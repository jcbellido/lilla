use fake::Dummy;

use std::any::Any;

use chrono::prelude::*;

use serde_derive::{Deserialize, Serialize};

use crate::event_base::EventBase;

#[derive(Clone, Debug, Dummy, Serialize, Deserialize, PartialEq)]
pub enum ExpulsionDegree {
    Clean,
    Pee,
    Shart,
    Poopies,
    Pooplosion,
}

pub trait Expulsion: EventBase {
    fn degree(&self) -> ExpulsionDegree;

    fn modify_expulsion(&mut self, degree: ExpulsionDegree, time_stamp: DateTime<Utc>);

    // https://bennetthardwick.com/rust/downcast-trait-object/
    fn as_any(&self) -> &dyn Any;

    fn serialize(&self) -> String;
}
