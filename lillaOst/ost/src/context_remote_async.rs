use std::boxed::Box;
use std::vec;

use chrono::prelude::*;

use serde_derive::{Deserialize, Serialize};

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
#[derive(Clone, PartialEq)]
pub struct AsyncRemoteMonolith {}

impl AsyncRemoteMonolith {
    pub async fn purge_all_data(&self) -> Result<(), String> {
        let _ = match post_message("api/admin/reset", None).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
        Ok(())
    }

    pub async fn purge_all_events(&self) -> Result<(), String> {
        let _ = match post_message("api/admin/purge-all-events", None).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
        Ok(())
    }

    pub async fn get_base_event_by_key(
        &self,
        key: &crate::event_key::OstEventKey,
    ) -> Option<Box<dyn crate::event_base::EventBase>> {
        match key.t {
            ost_EventKey::Expulsion => match self.get_expulsion_by_key(key).await {
                Some(o) => Some(o),
                None => None,
            },
            ost_EventKey::Event => match self.get_event_by_key(key).await {
                Some(o) => Some(o),
                None => None,
            },
            ost_EventKey::Feed => match self.get_feeding_by_key(key).await {
                Some(o) => Some(o),
                None => None,
            },
        }
    }

    pub async fn persons(&self) -> Vec<Box<dyn crate::person::Person>> {
        self.fetch_persons().await
    }

