use std::ops::Add;
use std::vec;
use std::{cell::RefCell, rc::Rc};

use chrono::{prelude::*, Duration};

use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::{Fake, Faker};
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::context::Context;
use crate::entity_impl::event_impl::EventImpl;
use crate::entity_impl::expulsion_impl::ExpulsionImpl;
use crate::entity_impl::feed_impl::FeedImpl;
use crate::entity_impl::person_impl::PersonImpl;
use crate::event::{Event, EventType};
use crate::event_base::EventBase;
use crate::event_key::{EventType as ost_EventKey, OstEventKey};
use crate::expulsion::{Expulsion, ExpulsionDegree};
use crate::feed::Feed;
use crate::person::Person;

pub struct ContextMonolithicImpl {
    pub target_file: String,
    pub persons: Vec<Rc<RefCell<PersonImpl>>>,
    pub feeds: Vec<FeedImpl>,
    pub expulsions: Vec<ExpulsionImpl>,
    pub events: Vec<EventImpl>,
    pub persist_function: fn(&ContextMonolithicImpl) -> Result<(), String>,
}

impl Context for ContextMonolithicImpl {
    fn purge_all_data(&mut self) -> Result<(), String> {
        self.events.clear();
        self.expulsions.clear();
        self.feeds.clear();
        self.persons.clear();

        (self.persist_function)(self)?;
        Ok(())
    }

    fn purge_all_events(&mut self) -> Result<(), String> {
        self.events.clear();
        self.expulsions.clear();
        self.feeds.clear();

        (self.persist_function)(self)?;
        Ok(())
    }

    fn persons(&self) -> Vec<Box<dyn Person>> {
        let mut output: Vec<Box<dyn Person>> = vec![];
        self.persons.iter().for_each(|p| {
            let nu_p: Box<dyn Person> = Box::new(p.borrow().clone());
            output.push(nu_p);
        });
        output
    }

    fn add_person(&mut self, name: &str) -> Result<Box<dyn Person>, String> {
        if let Some(_existing) = self.persons.iter().find(|p| p.borrow().name() == name) {
            return Err(format!("Person with name {} already exists", name));
        }

        let p = PersonImpl::new(self.persons.len() as u32, name);
        self.persons.push(Rc::new(RefCell::new(p.clone())));
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(Box::new(p))
    }

    fn add_fake_persons(&mut self, count: u32) -> Result<(), String> {
        for _i in 0..count {
            let first_name: String = Name(EN).fake();
            let p = PersonImpl::new(self.persons.len() as u32, first_name.as_str());
            self.persons.push(Rc::new(RefCell::new(p.clone())));
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn modify_person(&mut self, person: &Box<dyn Person>) -> Result<(), String> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        if let Some(existing_person) = self
            .persons
            .iter_mut()
            .find(|p| p.borrow().id() == person_impl_id)
        {
            let original_name = existing_person.borrow().name().to_string();
            existing_person
                .borrow_mut()
                .set_is_active(person.is_active());

            if original_name != person.name() {
                existing_person.borrow_mut().set_name(person.name());
            }
        } else {
            return Err(format!("No person with name {} found", person.name()));
        }
        (self.persist_function)(self)?;
        Ok(())
    }

    fn get_person_by_key(&self, key: crate::person_key::OstPersonKey) -> Option<Box<dyn Person>> {
        match self.persons.iter().find(|p| p.borrow().id() == key.id) {
            Some(p) => Some(Box::new(p.borrow().clone())),
            None => None,
        }
    }

    fn feedings(&self) -> Vec<Box<dyn Feed>> {
        let mut output: Vec<Box<dyn Feed>> = vec![];
        self.feeds.iter().for_each(|feed| {
            let nu_feed: Box<dyn Feed> = Box::new(feed.clone());
            output.push(nu_feed);
        });
        output
    }

    fn feedings_by(&self, person: &Box<dyn Person>) -> Vec<Box<dyn Feed>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let _existing_p = self
            .persons
            .iter()
            .find(|p| p.borrow().id() == person_impl_id)
            .expect("this person should exist");

        let mut output: Vec<Box<dyn Feed>> = vec![];
        self.feeds
            .iter()
            .filter(|p| p.person.borrow().id() == person_impl_id)
            .for_each(|f| {
                let nu_feed: Box<dyn Feed> = Box::new(f.clone());
                output.push(nu_feed);
            });
        output
    }

    fn add_feeding(
        &mut self,
        person: &Box<dyn Person>,
        breast_milk: u32,
        formula: u32,
        solids: u32,
    ) -> Result<Box<dyn Feed>, String> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let existing_p = self
            .persons
            .iter()
            .find(|p| p.borrow().id() == person_impl_id)
            .expect("this person should exist");

