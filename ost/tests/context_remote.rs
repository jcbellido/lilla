use ost::context::construct_monolith_remote;

use ost::event::EventType;
use ost::expulsion::ExpulsionDegree;

use ost::communications::{get_string_from_network, post_string_to_network, post_to_network};

static REMOTE_ENDPOINT: &str = "http://127.0.0.1:3030";

#[test]
#[ignore = "It's making the test suite go a bit too slowly"]
fn fail_on_backend_not_reachable() {
    let failed_remote = construct_monolith_remote(
        "http://127.0.0.1:5888",
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    );
    assert!(failed_remote.is_err());
}

#[test]
fn builds_on_backend_reachable() {
    let remote_exists = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    );
    assert!(remote_exists.is_ok());
}

// We know the remote server contains 10 persons
#[test]
fn can_list_persons() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    assert_eq!(remote.persons().len(), 10);
}

// We know the remote server contains 10 persons
#[test]
fn can_add_a_person() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    assert_eq!(remote.persons().len(), 10);
    let _ = remote.add_person("manolo").unwrap();
    assert_eq!(remote.persons().len(), 11);

    // Second round
    let _ = remote.purge_all_data();
    assert_eq!(remote.persons().len(), 10);
    let _ = remote.add_person("manolo").unwrap();
    assert_eq!(remote.persons().len(), 11);
    let failed = remote.add_person("manolo");
    assert!(failed.is_err());
}

// We know the remote server contains 10 persons
#[test]
fn can_list_feedings() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    assert_eq!(remote.feedings().len(), 150);
}

#[test]
fn can_modify_person() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let key;
    {
        let mut a_person = remote.persons().pop().unwrap();
        key = a_person.key();
        a_person.set_is_active(false);
        let _ = remote.modify_person(&a_person).unwrap();
    }
    {
        let p = remote.get_person_by_key(key).unwrap();
        assert!(!p.is_active());
    }
}

#[test]
fn get_feedigns_by() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let feedings = remote.feedings_by(&a_person);
    assert!(!feedings.is_empty());
}

#[test]
fn get_expulsions() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let expulsions = remote.expulsions();
    assert!(!expulsions.is_empty());
}

#[test]
fn get_expulsions_by() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let expulsions = remote.expulsions_by(&a_person);
    assert!(!expulsions.is_empty());
}

#[test]
fn get_events() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let expulsions = remote.events();
    assert!(!expulsions.is_empty());
}

#[test]
fn get_events_by() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let events = remote.events_by(&a_person);
    assert!(!events.is_empty());
}

#[test]
fn purge_all_events() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    assert_eq!(remote.events().len(), 150);
    let _ = remote.purge_all_events();
    assert_eq!(remote.events().len(), 0);
}

#[test]
fn add_fake_person() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let p_count = remote.persons().len();
    remote.add_fake_persons(5).unwrap();
    assert_eq!(remote.persons().len(), p_count + 5);
}

#[test]
fn add_fake_event() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let p_count = remote.events().len();
    remote.add_fake_events(5).unwrap();
    assert_eq!(remote.events().len(), p_count + 5);
}

#[test]
fn add_fake_expulsion() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let p_count = remote.expulsions().len();
    remote.add_fake_expulsions(5).unwrap();
    assert_eq!(remote.expulsions().len(), p_count + 5);
}

#[test]
fn add_fake_feeding() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let p_count = remote.feedings().len();
    remote.add_fake_feedings(5).unwrap();
    assert_eq!(remote.feedings().len(), p_count + 5);
}

#[test]
fn remove_event() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();

    let p_count = remote.events().len();
    let ev = remote.events().pop().unwrap();

    remote.remove_event(ev).unwrap();
    assert_eq!(remote.events().len(), p_count - 1);
}

#[test]
fn remove_expulsion() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();

    let p_count = remote.expulsions().len();
    let ev = remote.expulsions().pop().unwrap();

    remote.remove_expulsion(ev).unwrap();
    assert_eq!(remote.expulsions().len(), p_count - 1);
}

#[test]
fn remove_feeding() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();

    let p_count = remote.feedings().len();
    let ev = remote.feedings().pop().unwrap();

    remote.remove_feeding(ev).unwrap();
    assert_eq!(remote.feedings().len(), p_count - 1);
}

#[test]
fn get_event_by_key() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let ev = remote.events().pop().unwrap();
    let answer = remote.get_event_by_key(&ev.key());
    assert!(answer.is_some());
}

#[test]
fn get_expulsion_by_key() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let ev = remote.expulsions().pop().unwrap();
    let answer = remote.get_expulsion_by_key(&ev.key());
    assert!(answer.is_some());
}

#[test]
fn get_feeding_by_key() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let ev = remote.feedings().pop().unwrap();
    let answer = remote.get_feeding_by_key(&ev.key());
    assert!(answer.is_some());
}

#[test]
fn get_generic_event_by_key() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let key_event = remote.events().pop().unwrap().key();
    let key_expulsion = remote.expulsions().pop().unwrap().key();
    let key_feeding = remote.feedings().pop().unwrap().key();
    assert!(remote.get_base_event_by_key(&key_event).is_some());
    assert!(remote.get_base_event_by_key(&key_expulsion).is_some());
    assert!(remote.get_base_event_by_key(&key_feeding).is_some());
}

#[test]
fn add_event() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let event_added = remote.add_event(&a_person, EventType::Sleep {});
    if event_added.is_err() {
        println!(">>> add_event ERROR RECEIVED :: {:#?}", event_added.err());
        panic!("Shouldn't get an error!")
    } else {
        assert!(event_added.is_ok());
    }
}

#[test]
fn add_expulsion() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let expulsion_added = remote.add_expulsion(&a_person, ExpulsionDegree::Pooplosion {});
    if expulsion_added.is_err() {
        println!(
            ">>> add_expulsion ERROR RECEIVED :: {:#?}",
            expulsion_added.err()
        );
        panic!("Shouldn't get an error!")
    } else {
        assert!(expulsion_added.is_ok());
    }
}

#[test]
fn add_feeding() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let feeding_added = remote.add_feeding(&a_person, 11, 22, 33);
    if feeding_added.is_err() {
        println!(
            ">>> add_feeding ERROR RECEIVED :: {:#?}",
            feeding_added.err()
        );
        panic!("Shouldn't get an error!")
    } else {
        assert!(feeding_added.is_ok());
    }
}

#[test]
fn modify_event() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();
    let _ = remote.purge_all_data();
    let mut an_event = remote.events().pop().unwrap();

    an_event.modify_event(
        an_event.time_stamp().clone(),
        EventType::Note("FOOBAR".to_string()),
    );

    let result = remote.modify_event(&an_event);
    assert!(result.is_ok());
}

#[test]
fn modify_expulsion() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();

    let _ = remote.purge_all_data();
    let mut an_expulsion = remote.expulsions().pop().unwrap();

    an_expulsion.modify_expulsion(ExpulsionDegree::Shart, an_expulsion.time_stamp().clone());

    let result = remote.modify_expulsion(&an_expulsion);
    assert!(result.is_ok());
}

#[test]
fn modify_feeding() {
    let mut remote = construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap();

    let _ = remote.purge_all_data();
    let mut a_feeding = remote.feedings().pop().unwrap();

    a_feeding.modify_feed(
        a_feeding.breast_milk() + 100,
        a_feeding.formula() + 100,
        a_feeding.solids() + 100,
        a_feeding.time_stamp().clone(),
    );

    let result = remote.modify_feeding(&a_feeding);
    assert!(result.is_ok());
}
