use std::{cell::RefCell, rc::Rc};

use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;

use gloo_storage::{LocalStorage, Storage};

use super::context_monolithic_impl::ContextMonolithicImpl;

use crate::entity_impl::event_impl::EventImpl;
use crate::entity_impl::expulsion_impl::ExpulsionImpl;
use crate::entity_impl::feed_impl::FeedImpl;
use crate::entity_impl::person_impl::PersonImpl;

use crate::entity_impl::event_impl::EventPersistence;
use crate::entity_impl::expulsion_impl::ExpulsionPersistence;
use crate::entity_impl::feed_impl::FeedPersistence;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct ContextPersistence {
    persons: Vec<PersonImpl>,
    feeds: Vec<FeedPersistence>,
    expulsions: Vec<ExpulsionPersistence>,
    events: Vec<EventPersistence>,
}

pub fn new_monolith(storage_key: &str) -> Result<ContextMonolithicImpl, String> {
    let data: Result<String, _> = LocalStorage::get(storage_key);
    match data {
        Ok(payload) => match from_str::<ContextPersistence>(payload.as_str()) {
            Ok(persistence) => {
                let mut persons: Vec<Rc<RefCell<PersonImpl>>> = vec![];
                persistence
                    .persons
                    .iter()
                    .for_each(|p| persons.push(Rc::new(RefCell::new(p.clone()))));

                let feeds = persistence
                    .feeds
                    .iter()
                    .map(|f| FeedImpl::from_persistence(f, &persons))
                    .collect();

                let expulsions = persistence
                    .expulsions
                    .iter()
                    .map(|e| ExpulsionImpl::from_persistence(e, &persons))
                    .collect();

                let events = persistence
                    .events
                    .iter()
                    .map(|event| EventImpl::from_persistence(event, &persons))
                    .collect();

                Ok(ContextMonolithicImpl {
                    target_file: storage_key.to_string(),
                    persons,
                    feeds,
                    expulsions,
                    events,
                    persist_function: persist,
                })
            }
            Err(err) => Err(err.to_string()),
        },
        Err(_) => {
            let nu_monolith = ContextMonolithicImpl {
                target_file: storage_key.to_string(),
                persons: vec![],
                feeds: vec![],
                expulsions: vec![],
                events: vec![],
                persist_function: persist,
            };
            persist(&nu_monolith)?;
            Ok(nu_monolith)
        }
    }
}

pub fn persist(monolith: &ContextMonolithicImpl) -> Result<(), String> {
    let persons_to_persist = monolith
        .persons
        .iter()
        .map(|f| f.borrow().clone())
        .collect();

    let feeds_to_persist = monolith.feeds.iter().map(|f| f.to_persistence()).collect();

    let expulsions_to_persist = monolith
        .expulsions
        .iter()
        .map(|e| e.to_persistence())
        .collect();

    let events_to_persist = monolith.events.iter().map(|e| e.to_persistence()).collect();

    let to_persistence = ContextPersistence {
        persons: persons_to_persist,
        feeds: feeds_to_persist,
        expulsions: expulsions_to_persist,
        events: events_to_persist,
    };
    let _ignore = LocalStorage::set(monolith.target_file.as_str(), &to_persistence);
    Ok(())
}