        let f = FeedImpl::new(
            existing_p.clone(),
            self.feeds.len() as u32,
            breast_milk,
            formula,
            solids,
        );

        self.feeds.push(f.clone());
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(Box::new(f))
    }

    fn add_fake_feedings(&mut self, count: u32) -> Result<(), String> {
        assert!(
            !self.persons.is_empty(),
            "You can't add fake feedings without creating persons first!"
        );

        let mut rng = rand::thread_rng();
        for _i in 0..count {
            let existing_person = self
                .persons
                .get(rng.gen_range(0..self.persons.len() as usize))
                .expect("getting rand person, this person should exist ?");
            let mut f = FeedImpl::new(
                existing_person.clone(),
                self.feeds.len() as u32,
                rng.gen_range(0..150),
                rng.gen_range(0..150),
                rng.gen_range(0..150),
            );

            f.time_stamp = ContextMonolithicImpl::random_time_stamp_in_the_past(&mut rng);

            self.feeds.push(f.clone());
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn modify_feeding(&mut self, feed: &Box<dyn Feed>) -> Result<(), String> {
        let feed_impl_id: u32 = FeedImpl::from_feed(feed).id();

        if let Some(existing_feed) = self.feeds.iter_mut().find(|f| f.id() == feed_impl_id) {
            existing_feed.modify_feed(
                feed.breast_milk(),
                feed.formula(),
                feed.solids(),
                *feed.time_stamp(),
            );
        } else {
            return Err(format!(
                "Feeding not found: {} {}",
                feed.person_name(),
                feed.time_stamp(),
            ));
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn remove_feeding(&mut self, feed: Box<dyn Feed>) -> Result<(), String> {
        let feed_id_to_remove: u32 = FeedImpl::from_feed(&feed).id();

        if let Some(pos_to_remove) = self
            .feeds
            .iter()
            .position(|feed| feed.id() == feed_id_to_remove)
        {
            let _ = self.feeds.remove(pos_to_remove);
        } else {
            return Err(format!(
                "Feeding not found: {} {}",
                feed.person_name(),
                feed.time_stamp(),
            ));
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn expulsions(&self) -> Vec<Box<dyn Expulsion>> {
        let mut output: Vec<Box<dyn Expulsion>> = vec![];
        self.expulsions.iter().for_each(|feed| {
            let nu_expulsion: Box<dyn Expulsion> = Box::new(feed.clone());
            output.push(nu_expulsion);
        });
        output
    }

    fn expulsions_by(&self, person: &Box<dyn Person>) -> Vec<Box<dyn Expulsion>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let _existing_p = self
            .persons
            .iter()
            .find(|p| p.borrow().id() == person_impl_id)
            .expect("this person should exist");

        let mut output: Vec<Box<dyn Expulsion>> = vec![];
        self.expulsions
            .iter()
            .filter(|p| p.person.borrow().id() == person_impl_id)
            .for_each(|e| {
                let nu_feed: Box<dyn Expulsion> = Box::new(e.clone());
                output.push(nu_feed);
            });
        output
    }

    fn add_expulsion(
        &mut self,
        person: &Box<dyn Person>,
        degree: ExpulsionDegree,
    ) -> Result<Box<dyn Expulsion>, String> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let existing_p = self
            .persons
            .iter()
            .find(|p| p.borrow().id() == person_impl_id)
            .expect("this person should exist");

        let expulsion =
            ExpulsionImpl::new(existing_p.clone(), self.expulsions.len() as u32, degree);

        self.expulsions.push(expulsion.clone());
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(Box::new(expulsion))
    }

    fn add_fake_expulsions(&mut self, count: u32) -> Result<(), String> {
        assert!(
            !self.persons.is_empty(),
            "You can't add fake feedings without creating persons first!"
        );
        let mut rng = rand::thread_rng();
        for _i in 0..count {
            let existing_person = self
                .persons
                .get(rng.gen_range(0..self.persons.len() as usize))
                .expect("getting rand person, this person should exist ?");

            let mut expulsion = ExpulsionImpl::new(
                existing_person.clone(),
                self.expulsions.len() as u32,
                Faker.fake::<ExpulsionDegree>(),
            );

            expulsion.time_stamp = ContextMonolithicImpl::random_time_stamp_in_the_past(&mut rng);
            self.expulsions.push(expulsion.clone());
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn remove_expulsion(&mut self, expulsion: Box<dyn Expulsion>) -> Result<(), String> {
        let expulsion_impl_id: u32 = ExpulsionImpl::from_expulsion(&expulsion).id();

        if let Some(pos_to_remove) = self
            .expulsions
            .iter()
            .position(|feed| feed.id() == expulsion_impl_id)
        {
            let _ = self.expulsions.remove(pos_to_remove);
        } else {
            return Err(format!(
                "Expulsion not found: {} {}",
                expulsion.person_name(),
                expulsion.time_stamp(),
            ));
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn modify_expulsion(&mut self, expulsion: &Box<dyn Expulsion>) -> Result<(), String> {
        let expulsion_impl_id: u32 = ExpulsionImpl::from_expulsion(expulsion).id();

        if let Some(existing_expulsion) = self
            .expulsions
            .iter_mut()
            .find(|exp| exp.id() == expulsion_impl_id)
        {
            existing_expulsion.modify_expulsion(expulsion.degree(), *expulsion.time_stamp());
        } else {
            return Err(format!(
                "Expulsion not found: {} {}",
                expulsion.person_name(),
                expulsion.time_stamp(),
            ));
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn events(&self) -> Vec<Box<dyn crate::event::Event>> {
        let mut output: Vec<Box<dyn Event>> = vec![];
        self.events.iter().for_each(|event| {
            let nu_event: Box<dyn Event> = Box::new(event.clone());
            output.push(nu_event);
        });
        output
    }

    fn events_by(&self, person: &Box<dyn Person>) -> Vec<Box<dyn Event>> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let _existing_p = self
            .persons
            .iter()
            .find(|p| p.borrow().id() == person_impl_id)
            .expect("this person should exist");

        let mut output: Vec<Box<dyn Event>> = vec![];
        self.events
            .iter()
            .filter(|p| p.person.borrow().id() == person_impl_id)
            .for_each(|e| {
                let nu_feed: Box<dyn Event> = Box::new(e.clone());
                output.push(nu_feed);
            });
        output
    }

    fn add_event(
        &mut self,
        person: &Box<dyn Person>,
        event_type: crate::event::EventType,
    ) -> Result<Box<dyn crate::event::Event>, String> {
        let person_impl_id: u32 = PersonImpl::from_person(person).id();

        let existing_p = self
            .persons
            .iter()
            .find(|p| p.borrow().id() == person_impl_id)
            .expect("this person should exist");

        let event = EventImpl::new(existing_p.clone(), self.events.len() as u32, event_type);
        self.events.push(event.clone());
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(Box::new(event))
    }

    fn add_fake_events(&mut self, count: u32) -> Result<(), String> {
        assert!(
            !self.persons.is_empty(),
            "You can't add fake feedings without creating persons first!"
        );
        let mut rng = rand::thread_rng();
        for _i in 0..count {
            let existing_person = self
                .persons
                .get(rng.gen_range(0..self.persons.len() as usize))
                .expect("getting rand person, this person should exist ?");

            let mut event = EventImpl::new(
                existing_person.clone(),
                self.events.len() as u32,
                Faker.fake::<EventType>(),
            );
            event.time_stamp = ContextMonolithicImpl::random_time_stamp_in_the_past(&mut rng);
            self.events.push(event);
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn modify_event(&mut self, event: &Box<dyn Event>) -> Result<(), String> {
        let event_impl_id: u32 = EventImpl::from_event(event).id();
        if let Some(existing_event) = self
            .events
            .iter_mut()
            .find(|event| event.id() == event_impl_id)
        {
            existing_event.modify_event(*event.time_stamp(), event.event());
        } else {
            return Err(format!(
                "Event not found: {} {}",
                event.person_name(),
                event.time_stamp(),
            ));
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn remove_event(&mut self, event: Box<dyn Event>) -> Result<(), String> {
        let event_impl_id: u32 = EventImpl::from_event(&event).id();
        if let Some(index_existing_event) = self
            .events
            .iter()
            .position(|event| event.id() == event_impl_id)
        {
            self.events.remove(index_existing_event);
        } else {
            return Err(format!(
                "Event not found: {} {}",
                event.person_name(),
                event.time_stamp(),
            ));
        }
        self.sort_collection_by_time_stamp();
        (self.persist_function)(self)?;
        Ok(())
    }

    fn get_base_event_by_key(&self, key: &OstEventKey) -> Option<Box<dyn EventBase>> {
        match key.t {
            ost_EventKey::Expulsion => {
                if let Some(expulsion) = self.expulsions.iter().find(|f| f.id() == key.id) {
                    return Some(Box::new(expulsion.clone()) as Box<dyn EventBase>);
                }
            }
            ost_EventKey::Feed => {
                if let Some(feed) = self.feeds.iter().find(|f| f.id() == key.id) {
                    return Some(Box::new(feed.clone()) as Box<dyn EventBase>);
                }
            }
            ost_EventKey::Event => {
                if let Some(event) = self.events.iter().find(|f| f.id() == key.id) {
                    return Some(Box::new(event.clone()) as Box<dyn EventBase>);
                }
            }
        }
        None
    }

    fn get_feeding_by_key(&self, key: &OstEventKey) -> Option<Box<dyn Feed>> {
        if key.t != ost_EventKey::Feed {
            return None;
        }
        if let Some(feed) = self.feeds.iter().find(|f| f.id() == key.id) {
            return Some(Box::new(feed.clone()));
        }
        None
    }

    fn get_expulsion_by_key(&self, key: &OstEventKey) -> Option<Box<dyn Expulsion>> {
        if key.t != ost_EventKey::Expulsion {
            return None;
        }
        if let Some(expulsion) = self.expulsions.iter().find(|f| f.id() == key.id) {
            return Some(Box::new(expulsion.clone()));
        }
        None
    }

    fn get_event_by_key(&self, key: &OstEventKey) -> Option<Box<dyn Event>> {
        if key.t != ost_EventKey::Event {
            return None;
        }
        if let Some(event) = self.events.iter().find(|f| f.id() == key.id) {
            return Some(Box::new(event.clone()));
        }
        None
    }
}

impl ContextMonolithicImpl {
    fn random_time_stamp_in_the_past(rng: &mut ThreadRng) -> DateTime<Utc> {
        let time = Utc::now();
        let days_before = rng.gen_range(0..365);
        let duration_days = Duration::days(-days_before);
        let shift_hours = Duration::hours(rng.gen_range(-22..22));
        let shift_minutes = Duration::minutes(rng.gen_range(-55..55));
        time.add(duration_days).add(shift_hours).add(shift_minutes)
    }

    fn sort_collection_by_time_stamp(&mut self) {
        self.feeds.sort_by(|a, b| b.time_stamp.cmp(&a.time_stamp));
        self.expulsions
            .sort_by(|a, b| b.time_stamp.cmp(&a.time_stamp));
        self.events.sort_by(|a, b| b.time_stamp.cmp(&a.time_stamp));
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Context;
    use crate::context_impl::context_persistence_single_file::{from_file, new_monolith};

    static MONOLITHEMPTY: &str = "./test_data/monolith_empty.json";
    static MONOLITHSAVEDIFFERENT: &str = "./test_output/monolith_save_different.json";
    static MONOLITH00: &str = "./test_output/monolith_00.json";
    static MONOLITHPERSIST: &str = "./test_output/monolith_persist.json";

    #[test]
    fn creates_a_new_file_when_started() {
        let _ignore_result = std::fs::remove_file(MONOLITH00);
        let mono_file = new_monolith(MONOLITH00);
        assert!(mono_file.is_ok(), "{:#?}", mono_file.err());
        assert!(Path::new(MONOLITH00).exists());
        let delete_result = std::fs::remove_file(MONOLITH00);
        assert!(delete_result.is_ok());
    }

    #[test]
    fn can_target_different_files() {
        let _ignore_result = std::fs::remove_file(MONOLITHSAVEDIFFERENT);
        let mono_file = from_file(MONOLITHEMPTY, MONOLITHSAVEDIFFERENT);
        assert!(mono_file.is_ok(), "{:#?}", mono_file.err());
        assert!(Path::new(MONOLITHSAVEDIFFERENT).exists());
        let delete_result = std::fs::remove_file(MONOLITHSAVEDIFFERENT);
        assert!(delete_result.is_ok());
    }

    #[test]
    fn fetch_empty_persons() {
        let persistence = new_monolith(MONOLITHEMPTY).unwrap();
        let persons = persistence.persons();
        assert!(persons.is_empty(), "fetching no persons failed?");
    }

    #[test]
    fn persons_persist_between_executions() {
        let _ignore = std::fs::remove_file(MONOLITHPERSIST);
        {
            let mut persistence = new_monolith(MONOLITHPERSIST).unwrap();
            persistence
                .add_person("paco")
                .expect("add a person should simply work");
        }
        {
            let reloaded_persistence = new_monolith(MONOLITHPERSIST).unwrap();
            let persons = reloaded_persistence.persons();
            assert_eq!(persons.len(), 1, "no added persons in storage");
            let retrieved_paco = persons.first().unwrap();
            assert_eq!("paco", retrieved_paco.name());
        }
        let delete_result = std::fs::remove_file(MONOLITHPERSIST);
        assert!(delete_result.is_ok());
    }
}
