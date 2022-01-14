use crate::event::{Event, EventType};
use crate::event_base::EventBase;
use crate::event_key::OstEventKey;
use crate::expulsion::{Expulsion, ExpulsionDegree};
use crate::feed::Feed;
use crate::person::Person;
use crate::person_key::OstPersonKey;

use crate::context_impl::context_persistence_in_memory::new_monolith as new_monolith_in_memory;
use crate::context_impl::context_persistence_local_storage::new_monolith as new_monolith_in_local_storage;
use crate::context_impl::context_persistence_single_file::{from_file, new_monolith};
use crate::context_impl::context_remote::new_monolith as new_remote_monolith;

pub trait Context {
    fn purge_all_data(&mut self) -> Result<(), String>;
    fn purge_all_events(&mut self) -> Result<(), String>;
    fn get_base_event_by_key(&self, key: &OstEventKey) -> Option<Box<dyn EventBase>>;

    fn persons(&self) -> Vec<Box<dyn Person>>;
    fn add_person(&mut self, name: &str) -> Result<Box<dyn Person>, String>;
    fn add_fake_persons(&mut self, count: u32) -> Result<(), String>;
    fn modify_person(&mut self, person: &Box<dyn Person>) -> Result<(), String>;

    fn get_person_by_key(&self, key: OstPersonKey) -> Option<Box<dyn Person>>;

    // Feedings block
    fn feedings(&self) -> Vec<Box<dyn Feed>>;
    fn feedings_by(&self, person: &Box<dyn Person>) -> Vec<Box<dyn Feed>>;
    fn add_feeding(
        &mut self,
        person: &Box<dyn Person>,
        breast_milk: u32,
        formula: u32,
        solids: u32,
    ) -> Result<Box<dyn Feed>, String>;
    fn add_fake_feedings(&mut self, count: u32) -> Result<(), String>;
    fn modify_feeding(&mut self, feed: &Box<dyn Feed>) -> Result<(), String>;
    fn remove_feeding(&mut self, feed: Box<dyn Feed>) -> Result<(), String>;
    fn get_feeding_by_key(&self, key: &OstEventKey) -> Option<Box<dyn Feed>>;

    // Expulsion block
    fn expulsions(&self) -> Vec<Box<dyn Expulsion>>;
    fn expulsions_by(&self, person: &Box<dyn Person>) -> Vec<Box<dyn Expulsion>>;
    fn add_expulsion(
        &mut self,
        person: &Box<dyn Person>,
        degree: ExpulsionDegree,
    ) -> Result<Box<dyn Expulsion>, String>;
    fn add_fake_expulsions(&mut self, count: u32) -> Result<(), String>;
    fn modify_expulsion(&mut self, expulsion: &Box<dyn Expulsion>) -> Result<(), String>;
    fn remove_expulsion(&mut self, expulsion: Box<dyn Expulsion>) -> Result<(), String>;
    fn get_expulsion_by_key(&self, key: &OstEventKey) -> Option<Box<dyn Expulsion>>;

    fn events(&self) -> Vec<Box<dyn Event>>;
    fn events_by(&self, person: &Box<dyn Person>) -> Vec<Box<dyn Event>>;
    fn add_event(
        &mut self,
        person: &Box<dyn Person>,
        event_type: EventType,
    ) -> Result<Box<dyn Event>, String>;
    fn add_fake_events(&mut self, count: u32) -> Result<(), String>;
    fn modify_event(&mut self, event: &Box<dyn Event>) -> Result<(), String>;
    fn remove_event(&mut self, event: Box<dyn Event>) -> Result<(), String>;
    fn get_event_by_key(&self, key: &OstEventKey) -> Option<Box<dyn Event>>;
}

pub fn construct_monolith_in_memory() -> Result<Box<dyn Context>, String> {
    Ok(Box::new(new_monolith_in_memory()?))
}

pub fn construct_monolith_in_local_storage(storage_key: &str) -> Result<Box<dyn Context>, String> {
    Ok(Box::new(new_monolith_in_local_storage(storage_key)?))
}

pub fn construct_monolith(path_to_monolith: &str) -> Result<Box<dyn Context>, String> {
    Ok(Box::new(new_monolith(path_to_monolith)?))
}

pub fn construct_monolith_remote(
    remote_endpoint: &str,
    get_call: fn(&str) -> Result<String, String>,
    post_call: fn(&str, String) -> Result<String, String>,
    post_empty_call: fn(&str) -> Result<String, String>,
) -> Result<Box<dyn Context>, String> {
    Ok(Box::new(new_remote_monolith(
        remote_endpoint,
        get_call,
        post_call,
        post_empty_call,
    )?))
}

pub fn construct_monolith_from_file(
    path_to_source: &str,
    path_to_output: &str,
) -> Result<Box<dyn Context>, String> {
    Ok(Box::new(from_file(path_to_source, path_to_output)?))
}
