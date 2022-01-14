use chrono::prelude::*;
use chrono::Utc;

use ost::context::construct_monolith;
use ost::feed::Feed;

#[test]
fn on_context_construct_feedings_are_empty() {
    let mono_file: &str = "./test_output/feed_changes_00.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    let monolith_context = construct_monolith(mono_file).unwrap();
    let feeds = monolith_context.feedings();
    assert!(feeds.is_empty());
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn feeds_can_be_constructed_and_persisted() {
    let mono_file: &str = "./test_output/feed_changes_01.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        monolith_context
            .add_feeding(&zardoz, 11, 22, 33)
            .expect("feed insertion works");
        let feeds = monolith_context.feedings();
        assert!(!feeds.is_empty());
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let feeds = monolith_context.feedings();
        assert!(!feeds.is_empty());
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn feeds_can_be_read() {
    let mono_file: &str = "./test_output/feed_changes_02.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        monolith_context
            .add_feeding(&zardoz, 11, 22, 33)
            .expect("feed insertion works");
        let feeds = monolith_context.feedings();
        assert!(!feeds.is_empty());
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let feeds = monolith_context.feedings();
        let first_feed = feeds.first().expect("feeds aren't empty");
        assert_eq!(first_feed.breast_milk(), 11);
        assert_eq!(first_feed.formula(), 22);
        assert_eq!(first_feed.solids(), 33);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn feeds_have_timestamps() {
    let mono_file: &str = "./test_output/feed_changes_03.json";
    let _ignore_fail = std::fs::remove_file(mono_file);

    let created_feed: Box<dyn Feed>;
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        created_feed = monolith_context
            .add_feeding(&zardoz, 11, 22, 33)
            .expect("feed insertion works");
        assert_eq!(created_feed.formula(), 22);
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let feeds = monolith_context.feedings();
        let persisted_feed = feeds.first().expect("persisted feed");
        assert_eq!(created_feed.time_stamp(), persisted_feed.time_stamp());
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn feeds_have_names() {
    let mono_file: &str = "./test_output/feed_changes_04.json";
    let _ignore_fail = std::fs::remove_file(mono_file);

    let created_feed: Box<dyn Feed>;
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        created_feed = monolith_context
            .add_feeding(&zardoz, 11, 22, 33)
            .expect("feed insertion works");
        assert_eq!(created_feed.formula(), 22);
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let persons = monolith_context.persons();
        let should_be_zardoz = persons.first().unwrap();
        let feeds = monolith_context.feedings();
        let persisted_feed = feeds.first().expect("persisted feed should be there (??)");
        assert_eq!(persisted_feed.person_name(), should_be_zardoz.name());
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn feeds_can_be_modified() {
    let mono_file: &str = "./test_output/feed_changes_05.json";
    let _ignore_fail = std::fs::remove_file(mono_file);

    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let mut created_feed = monolith_context
            .add_feeding(&zardoz, 11, 22, 33)
            .expect("feed insertion works");

        let bd = Utc.ymd(1979, 6, 10).and_hms(0, 0, 0);

        created_feed.modify_feed(44, 55, 66, bd);

        assert_eq!(created_feed.breast_milk(), 44);
        assert_eq!(created_feed.formula(), 55);
        assert_eq!(created_feed.solids(), 66);

        let result = monolith_context.modify_feeding(&created_feed);
        assert!(result.is_ok());
    }

    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let feeds = monolith_context.feedings();
        assert_eq!(feeds.len(), 1);
        let last_feed = feeds.first().unwrap();

        let bd = Utc.ymd(1979, 6, 10).and_hms(0, 0, 0);

        assert_eq!(last_feed.breast_milk(), 44);
        assert_eq!(last_feed.formula(), 55);
        assert_eq!(last_feed.solids(), 66);
        assert_eq!(last_feed.time_stamp().clone(), bd);
    }

    let _ignore_fail = std::fs::remove_file(mono_file);
}
