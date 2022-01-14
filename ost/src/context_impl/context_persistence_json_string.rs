use std::vec;
use std::{cell::RefCell, rc::Rc};

use serde_derive::{Deserialize, Serialize};
// use serde_json::to_writer_pretty;

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

#[allow(dead_code)]
pub fn new_monolith(from_string: String) -> Result<ContextMonolithicImpl, String> {
    let persistence: ContextPersistence = match serde_json::from_str(from_string.as_str()) {
        Ok(deserialized) => deserialized,
        Err(err) => return Err(err.to_string()),
    };

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
        target_file: String::default(),
        persons,
        feeds,
        expulsions,
        events,
        persist_function: persist,
    })
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

    let _payload = serde_json::to_string(&to_persistence);

    Ok(())
}
