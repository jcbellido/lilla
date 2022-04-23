#![feature(trait_upcasting)]
#![allow(incomplete_features)]

use ost::context::construct_monolith;
use ost::event_base::EventBase;
use ost::event_key::OstEventKey;

fn populate_with_fake_data(mono_file: &str, count_persons: u32, count_events: u32) {
    let mut monolith_context = construct_monolith(mono_file).unwrap();
    assert!(monolith_context.add_fake_persons(count_persons).is_ok());
    assert!(monolith_context.add_fake_feedings(count_events).is_ok());
    assert!(monolith_context.add_fake_expulsions(count_events).is_ok());
    assert!(monolith_context.add_fake_events(count_events).is_ok());
}

#[test]
fn entities_have_external_keys() {
    let mono_file: &str = "./test_output/external_keys_00.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    populate_with_fake_data(mono_file, 1, 1);
    let target_key: OstEventKey;
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let feed = monolith_context.feedings().pop().unwrap();
        let base_feed = feed as Box<dyn EventBase>;
        target_key = base_feed.key();
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let recovered_feed = monolith_context.get_base_event_by_key(&target_key);
        assert!(recovered_feed.is_some());
        let recovered_feed = recovered_feed.unwrap();
        assert_eq!(recovered_feed.key().t, target_key.t);
        assert_eq!(recovered_feed.key().id, target_key.id);

        println!("{:#?}", target_key);

        let feed = monolith_context.get_feeding_by_key(&target_key);
        assert!(feed.is_some());
    }

    let _ignore_fail = std::fs::remove_file(mono_file);
}