    pub async fn add_person(&self, name: &str) -> Result<Box<dyn crate::person::Person>, String> {
        let payload = serde_json::to_string(&ArgAddNameCommand {
            name: name.to_string(),
        })
        .unwrap();
        let remote_call_result: Result<String, String> =
            match post_message("api/persons", Some(payload)).await {
                Ok(o) => serde_json::from_str(&o).unwrap(),
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(json) => Ok(person_deserialize(&json).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub async fn add_fake_persons(&self, count: u32) -> Result<(), String> {
        let payload = serde_json::to_string(&ArgFakeCount { count }).unwrap();
        match post_message("api/persons/add-fake-count", Some(payload)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn modify_person(
        &self,
        person: &Box<dyn crate::person::Person>,
    ) -> Result<(), String> {
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
            match post_message("api/person", Some(message)).await {
                Ok(o) => serde_json::from_str(&o).unwrap(),
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_person_by_key(
        &self,
        key: crate::person_key::OstPersonKey,
    ) -> Option<Box<dyn Person>> {
        self.fetch_person_by_key(key).await
    }

    pub async fn feedings_by(
        &self,
        person: &Box<dyn crate::person::Person>,
    ) -> Vec<Box<dyn crate::feed::Feed>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let persons = self.fetch_persons_impl().await;

        let target_person = match persons.iter().find(|p| p.id() == person_impl_id) {
            Some(p) => p.clone(),
            None => return vec![],
        };

        let mut output: Vec<Box<dyn Feed>> = vec![];
        self.fetch_feedings_persistence()
            .await
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

    pub async fn add_feeding(
        &self,
        person: &Box<dyn crate::person::Person>,
        breast_milk: u32,
        formula: u32,
        solids: u32,
    ) -> Result<Box<dyn crate::feed::Feed>, String> {
        let payload = serde_json::to_string(&ArgAddFeeding {
            person_key: person.key(),
            breast_milk,
            formula,
            solids,
        })
        .unwrap();
        match post_message("api/feedings/add", Some(payload)).await {
            Ok(server_message) => {
                let unpacked_server_message: Result<String, String> =
                    serde_json::from_str(&server_message).unwrap();
                match unpacked_server_message {
                    Ok(server_response) => {
                        let de_serialized_feedings: FeedPersistence =
                            serde_json::from_str(&server_response).unwrap();
                        match self
                            .fetch_persons_impl()
                            .await
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

    pub async fn add_fake_feedings(&self, count: u32) -> Result<(), String> {
        let payload = serde_json::to_string(&ArgFakeCount { count }).unwrap();
        match post_message("api/feedings/add-fake-count", Some(payload)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn modify_feeding(&self, feed: &Box<dyn crate::feed::Feed>) -> Result<(), String> {
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
            match post_message("api/feed", Some(message)).await {
                Ok(_) => Ok(()), // Server is returning a ()
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn remove_feeding(&self, feed: Box<dyn crate::feed::Feed>) -> Result<(), String> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: feed.key(),
        })
        .unwrap();
        match post_message("api/feedings/remove", Some(event_key)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_feeding_by_key(
        &self,
        key: &crate::event_key::OstEventKey,
    ) -> Option<Box<dyn crate::feed::Feed>> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: key.clone(),
        })
        .unwrap();

        match post_message("api/feedings", Some(event_key)).await {
            Ok(o) => {
                let is_some: Option<String> = serde_json::from_str(&o).unwrap();
                match is_some {
                    Some(payload) => {
                        let event_persistence: FeedPersistence =
                            serde_json::from_str(&payload).unwrap();

                        let persons = self.fetch_persons_impl().await;
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

    pub async fn expulsions(&self) -> Vec<Box<dyn crate::expulsion::Expulsion>> {
        let p = self.fetch_persons_impl().await;
        self.fetch_expulsions(&p).await
    }

    pub async fn expulsions_by(
        &self,
        person: &Box<dyn crate::person::Person>,
    ) -> Vec<Box<dyn Expulsion>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let persons = self.fetch_persons_impl().await;

        let target_person = match persons.iter().find(|p| p.id() == person_impl_id) {
            Some(p) => p.clone(),
            None => return vec![],
        };

        let mut output: Vec<Box<dyn Expulsion>> = vec![];
        self.fetch_expulsions_persistence()
            .await
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

    pub async fn add_expulsion(
        &self,
        person: &Box<dyn crate::person::Person>,
        degree: crate::expulsion::ExpulsionDegree,
    ) -> Result<Box<dyn crate::expulsion::Expulsion>, String> {
        let payload = serde_json::to_string(&ArgAddExpulsion {
            person_key: person.key(),
            expulsion_degree: degree,
        })
        .unwrap();
        match post_message("api/expulsions/add", Some(payload)).await {
            Ok(server_message) => {
                let unpacked_server_message: Result<String, String> =
                    serde_json::from_str(&server_message).unwrap();
                match unpacked_server_message {
                    Ok(server_response) => {
                        let de_serialized_expulsion: ExpulsionPersistence =
                            serde_json::from_str(&server_response).unwrap();
                        match self
                            .fetch_persons_impl()
                            .await
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

    pub async fn add_fake_expulsions(&self, count: u32) -> Result<(), String> {
        let payload = serde_json::to_string(&ArgFakeCount { count }).unwrap();
        match post_message("api/expulsions/add-fake-count", Some(payload)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn modify_expulsion(
        &self,
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
            match post_message("api/expulsion", Some(message)).await {
                Ok(_) => Ok(()), // Server is returning a ()
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn remove_expulsion(
        &self,
        expulsion: Box<dyn crate::expulsion::Expulsion>,
    ) -> Result<(), String> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: expulsion.key(),
        })
        .unwrap();
        match post_message("api/expulsions/remove", Some(event_key)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_expulsion_by_key(
        &self,
        key: &crate::event_key::OstEventKey,
    ) -> Option<Box<dyn crate::expulsion::Expulsion>> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: key.clone(),
        })
        .unwrap();

        match post_message("api/expulsions", Some(event_key)).await {
            Ok(o) => {
                let is_some: Option<String> = serde_json::from_str(&o).unwrap();
                match is_some {
                    Some(payload) => {
                        let event_persistence: ExpulsionPersistence =
                            serde_json::from_str(&payload).unwrap();

                        let persons = self.fetch_persons_impl().await;
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

    pub async fn events(&self) -> Vec<Box<dyn crate::event::Event>> {
        let p = self.fetch_persons_impl().await;
        self.fetch_events(&p).await
    }

    pub async fn events_by(&self, person: &Box<dyn crate::person::Person>) -> Vec<Box<dyn Event>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let persons = self.fetch_persons_impl().await;

        let target_person = match persons.iter().find(|p| p.id() == person_impl_id) {
            Some(p) => p.clone(),
            None => return vec![],
        };

        let mut output: Vec<Box<dyn Event>> = vec![];
        self.fetch_events_persistence()
            .await
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

    pub async fn add_event(
        &self,
        person: &Box<dyn crate::person::Person>,
        event_type: crate::event::EventType,
    ) -> Result<Box<dyn crate::event::Event>, String> {
        let payload = serde_json::to_string(&ArgAddEvent {
            person_key: person.key(),
            new_event: event_type,
        })
        .unwrap();
        match post_message("api/events/add", Some(payload)).await {
            Ok(server_message) => {
                let unpacked_server_message: Result<String, String> =
                    serde_json::from_str(&server_message).unwrap();
                match unpacked_server_message {
                    Ok(server_response) => {
                        let de_serialized_event: EventPersistence =
                            serde_json::from_str(&server_response).unwrap();
                        match self
                            .fetch_persons_impl()
                            .await
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

    pub async fn add_fake_events(&self, count: u32) -> Result<(), String> {
        let payload = serde_json::to_string(&ArgFakeCount { count }).unwrap();
        match post_message("api/events/add-fake-count", Some(payload)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn modify_event(&self, event: &Box<dyn crate::event::Event>) -> Result<(), String> {
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
            match post_message("api/event", Some(message)).await {
                Ok(_) => Ok(()), // Server is returning a ()
                Err(e) => return Err(e),
            };

        match remote_call_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn remove_event(&self, event: Box<dyn crate::event::Event>) -> Result<(), String> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: event.key(),
        })
        .unwrap();
        match post_message("api/events/remove", Some(event_key)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_event_by_key(
        &self,
        key: &crate::event_key::OstEventKey,
    ) -> Option<Box<dyn crate::event::Event>> {
        let event_key = serde_json::to_string(&ArgEventKey {
            event_key: key.clone(),
        })
        .unwrap();

        match post_message("api/events", Some(event_key)).await {
            Ok(o) => {
                let is_some: Option<String> = serde_json::from_str(&o).unwrap();
                match is_some {
                    Some(payload) => {
                        let event_persistence: EventPersistence =
                            serde_json::from_str(&payload).unwrap();

                        let persons = self.fetch_persons_impl().await;
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

    pub async fn feedings(&self) -> Vec<Box<dyn crate::feed::Feed>> {
        let p = self.fetch_persons_impl().await;
        self.fetch_feedings(&p).await
    }
}

impl AsyncRemoteMonolith {
    async fn fetch_person_by_key(
        &self,
        key: crate::person_key::OstPersonKey,
    ) -> Option<Box<dyn Person>> {
        if let Ok(serialized_persons) = get_string("api/persons").await {
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

    async fn fetch_persons(&self) -> Vec<Box<dyn Person>> {
        if let Ok(serialized_persons) = get_string("api/persons").await {
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

    async fn fetch_persons_impl(&self) -> Vec<PersonImpl> {
        if let Ok(serialized_persons) = get_string("api/persons").await {
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

    async fn fetch_feedings(&self, persons: &[PersonImpl]) -> Vec<Box<dyn Feed>> {
        let deserialized_feedings = self.fetch_feedings_persistence().await;

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

    async fn fetch_feedings_persistence(&self) -> Vec<FeedPersistence> {
        if let Ok(serialized_feedings) = get_string("api/feedings").await {
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

    async fn fetch_expulsions(&self, persons: &[PersonImpl]) -> Vec<Box<dyn Expulsion>> {
        let deserialized_expulsions = self.fetch_expulsions_persistence().await;

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

    async fn fetch_expulsions_persistence(&self) -> Vec<ExpulsionPersistence> {
        if let Ok(serialized_feedings) = get_string("api/expulsions").await {
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

    async fn fetch_events(&self, persons: &[PersonImpl]) -> Vec<Box<dyn Event>> {
        let deserialized_events = self.fetch_events_persistence().await;

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

    async fn fetch_events_persistence(&self) -> Vec<EventPersistence> {
        if let Ok(serialized_feedings) = get_string("api/events").await {
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

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub async fn get_string(url: &'static str) -> Result<String, String> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = match Request::new_with_str_and_init(url, &opts) {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .as_string()
                .unwrap_or_else(|| "Unknown error constructing request".to_string()))
        }
    };

    let window = gloo_utils::window();
    let resp_value = match JsFuture::from(window.fetch_with_request(&request)).await {
        Ok(o) => o,
        Err(e) => {
            return Err(e
                .as_string()
                .unwrap_or_else(|| "Unknown error calling window fetch with request".to_string()))
        }
    };

    let resp: Response = resp_value.dyn_into().unwrap();

    match resp.text() {
        Ok(o) => {
            let text = match JsFuture::from(o).await {
                Ok(t) => t,
                Err(e) => {
                    return Err(e
                        .as_string()
                        .unwrap_or_else(|| "Unknown error unpacking response".to_string()))
                }
            };
            Ok(text.as_string().unwrap())
        }
        Err(e) => Err(e
            .as_string()
            .unwrap_or_else(|| "Unknown error unpacking response".to_string())),
    }
}

/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub async fn post_message(url: &'static str, payload: Option<String>) -> Result<String, String> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let has_payload = payload.is_some();

    if has_payload {
        opts.body(Some(&JsValue::from_str(&payload.unwrap())));
    }

    let request = match Request::new_with_str_and_init(url, &opts) {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .as_string()
                .unwrap_or_else(|| "Unknown error constructing request".to_string()))
        }
    };

    if has_payload {
        match request.headers().set("content-type", "application/json") {
            Ok(_) => {}
            Err(e) => {
                return Err(e
                    .as_string()
                    .unwrap_or_else(|| "Can't modify headers".to_string()))
            }
        }
    }

    let window = gloo_utils::window();
    let resp_value = match JsFuture::from(window.fetch_with_request(&request)).await {
        Ok(o) => o,
        Err(e) => {
            return Err(e
                .as_string()
                .unwrap_or_else(|| "Unknown error calling window fetch with request".to_string()))
        }
    };

    let resp: Response = resp_value.dyn_into().unwrap();

    match resp.text() {
        Ok(o) => {
            let text = match JsFuture::from(o).await {
                Ok(t) => t,
                Err(e) => {
                    return Err(e
                        .as_string()
                        .unwrap_or_else(|| "Unknown error unpacking response".to_string()))
                }
            };
            Ok(text.as_string().unwrap())
        }
        Err(e) => Err(e
            .as_string()
            .unwrap_or_else(|| "Unknown error unpacking response".to_string())),
    }
}
