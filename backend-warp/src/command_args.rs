use chrono::prelude::*;

use ost::event::EventType;
use ost::event_key::OstEventKey;
use ost::expulsion::ExpulsionDegree;
use ost::person_key::OstPersonKey;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgAddPerson {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgFakeCount {
    pub count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgAModifyPerson {
    pub person_key: OstPersonKey,
    pub serialized_person: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgEntityKey {
    pub event_key: OstEventKey,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgAddEvent {
    pub person_key: OstPersonKey,
    pub new_event: EventType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgAModifyEvent {
    pub event_key: OstEventKey,
    pub time_stamp: DateTime<Utc>,
    pub event_payload: EventType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgAddExpulsion {
    pub person_key: OstPersonKey,
    pub expulsion_degree: ExpulsionDegree,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgModifyExpulsion {
    pub event_key: OstEventKey,
    pub time_stamp: DateTime<Utc>,
    pub expulsion_degree: ExpulsionDegree,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgAddFeeding {
    pub person_key: OstPersonKey,
    pub breast_milk: u32,
    pub formula: u32,
    pub solids: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgAModifyFeeding {
    pub event_key: OstEventKey,
    pub time_stamp: DateTime<Utc>,
    pub breast_milk: u32,
    pub formula: u32,
    pub solids: u32,
}
