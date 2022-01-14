use chrono::prelude::*;
use chrono::Utc;

use ost::context::construct_monolith;
// use ost::context::Context;
use ost::event::EventType;

#[test]
fn create_event() {
    let mono_file: &str = "./test_output/test_events_00.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    let mut monolith = construct_monolith(mono_file).unwrap();
    let zardoz = monolith.add_person("Zardoz").unwrap();

    let event_bath = monolith.add_event(&zardoz, EventType::Bath);
    assert!(event_bath.is_ok());

    let event_medicine = monolith.add_event(&zardoz, EventType::Medicine("water".to_string()));
    assert!(event_medicine.is_ok());

    let events = monolith.events();
    assert!(!events.is_empty());

    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn created_events_persist() {
    let mono_file: &str = "./test_output/test_events_01.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith = construct_monolith(mono_file).unwrap();
        let zardoz = monolith.add_person("Zardoz").unwrap();
        let _ = monolith.add_event(&zardoz, EventType::Bath);
        let _ = monolith.add_event(&zardoz, EventType::Medicine("water".to_string()));
    }
    {
        let monolith = construct_monolith(mono_file).unwrap();
        let events = monolith.events();
        assert!(!events.is_empty());
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn events_can_be_modified() {
    let mono_file: &str = "./test_output/test_events_02.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith = construct_monolith(mono_file).unwrap();
        let zardoz = monolith.add_person("Zardoz").unwrap();
        let mut bath = monolith.add_event(&zardoz, EventType::Bath).unwrap();

        let bd = Utc.ymd(1979, 6, 10).and_hms(0, 0, 0);
        bath.modify_event(bd, EventType::Awake);

        let res = monolith.modify_event(&bath);
        assert!(res.is_ok());
    }
    {
        let monolith = construct_monolith(mono_file).unwrap();
        let events = monolith.events();
        let awake = events.first().unwrap();
        assert_eq!(awake.event(), EventType::Awake);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn events_can_be_deleted() {
    let mono_file: &str = "./test_output/test_events_03.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith = construct_monolith(mono_file).unwrap();
        let zardoz = monolith.add_person("Zardoz").unwrap();
        let _ = monolith.add_event(&zardoz, EventType::Awake);
        let _ = monolith.add_event(&zardoz, EventType::Bath);
        let _ = monolith.add_event(&zardoz, EventType::Medicine("water".to_string()));
        let _ = monolith.add_event(&zardoz, EventType::Note("notie".to_string()));
        let _ = monolith.add_event(&zardoz, EventType::Sleep);
        let _ = monolith.add_event(&zardoz, EventType::Temperature(37.5));
    }
    {
        let mut monolith = construct_monolith(mono_file).unwrap();

        let mut events = monolith.events();

        let event_to_remove = events.remove(
            events
                .iter()
                .position(|event| event.event() == EventType::Bath)
                .unwrap(),
        );
        let result = monolith.remove_event(event_to_remove);
        assert!(result.is_ok());
    }
    {
        let monolith = construct_monolith(mono_file).unwrap();
        assert_eq!(monolith.events().len(), 5);

        let should_be_none = monolith
            .events()
            .iter()
            .position(|event| event.event() == EventType::Bath);
        assert!(should_be_none.is_none());
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}
