use std::panic;
use std::thread;
use std::thread::JoinHandle;

use ost::context::{construct_monolith_remote, Context};

use ost::event::EventType;
use ost::expulsion::ExpulsionDegree;

use ost::communications::{get_string_from_network, post_string_to_network, post_to_network};

use backend::servers::faked_ost_api::faked_ost_api;

static REMOTE_ENDPOINT: &str = "http://127.0.0.1:3030";

#[tokio::main]
async fn inner_server() {
    faked_ost_api().await;
}

fn setup() -> JoinHandle<()> {
    thread::spawn(inner_server)
}

struct TestRegistry {
    pub execute: fn(),
    pub description: &'static str,
}

fn new_test(execute: fn(), description: &'static str) -> TestRegistry {
    TestRegistry {
        execute,
        description,
    }
}

fn main() {
    println!("Starting `custom_context_remote`");

    let all_test: Vec<TestRegistry> = vec![
        new_test(can_list_persons, "can_list_persons"),
        new_test(can_add_a_person, "can_add_a_person"),
        new_test(can_list_feedings, "can_list_feedings"),
        new_test(can_modify_person, "can_modify_person"),
        new_test(get_feedigns_by, "get_feedigns_by"),
        new_test(get_expulsions, "get_expulsions"),
        new_test(get_expulsions_by, "get_expulsions_by"),
        new_test(get_events, "get_events"),
        new_test(get_events_by, "get_events_by"),
        new_test(purge_all_events, "purge_all_events"),
        new_test(add_fake_person, "add_fake_person"),
        new_test(add_fake_event, "add_fake_event"),
        new_test(add_fake_expulsion, "add_fake_expulsion"),
        new_test(add_fake_feeding, "add_fake_feeding"),
        new_test(remove_event, "remove_event"),
        new_test(remove_expulsion, "remove_expulsion"),
        new_test(remove_feeding, "remove_feeding"),
        new_test(get_event_by_key, "get_event_by_key"),
        new_test(get_expulsion_by_key, "get_expulsion_by_key"),
        new_test(get_feeding_by_key, "get_feeding_by_key"),
        new_test(get_generic_event_by_key, "get_generic_event_by_key"),
        new_test(add_event, "add_event"),
        new_test(add_expulsion, "add_expulsion"),
        new_test(add_feeding, "add_feeding"),
        new_test(modify_event, "modify_event"),
        new_test(modify_expulsion, "modify_expulsion"),
        new_test(modify_feeding, "modify_feeding"),
    ];

    let _server_join_handle = setup();

    // This is what should pass as a `test runner`
    all_test.iter().for_each(|tr| {
        println!("Executing `{}`", tr.description);
        let r = panic::catch_unwind(tr.execute);
        match r {
            Ok(_) => println!(".. ok"),
            Err(_) => println!(".. failed!"),
        }
    });
}

fn remote_monolith() -> Box<dyn Context> {
    construct_monolith_remote(
        REMOTE_ENDPOINT,
        get_string_from_network,
        post_string_to_network,
        post_to_network,
    )
    .unwrap()
}

// We know the remote server contains 10 persons
fn can_list_persons() {
    let mut remote = remote_monolith();
    let r = remote.purge_all_data();
    assert!(r.is_ok());
    assert_eq!(remote.persons().len(), 10);
}

// We know the remote server contains 10 persons
fn can_add_a_person() {
    let mut remote = remote_monolith();
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
fn can_list_feedings() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    assert_eq!(remote.feedings().len(), 150);
}

fn can_modify_person() {
    let mut remote = remote_monolith();
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

fn get_feedigns_by() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let feedings = remote.feedings_by(&a_person);
    assert!(!feedings.is_empty());
}

fn get_expulsions() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let expulsions = remote.expulsions();
    assert!(!expulsions.is_empty());
}

fn get_expulsions_by() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let expulsions = remote.expulsions_by(&a_person);
    assert!(!expulsions.is_empty());
}

