use std::vec;

use chrono::prelude::*;

use serde_derive::{Deserialize, Serialize};

use crate::context::Context;
use crate::entity_impl::event_impl::EventPersistence;
use crate::entity_impl::expulsion_impl::ExpulsionPersistence;
use crate::entity_impl::feed_impl::FeedPersistence;
use crate::entity_impl::person_impl::PersonImpl;
use crate::entity_impl::remote_event_impl::RemoteEventImpl;
use crate::entity_impl::remote_expulsion_impl::RemoteExpulsionImpl;
use crate::entity_impl::remote_feed_impl::RemoteFeedImpl;
use crate::event::{Event, EventType};
use crate::event_key::{EventType as ost_EventKey, OstEventKey};
use crate::expulsion::{Expulsion, ExpulsionDegree};
use crate::feed::Feed;
use crate::person::{deserialize as person_deserialize, Person};
use crate::person_key::OstPersonKey;

pub fn new_monolith(
    remote_endpoint: &str,
    get_call: fn(&str) -> Result<String, String>,
    post_call: fn(&str, String) -> Result<String, String>,
    post_empty_call: fn(&str) -> Result<String, String>,
) -> Result<RemoteMonolithicContextImpl, String> {
    Ok(RemoteMonolithicContextImpl {
        remote_endpoint: remote_endpoint.to_string(),
        get_call,
        post_call,
        post_empty_call,
    })
}

#[derive(Serialize, Deserialize)]
struct ArgAddNameCommand {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct ArgFakeCount {
    pub count: u32,
}

#[derive(Serialize, Deserialize)]
struct ArgEventKey {
    pub event_key: OstEventKey,
}

#[derive(Serialize, Deserialize)]
pub struct ArgAddEvent {
    pub person_key: OstPersonKey,
    pub new_event: EventType,
}

#[derive(Serialize, Deserialize)]
pub struct ArgAddExpulsion {
    pub person_key: OstPersonKey,
    pub expulsion_degree: ExpulsionDegree,
}

#[derive(Serialize, Deserialize)]
pub struct ArgAddFeeding {
    pub person_key: OstPersonKey,
    pub breast_milk: u32,
    pub formula: u32,
    pub solids: u32,
}

// This is the contact from the UI
pub struct RemoteMonolithicContextImpl {
    pub remote_endpoint: String,
    pub get_call: fn(&str) -> Result<String, String>,
    pub post_call: fn(&str, String) -> Result<String, String>,
    pub post_empty_call: fn(&str) -> Result<String, String>,
}

impl Context for RemoteMonolithicContextImpl {
    fn purge_all_data(&mut self) -> Result<(), String> {
        let _ = match (self.post_empty_call)(self.build_api_url("api/admin/reset").as_str()) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
        Ok(())
    }

