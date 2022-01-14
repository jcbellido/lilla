use std::any::Any;

use serde_derive::{Deserialize, Serialize};

use crate::person::Person;
use crate::person_key::OstPersonKey;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonImpl {
    pub id: u32,
    pub name: String,
    pub is_active: bool,
}

impl PersonImpl {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            is_active: true,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn from_person(person: &Box<dyn Person>) -> &PersonImpl {
        person
            .as_any()
            .downcast_ref::<PersonImpl>()
            .expect("Wrong type")
    }
}

impl Person for PersonImpl {
    #[allow(dead_code)]
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    #[allow(dead_code)]
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn key(&self) -> crate::person_key::OstPersonKey {
        OstPersonKey { id: self.id }
    }
}

impl PartialEq for PersonImpl {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::PersonImpl;
    use crate::person::{deserialize, Person};

    #[test]
    fn create_person() {
        let nu_person = PersonImpl::new(77, "nu name");
        assert!(nu_person.is_active());
        assert_eq!(nu_person.name(), "nu name");
    }

    #[test]
    fn serialize_person() {
        let p_once = PersonImpl::new(11, "once");
        let serialized_p_once = p_once.serialize();
        assert!(!serialized_p_once.is_empty());

        let rebuilt_once = deserialize(&serialized_p_once).unwrap();
        assert_eq!(p_once.name(), rebuilt_once.name());
    }
}
