use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::vec;
use std::{cell::RefCell, rc::Rc};

use serde_derive::{Deserialize, Serialize};
use serde_json::to_writer_pretty;

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

pub fn new_monolith(path_to_file: &str) -> Result<ContextMonolithicImpl, String> {
    let target_file = Path::new(path_to_file);
    if !target_file.exists() {
        let nu_monolith = ContextMonolithicImpl {
            target_file: path_to_file.to_string(),
            persons: vec![],
            feeds: vec![],
            expulsions: vec![],
            events: vec![],
            persist_function: persist,
        };
        persist(&nu_monolith)?;
        Ok(nu_monolith)
    } else {
        let persistence: ContextPersistence = match File::open(target_file) {
            Ok(opened_file) => {
                let reader = BufReader::new(opened_file);
                match serde_json::from_reader(reader) {
                    Ok(deserialized) => deserialized,
                    Err(err) => return Err(err.to_string()),
                }
            }
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
            target_file: path_to_file.to_string(),
            persons,
            feeds,
            expulsions,
            events,
            persist_function: persist,
        })
    }
}

pub fn persist(monolith: &ContextMonolithicImpl) -> Result<(), String> {
    if std::path::Path::new(&monolith.target_file).exists() {
        match std::fs::remove_file(&monolith.target_file) {
            Ok(_) => {}
            Err(err) => return Err(err.to_string()),
        }
    }

    let target_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(&monolith.target_file)
    {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    let writer = BufWriter::new(target_file);

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

    match to_writer_pretty(writer, &to_persistence) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

/// Allows the creation of a monolithic context that saves to a different file
pub fn from_file(
    path_origin_file: &str,
    path_destination_file: &str,
) -> Result<ContextMonolithicImpl, String> {
    let mut temp_monolith = new_monolith(path_origin_file)?;
    temp_monolith.target_file = path_destination_file.to_string();
    persist(&temp_monolith)?;
    Ok(temp_monolith)
}
