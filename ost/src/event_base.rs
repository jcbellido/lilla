use chrono::prelude::*;

use crate::event_key::OstEventKey;

pub trait EventBase {
    fn person_name(&self) -> String;

    fn is_person_active(&self) -> bool;

    fn time_stamp(&self) -> &DateTime<Utc>;

    fn summary(&self) -> String;

    fn key(&self) -> OstEventKey;
}
