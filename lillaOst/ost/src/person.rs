use std::{any::Any, fmt::Debug};

use crate::person_key::OstPersonKey;

use crate::entity_impl::person_impl::PersonImpl;

pub trait Person: Debug {
    fn is_active(&self) -> bool;

    fn set_is_active(&mut self, is_active: bool);

    fn name(&self) -> &str;

    fn set_name(&mut self, name: &str);

    // https://bennetthardwick.com/rust/downcast-trait-object/
    fn as_any(&self) -> &dyn Any;

    fn serialize(&self) -> String;

    fn key(&self) -> OstPersonKey;
}

pub fn deserialize(json: &str) -> Result<Box<dyn Person>, String> {
    let deserialized_person: PersonImpl = match serde_json::from_str(json) {
        Ok(p) => p,
        Err(err) => return Err(err.to_string()),
    };
    Ok(Box::new(deserialized_person))
}
