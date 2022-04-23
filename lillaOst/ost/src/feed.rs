use chrono::prelude::*;
use std::any::Any;

use crate::event_base::EventBase;

pub trait Feed: EventBase {
    fn breast_milk(&self) -> u32;
    fn formula(&self) -> u32;
    fn solids(&self) -> u32;

    fn modify_feed(
        &mut self,
        breast_milk: u32,
        formula: u32,
        solids: u32,
        time_stamp: DateTime<Utc>,
    );

    // https://bennetthardwick.com/rust/downcast-trait-object/
    fn as_any(&self) -> &dyn Any;

    fn serialize(&self) -> String;
}