    fn purge_all_events(&mut self) -> Result<(), String> {
        let _ =
            match (self.post_empty_call)(self.build_api_url("api/admin/purge-all-events").as_str())
            {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        Ok(())
    }

    fn get_base_event_by_key(
        &self,
        key: &crate::event_key::OstEventKey,
    ) -> Option<Box<dyn crate::event_base::EventBase>> {
        match key.t {
            ost_EventKey::Expulsion => match self.get_expulsion_by_key(key) {
                Some(o) => Some(o),
                None => None,
            },
            ost_EventKey::Event => match self.get_event_by_key(key) {
                Some(o) => Some(o),
                None => None,
            },
            ost_EventKey::Feed => match self.get_feeding_by_key(key) {
                Some(o) => Some(o),
                None => None,
            },
        }
    }

    fn persons(&self) -> Vec<Box<dyn crate::person::Person>> {
        self.fetch_persons()
    }

    fn add_person(&mut self, name: &str) -> Result<Box<dyn crate::person::Person>, String> {
        let post_message = serde_json::to_string(&ArgAddNameCommand {
            name: name.to_string(),
        })
        .unwrap();
        let remote_call_result: Result<String, String> =
            match (self.post_call)(self.build_api_url("api/persons").as_str(), post_message) {
                Ok(o) => serde_json::from_str(&o).unwrap(),
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(json) => Ok(person_deserialize(&json).unwrap()),
            Err(e) => Err(e),
        }
    }

    fn add_fake_persons(&mut self, count: u32) -> Result<(), String> {
        let post_message = serde_json::to_string(&ArgFakeCount { count }).unwrap();
        self.post_call_to("api/persons/add-fake-count", post_message)
    }

    fn modify_person(&mut self, person: &Box<dyn crate::person::Person>) -> Result<(), String> {
        #[derive(Serialize)]
        struct ArgAModifyPerson {
            pub person_key: OstPersonKey,
            pub serialized_person: String,
        }

        let message = serde_json::to_string(&ArgAModifyPerson {
            person_key: person.key(),
            serialized_person: person.serialize(),
        })
        .unwrap();

        let remote_call_result: Result<(), String> =
            match (self.post_call)(self.build_api_url("api/person").as_str(), message) {
                Ok(o) => serde_json::from_str(&o).unwrap(),
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn get_person_by_key(&self, key: crate::person_key::OstPersonKey) -> Option<Box<dyn Person>> {
        self.fetch_person_by_key(key)
    }

    fn feedings(&self) -> Vec<Box<dyn crate::feed::Feed>> {
        let p = self.fetch_persons_impl();
        self.fetch_feedings(&p)
    }

    fn feedings_by(
        &self,
        person: &Box<dyn crate::person::Person>,
    ) -> Vec<Box<dyn crate::feed::Feed>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let persons = self.fetch_persons_impl();

        let target_person = match persons.iter().find(|p| p.id() == person_impl_id) {
            Some(p) => p.clone(),
            None => return vec![],
        };

        let mut output: Vec<Box<dyn Feed>> = vec![];
        self.fetch_feedings_persistence()
            .iter()
            .filter(|f| f.person_id == person_impl_id)
            .for_each(|feed_persistence| {
                output.push(Box::new(RemoteFeedImpl {
                    id: feed_persistence.id,
                    breast_milk: feed_persistence.breast_milk,
                    formula: feed_persistence.formula,
                    solids: feed_persistence.solids,
                    time_stamp: feed_persistence.time_stamp,
                    person: target_person.clone(),
                }))
            });
        output
    }

    fn add_feeding(
        &mut self,
        person: &Box<dyn crate::person::Person>,
        breast_milk: u32,
        formula: u32,
        solids: u32,
    ) -> Result<Box<dyn crate::feed::Feed>, String> {
        let post_message = serde_json::to_string(&ArgAddFeeding {
            person_key: person.key(),
            breast_milk,
            formula,
            solids,
        })
        .unwrap();
        match (self.post_call)(&self.build_api_url("api/feedings/add"), post_message) {
            Ok(server_message) => {
                let unpacked_server_message: Result<String, String> =
                    serde_json::from_str(&server_message).unwrap();
                match unpacked_server_message {
                    Ok(server_response) => {
                        let de_serialized_feedings: FeedPersistence =
                            serde_json::from_str(&server_response).unwrap();
                        match self
                            .fetch_persons_impl()
                            .iter()
                            .find(|pi| pi.id() == de_serialized_feedings.person_id)
                        {
                            Some(p) => Ok(Box::new(RemoteFeedImpl {
                                id: de_serialized_feedings.id,
                                time_stamp: de_serialized_feedings.time_stamp,
                                person: p.clone(),
                                breast_milk: de_serialized_feedings.breast_milk,
                                formula: de_serialized_feedings.formula,
                                solids: de_serialized_feedings.solids,
                            })),
                            None => Err(format!("Person {:#?} not found!", person.key())),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn add_fake_feedings(&mut self, count: u32) -> Result<(), String> {
        let post_message = serde_json::to_string(&ArgFakeCount { count }).unwrap();
        self.post_call_to("api/feedings/add-fake-count", post_message)
    }

    fn modify_feeding(&mut self, feed: &Box<dyn crate::feed::Feed>) -> Result<(), String> {
        #[derive(Serialize)]
        struct ArgAModifyFeeding {
            pub event_key: OstEventKey,
            pub time_stamp: DateTime<Utc>,
            pub breast_milk: u32,
            pub formula: u32,
            pub solids: u32,
        }

        let message = serde_json::to_string(&ArgAModifyFeeding {
            event_key: feed.key(),
            time_stamp: feed.time_stamp().clone(),
            breast_milk: feed.breast_milk(),
            formula: feed.formula(),
            solids: feed.solids(),
        })
        .unwrap();

        let remote_call_result: Result<(), String> =
            match (self.post_call)(self.build_api_url("api/feed").as_str(), message) {
                Ok(_) => Ok(()), // Server is returning a ()
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn remove_feeding(&mut self, feed: Box<dyn crate::feed::Feed>) -> Result<(), String> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: feed.key(),
        })
        .unwrap();
        self.post_call_to("api/feedings/remove", event_key)
    }

    fn get_feeding_by_key(
        &self,
        key: &crate::event_key::OstEventKey,
    ) -> Option<Box<dyn crate::feed::Feed>> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: key.clone(),
        })
        .unwrap();

        match (self.post_call)(self.build_api_url("api/feedings").as_str(), event_key) {
            Ok(o) => {
                let is_some: Option<String> = serde_json::from_str(&o).unwrap();
                match is_some {
                    Some(payload) => {
                        let event_persistence: FeedPersistence =
                            serde_json::from_str(&payload).unwrap();

                        let persons = self.fetch_persons_impl();
                        match persons
                            .iter()
                            .find(|p| p.id() == event_persistence.person_id)
                        {
                            Some(p) => Some(Box::new(RemoteFeedImpl {
                                id: event_persistence.id,
                                time_stamp: event_persistence.time_stamp,
                                person: p.clone(),
                                breast_milk: event_persistence.breast_milk,
                                formula: event_persistence.formula,
                                solids: event_persistence.solids,
                            })),
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            Err(_) => None,
        }
    }

    fn expulsions(&self) -> Vec<Box<dyn crate::expulsion::Expulsion>> {
        let p = self.fetch_persons_impl();
        self.fetch_expulsions(&p)
    }

    fn expulsions_by(&self, person: &Box<dyn crate::person::Person>) -> Vec<Box<dyn Expulsion>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let persons = self.fetch_persons_impl();

        let target_person = match persons.iter().find(|p| p.id() == person_impl_id) {
            Some(p) => p.clone(),
            None => return vec![],
        };

        let mut output: Vec<Box<dyn Expulsion>> = vec![];
        self.fetch_expulsions_persistence()
            .iter()
            .filter(|f| f.person_id == person_impl_id)
            .for_each(|expulsion_persistence| {
                output.push(Box::new(RemoteExpulsionImpl {
                    id: expulsion_persistence.id,
                    time_stamp: expulsion_persistence.time_stamp,
                    person: target_person.clone(),
                    degree: expulsion_persistence.degree.clone(),
                }))
            });
        output
    }

    fn add_expulsion(
        &mut self,
        person: &Box<dyn crate::person::Person>,
        degree: crate::expulsion::ExpulsionDegree,
    ) -> Result<Box<dyn crate::expulsion::Expulsion>, String> {
        let post_message = serde_json::to_string(&ArgAddExpulsion {
            person_key: person.key(),
            expulsion_degree: degree,
        })
        .unwrap();
        match (self.post_call)(&self.build_api_url("api/expulsions/add"), post_message) {
            Ok(server_message) => {
                let unpacked_server_message: Result<String, String> =
                    serde_json::from_str(&server_message).unwrap();
                match unpacked_server_message {
                    Ok(server_response) => {
                        let de_serialized_expulsion: ExpulsionPersistence =
                            serde_json::from_str(&server_response).unwrap();
                        match self
                            .fetch_persons_impl()
                            .iter()
                            .find(|pi| pi.id() == de_serialized_expulsion.person_id)
                        {
                            Some(p) => Ok(Box::new(RemoteExpulsionImpl {
                                id: de_serialized_expulsion.id,
                                time_stamp: de_serialized_expulsion.time_stamp,
                                person: p.clone(),
                                degree: de_serialized_expulsion.degree,
                            })),
                            None => Err(format!("Person {:#?} not found!", person.key())),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn add_fake_expulsions(&mut self, count: u32) -> Result<(), String> {
        let post_message = serde_json::to_string(&ArgFakeCount { count }).unwrap();
        self.post_call_to("api/expulsions/add-fake-count", post_message)
    }

    fn modify_expulsion(
        &mut self,
        expulsion: &Box<dyn crate::expulsion::Expulsion>,
    ) -> Result<(), String> {
        #[derive(Serialize)]
        struct ArgModifyExpulsion {
            pub event_key: OstEventKey,
            pub time_stamp: DateTime<Utc>,
            pub expulsion_degree: ExpulsionDegree,
        }

        let message = serde_json::to_string(&ArgModifyExpulsion {
            event_key: expulsion.key(),
            time_stamp: expulsion.time_stamp().clone(),
            expulsion_degree: expulsion.degree(),
        })
        .unwrap();

        let remote_call_result: Result<(), String> =
            match (self.post_call)(self.build_api_url("api/expulsion").as_str(), message) {
                Ok(_) => Ok(()), // Server is returning a ()
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn remove_expulsion(
        &mut self,
        expulsion: Box<dyn crate::expulsion::Expulsion>,
    ) -> Result<(), String> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: expulsion.key(),
        })
        .unwrap();
        self.post_call_to("api/expulsions/remove", event_key)
    }

    fn get_expulsion_by_key(
        &self,
        key: &crate::event_key::OstEventKey,
    ) -> Option<Box<dyn crate::expulsion::Expulsion>> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: key.clone(),
        })
        .unwrap();

        match (self.post_call)(self.build_api_url("api/expulsions").as_str(), event_key) {
            Ok(o) => {
                let is_some: Option<String> = serde_json::from_str(&o).unwrap();
                match is_some {
                    Some(payload) => {
                        let event_persistence: ExpulsionPersistence =
                            serde_json::from_str(&payload).unwrap();

                        let persons = self.fetch_persons_impl();
                        match persons
                            .iter()
                            .find(|p| p.id() == event_persistence.person_id)
                        {
                            Some(p) => Some(Box::new(RemoteExpulsionImpl {
                                id: event_persistence.id,
                                time_stamp: event_persistence.time_stamp,
                                person: p.clone(),
                                degree: event_persistence.degree.clone(),
                            })),
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            Err(_) => None,
        }
    }

    fn events(&self) -> Vec<Box<dyn crate::event::Event>> {
        let p = self.fetch_persons_impl();
        self.fetch_events(&p)
    }

    fn events_by(&self, person: &Box<dyn crate::person::Person>) -> Vec<Box<dyn Event>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let persons = self.fetch_persons_impl();

        let target_person = match persons.iter().find(|p| p.id() == person_impl_id) {
            Some(p) => p.clone(),
            None => return vec![],
        };

        let mut output: Vec<Box<dyn Event>> = vec![];
        self.fetch_events_persistence()
            .iter()
            .filter(|f| f.person_id == person_impl_id)
            .for_each(|event_persistence| {
                output.push(Box::new(RemoteEventImpl {
                    id: event_persistence.id,
                    time_stamp: event_persistence.time_stamp,
                    person: target_person.clone(),
                    event: event_persistence.event.clone(),
                }))
            });
        output
    }

    fn add_event(
        &mut self,
        person: &Box<dyn crate::person::Person>,
        event_type: crate::event::EventType,
    ) -> Result<Box<dyn crate::event::Event>, String> {
        let post_message = serde_json::to_string(&ArgAddEvent {
            person_key: person.key(),
            new_event: event_type,
        })
        .unwrap();
        match (self.post_call)(&self.build_api_url("api/events/add"), post_message) {
            Ok(server_message) => {
                let unpacked_server_message: Result<String, String> =
                    serde_json::from_str(&server_message).unwrap();
                match unpacked_server_message {
                    Ok(server_response) => {
                        let de_serialized_event: EventPersistence =
                            serde_json::from_str(&server_response).unwrap();
                        match self
                            .fetch_persons_impl()
                            .iter()
                            .find(|pi| pi.id() == de_serialized_event.person_id)
                        {
                            Some(p) => Ok(Box::new(RemoteEventImpl {
                                id: de_serialized_event.id,
                                time_stamp: de_serialized_event.time_stamp,
                                person: p.clone(),
                                event: de_serialized_event.event,
                            })),
                            None => Err(format!("Person {:#?} not found!", person.key())),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn add_fake_events(&mut self, count: u32) -> Result<(), String> {
        let post_message = serde_json::to_string(&ArgFakeCount { count }).unwrap();
        self.post_call_to("api/events/add-fake-count", post_message)
    }

    fn modify_event(&mut self, event: &Box<dyn crate::event::Event>) -> Result<(), String> {
        #[derive(Serialize)]
        struct ArgAModifyEvent {
            pub event_key: OstEventKey,
            pub time_stamp: DateTime<Utc>,
            pub event_payload: EventType,
        }

        let message = serde_json::to_string(&ArgAModifyEvent {
            event_key: event.key(),
            time_stamp: event.time_stamp().clone(),
            event_payload: event.event(),
        })
        .unwrap();

        let remote_call_result: Result<(), String> =
            match (self.post_call)(self.build_api_url("api/event").as_str(), message) {
                Ok(_) => Ok(()), // Server is returning a ()
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn remove_event(&mut self, event: Box<dyn crate::event::Event>) -> Result<(), String> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: event.key(),
        })
        .unwrap();
        self.post_call_to("api/events/remove", event_key)
    }

    fn get_event_by_key(
        &self,
        key: &crate::event_key::OstEventKey,
    ) -> Option<Box<dyn crate::event::Event>> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: key.clone(),
        })
        .unwrap();

        match (self.post_call)(self.build_api_url("api/events").as_str(), event_key) {
            Ok(o) => {
                let is_some: Option<String> = serde_json::from_str(&o).unwrap();
                match is_some {
                    Some(payload) => {
                        let event_persistence: EventPersistence =
                            serde_json::from_str(&payload).unwrap();

                        let persons = self.fetch_persons_impl();
                        match persons
                            .iter()
                            .find(|p| p.id() == event_persistence.person_id)
                        {
                            Some(p) => Some(Box::new(RemoteEventImpl {
                                id: event_persistence.id,
                                time_stamp: event_persistence.time_stamp,
                                person: p.clone(),
                                event: event_persistence.event.clone(),
                            })),
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            Err(_) => None,
        }
    }
}

impl RemoteMonolithicContextImpl {
    fn build_api_url(&self, api_path: &str) -> String {
        format!("{}/{}", self.remote_endpoint, api_path)
    }

    fn post_call_to(&self, api_path: &str, payload: String) -> Result<(), String> {
        let remote_call_result: Result<(), String> =
            match (self.post_call)(self.build_api_url(api_path).as_str(), payload) {
                Ok(o) => serde_json::from_str(&o).unwrap(),
                Err(e) => return Err(e),
            };
        remote_call_result
    }

    fn fetch_person_by_key(&self, key: crate::person_key::OstPersonKey) -> Option<Box<dyn Person>> {
        if let Ok(serialized_persons) = (self.get_call)(self.build_api_url("api/persons").as_str())
        {
            let vec_of_serialized_persons: Vec<String> =
                serde_json::from_str(&serialized_persons).unwrap();

            #[allow(clippy::needless_collect)]
            let available_persons: Vec<PersonImpl> = vec_of_serialized_persons
                .iter()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();

            match available_persons.iter().find(|f| f.id() == key.id) {
                Some(p) => Some(Box::new(p.clone())),
                None => None,
            }
        } else {
            None
        }
    }

    fn fetch_persons(&self) -> Vec<Box<dyn Person>> {
        if let Ok(serialized_persons) = (self.get_call)(self.build_api_url("api/persons").as_str())
        {
            let vec_of_serialized_persons: Vec<String> =
                serde_json::from_str(&serialized_persons).unwrap();

            #[allow(clippy::needless_collect)]
            let reconstructed: Vec<Box<dyn Person>> = vec_of_serialized_persons
                .iter()
                .map(|s| person_deserialize(s).unwrap())
                .collect();
            reconstructed
        } else {
            vec![]
        }
    }

    fn fetch_persons_impl(&self) -> Vec<PersonImpl> {
        if let Ok(serialized_persons) = (self.get_call)(self.build_api_url("api/persons").as_str())
        {
            let vec_of_serialized_persons: Vec<String> =
                serde_json::from_str(&serialized_persons).unwrap();

            let reconstructed: Vec<PersonImpl> = vec_of_serialized_persons
                .iter()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();
            reconstructed
        } else {
            vec![]
        }
    }

    fn fetch_feedings(&self, persons: &[PersonImpl]) -> Vec<Box<dyn Feed>> {
        let deserialized_feedings = self.fetch_feedings_persistence();

        if !deserialized_feedings.is_empty() {
            let mut output: Vec<Box<dyn Feed>> = vec![];

            deserialized_feedings
                .iter()
                .map(|feed_persistence| {
                    let person_impl = persons
                        .iter()
                        .find(|person| person.id == feed_persistence.person_id)
                        .unwrap()
                        .clone();
                    RemoteFeedImpl {
                        id: feed_persistence.id,
                        breast_milk: feed_persistence.breast_milk,
                        formula: feed_persistence.formula,
                        solids: feed_persistence.solids,
                        time_stamp: feed_persistence.time_stamp,
                        person: person_impl,
                    }
                })
                .for_each(|f| output.push(Box::new(f)));
            output
        } else {
            vec![]
        }
    }

    fn fetch_feedings_persistence(&self) -> Vec<FeedPersistence> {
        if let Ok(serialized_feedings) =
            (self.get_call)(self.build_api_url("api/feedings").as_str())
        {
            let vec_of_serialized_feedings: Vec<String> =
                serde_json::from_str(&serialized_feedings).unwrap();
            let deserialized_feedings: Vec<FeedPersistence> = vec_of_serialized_feedings
                .iter()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();
            deserialized_feedings
        } else {
            vec![]
        }
    }

    fn fetch_expulsions(&self, persons: &[PersonImpl]) -> Vec<Box<dyn Expulsion>> {
        let deserialized_expulsions = self.fetch_expulsions_persistence();

        if !deserialized_expulsions.is_empty() {
            let mut output: Vec<Box<dyn Expulsion>> = vec![];

            deserialized_expulsions
                .iter()
                .map(|expulsion_persistence| {
                    let person_impl = persons
                        .iter()
                        .find(|person| person.id == expulsion_persistence.person_id)
                        .unwrap()
                        .clone();
                    RemoteExpulsionImpl {
                        id: expulsion_persistence.id,
                        time_stamp: expulsion_persistence.time_stamp,
                        person: person_impl,
                        degree: expulsion_persistence.degree.clone(),
                    }
                })
                .for_each(|f| output.push(Box::new(f)));
            output
        } else {
            vec![]
        }
    }

    fn fetch_expulsions_persistence(&self) -> Vec<ExpulsionPersistence> {
        if let Ok(serialized_feedings) =
            (self.get_call)(self.build_api_url("api/expulsions").as_str())
        {
            let vec_of_serialized_expulsions: Vec<String> =
                serde_json::from_str(&serialized_feedings).unwrap();
            let deserialized_expulsions: Vec<ExpulsionPersistence> = vec_of_serialized_expulsions
                .iter()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();
            deserialized_expulsions
        } else {
            vec![]
        }
    }

    fn fetch_events(&self, persons: &[PersonImpl]) -> Vec<Box<dyn Event>> {
        let deserialized_events = self.fetch_events_persistence();

        if !deserialized_events.is_empty() {
            let mut output: Vec<Box<dyn Event>> = vec![];

            deserialized_events
                .iter()
                .map(|events_persistence| {
                    let person_impl = persons
                        .iter()
                        .find(|person| person.id == events_persistence.person_id)
                        .unwrap()
                        .clone();
                    RemoteEventImpl {
                        id: events_persistence.id,
                        time_stamp: events_persistence.time_stamp,
                        person: person_impl,
                        event: events_persistence.event.clone(),
                    }
                })
                .for_each(|f| output.push(Box::new(f)));
            output
        } else {
            vec![]
        }
    }

    fn fetch_events_persistence(&self) -> Vec<EventPersistence> {
        if let Ok(serialized_feedings) = (self.get_call)(self.build_api_url("api/events").as_str())
        {
            let vec_of_serialized_events: Vec<String> =
                serde_json::from_str(&serialized_feedings).unwrap();
            let deserialized_events: Vec<EventPersistence> = vec_of_serialized_events
                .iter()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();
            deserialized_events
        } else {
            vec![]
        }
    }
}