fn get_events() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let expulsions = remote.events();
    assert!(!expulsions.is_empty());
}

fn get_events_by() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let a_person = remote.persons().pop().unwrap();
    let events = remote.events_by(&a_person);
    assert!(!events.is_empty());
}

fn purge_all_events() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    assert_eq!(remote.events().len(), 150);
    let _ = remote.purge_all_events();
    assert_eq!(remote.events().len(), 0);
}

fn add_fake_person() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let p_count = remote.persons().len();
    remote.add_fake_persons(5).unwrap();
    assert_eq!(remote.persons().len(), p_count + 5);
}

fn add_fake_event() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let p_count = remote.events().len();
    remote.add_fake_events(5).unwrap();
    assert_eq!(remote.events().len(), p_count + 5);
}

fn add_fake_expulsion() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let p_count = remote.expulsions().len();
    remote.add_fake_expulsions(5).unwrap();
    assert_eq!(remote.expulsions().len(), p_count + 5);
}

fn add_fake_feeding() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let p_count = remote.feedings().len();
    remote.add_fake_feedings(5).unwrap();
    assert_eq!(remote.feedings().len(), p_count + 5);
}

fn remove_event() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();

    let p_count = remote.events().len();
    let ev = remote.events().pop().unwrap();

    remote.remove_event(ev).unwrap();
    assert_eq!(remote.events().len(), p_count - 1);
}

fn remove_expulsion() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();

    let p_count = remote.expulsions().len();
    let ev = remote.expulsions().pop().unwrap();

    remote.remove_expulsion(ev).unwrap();
    assert_eq!(remote.expulsions().len(), p_count - 1);
}

fn remove_feeding() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();

    let p_count = remote.feedings().len();
    let ev = remote.feedings().pop().unwrap();

    remote.remove_feeding(ev).unwrap();
    assert_eq!(remote.feedings().len(), p_count - 1);
}

fn get_event_by_key() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let ev = remote.events().pop().unwrap();
    let answer = remote.get_event_by_key(&ev.key());
    assert!(answer.is_some());
}

fn get_expulsion_by_key() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let ev = remote.expulsions().pop().unwrap();
    let answer = remote.get_expulsion_by_key(&ev.key());
    assert!(answer.is_some());
}

fn get_feeding_by_key() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let ev = remote.feedings().pop().unwrap();
    let answer = remote.get_feeding_by_key(&ev.key());
    assert!(answer.is_some());
}

fn get_generic_event_by_key() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let key_event = remote.events().pop().unwrap().key();
    let key_expulsion = remote.expulsions().pop().unwrap().key();
    let key_feeding = remote.feedings().pop().unwrap().key();
    assert!(remote.get_base_event_by_key(&key_event).is_some());
    assert!(remote.get_base_event_by_key(&key_expulsion).is_some());
    assert!(remote.get_base_event_by_key(&key_feeding).is_some());
}

fn add_event() {
    let mut remote = remote_monolith();
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

fn add_expulsion() {
    let mut remote = remote_monolith();
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

fn add_feeding() {
    let mut remote = remote_monolith();
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

fn modify_event() {
    let mut remote = remote_monolith();
    let _ = remote.purge_all_data();
    let mut an_event = remote.events().pop().unwrap();

    an_event.modify_event(
        an_event.time_stamp().clone(),
        EventType::Note("FOOBAR".to_string()),
    );

    let result = remote.modify_event(&an_event);
    assert!(result.is_ok());
}

fn modify_expulsion() {
    let mut remote = remote_monolith();

    let _ = remote.purge_all_data();
    let mut an_expulsion = remote.expulsions().pop().unwrap();

    an_expulsion.modify_expulsion(ExpulsionDegree::Shart, an_expulsion.time_stamp().clone());

    let result = remote.modify_expulsion(&an_expulsion);
    assert!(result.is_ok());
}

fn modify_feeding() {
    let mut remote = remote_monolith();

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
