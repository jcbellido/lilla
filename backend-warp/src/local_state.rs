use tokio::sync::mpsc::Receiver;

use crate::command::CommandToBackend;

use ost::context::{construct_monolith, construct_monolith_in_memory};
use ost::person::deserialize as person_deserialize;

pub async fn faked_state_ost_context(mut rx: Receiver<CommandToBackend>) {
    let mut ost = construct_monolith_in_memory().unwrap();
    let _ = ost.add_fake_persons(10);
    let _ = ost.add_fake_feedings(150);
    let _ = ost.add_fake_events(150);
    let _ = ost.add_fake_expulsions(150);
    log::info!("[Server] created and fake persons in place");

    while let Some(cmd) = rx.recv().await {
        log::debug!("cmd received{:#?}", cmd);
        match cmd {
            // Admin Calls
            CommandToBackend::AdminReset { resp } => {
                ost.purge_all_data().unwrap();
                let _ = ost.add_fake_persons(10).unwrap();
                let _ = ost.add_fake_feedings(150).unwrap();
                let _ = ost.add_fake_events(150).unwrap();
                let _ = ost.add_fake_expulsions(150).unwrap();
                let _ = resp.send("OK".to_string());
            }
            CommandToBackend::AdminPurgeEvents { resp } => {
                let result = ost.purge_all_events();
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            // Person
            CommandToBackend::GetPersons { resp } => {
                let persons: Vec<String> = ost.persons().iter().map(|p| p.serialize()).collect();
                let serialized_response = serde_json::to_string(&persons).unwrap();
                let _ = resp.send(serialized_response);
            }
            CommandToBackend::AddPerson { args, resp } => {
                let result = ost.add_person(args.name.as_str());
                let payload: Result<String, String> = match result {
                    Ok(o) => Ok(o.serialize()),
                    Err(err) => Err(err),
                };
                let to_wire = serde_json::to_string(&payload).unwrap();
                let _ = resp.send(to_wire);
            }
            CommandToBackend::AddFakePerson { args, resp } => {
                let result = ost.add_fake_persons(args.count);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            CommandToBackend::ModifyPerson { args, resp } => {
                let deserialized_person = person_deserialize(&args.serialized_person).unwrap();
                let result = ost.modify_person(&deserialized_person);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            // Feedings
            CommandToBackend::GetFeedings { resp } => {
                let persons: Vec<String> = ost.feedings().iter().map(|p| p.serialize()).collect();
                let serialized_response = serde_json::to_string(&persons).unwrap();
                let _ = resp.send(serialized_response);
            }
            CommandToBackend::GetFeedingByKey { args, resp } => {
                let found = ost.get_feeding_by_key(&args.event_key);
                match found {
                    Some(o) => {
                        let some_answer: Option<String> = Some(o.serialize());
                        let _ = resp
                            .send(serde_json::to_string(&some_answer).unwrap())
                            .unwrap();
                    }
                    None => {
                        let no_answer: Option<String> = None;
                        let _ = resp
                            .send(serde_json::to_string(&no_answer).unwrap())
                            .unwrap();
                    }
                }
            }
            CommandToBackend::AddFakeFeedings { args, resp } => {
                let result = ost.add_fake_feedings(args.count);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            CommandToBackend::AddFeeding { args, resp } => {
                let message: Result<String, String> = match ost.get_person_by_key(args.person_key) {
                    Some(target_person) => {
                        match ost.add_feeding(
                            &target_person,
                            args.breast_milk,
                            args.formula,
                            args.solids,
                        ) {
                            Ok(new_event) => Ok(new_event.serialize()),
                            Err(e) => Err(e),
                        }
                    }
                    None => Err("Add feeding: person not found".to_string()),
                };
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::ModifyFeeding { args, resp } => {
                let mut message: Result<(), String> = Ok(());
                if let Some(mut target_event) = ost.get_feeding_by_key(&args.event_key) {
                    target_event.modify_feed(
                        args.breast_milk,
                        args.formula,
                        args.solids,
                        args.time_stamp,
                    );

                    match ost.modify_feeding(&target_event) {
                        Ok(_) => {}
                        Err(e) => message = Err(e),
                    }
                } else {
                    message = Err("Event not found".to_string());
                }
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::RemoveFeeding { args, resp } => {
                if let Some(target_feeding) = ost.get_feeding_by_key(&args.event_key) {
                    let _ = resp
                        .send(serde_json::to_string(&ost.remove_feeding(target_feeding)).unwrap())
                        .unwrap();
                } else {
                    let message: Result<String, String> = Err("Feeding not found".to_string());
                    let _ = resp.send(serde_json::to_string(&message).unwrap());
                }
            }
            // Expulsions
            CommandToBackend::GetExpulsions { resp } => {
                let expulsions: Vec<String> =
                    ost.expulsions().iter().map(|p| p.serialize()).collect();
                let serialized_response = serde_json::to_string(&expulsions).unwrap();
                let _ = resp.send(serialized_response);
            }
            CommandToBackend::GetExpulsionByKey { args, resp } => {
                let found = ost.get_expulsion_by_key(&args.event_key);
                match found {
                    Some(o) => {
                        let some_answer: Option<String> = Some(o.serialize());
                        let _ = resp
                            .send(serde_json::to_string(&some_answer).unwrap())
                            .unwrap();
                    }
                    None => {
                        let no_answer: Option<String> = None;
                        let _ = resp
                            .send(serde_json::to_string(&no_answer).unwrap())
                            .unwrap();
                    }
                }
            }
            CommandToBackend::AddFakeExpulsions { args, resp } => {
                let result = ost.add_fake_expulsions(args.count);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            CommandToBackend::AddExpulsion { args, resp } => {
                let message: Result<String, String> = match ost.get_person_by_key(args.person_key) {
                    Some(target_person) => {
                        match ost.add_expulsion(&target_person, args.expulsion_degree) {
                            Ok(new_event) => Ok(new_event.serialize()),
                            Err(e) => Err(e),
                        }
                    }
                    None => Err("Add Expulsion: person not found".to_string()),
                };
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::ModifyExpulsion { args, resp } => {
                let mut message: Result<(), String> = Ok(());
                if let Some(mut target_event) = ost.get_expulsion_by_key(&args.event_key) {
                    target_event.modify_expulsion(args.expulsion_degree, args.time_stamp);
                    match ost.modify_expulsion(&target_event) {
                        Ok(_) => {}
                        Err(e) => message = Err(e),
                    }
                } else {
                    message = Err("Event not found".to_string());
                }
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::RemoveExpulsion { args, resp } => {
                if let Some(target_expulsion) = ost.get_expulsion_by_key(&args.event_key) {
                    let _ = resp
                        .send(
                            serde_json::to_string(&ost.remove_expulsion(target_expulsion)).unwrap(),
                        )
                        .unwrap();
                } else {
                    let message: Result<String, String> = Err("Expulsion not found".to_string());
                    let _ = resp.send(serde_json::to_string(&message).unwrap());
                }
            }
            // Events
            CommandToBackend::GetEvents { resp } => {
                let events: Vec<String> = ost.events().iter().map(|p| p.serialize()).collect();
                let serialized_response = serde_json::to_string(&events).unwrap();
                let _ = resp.send(serialized_response);
            }
            CommandToBackend::GetEventByKey { args, resp } => {
                let found = ost.get_event_by_key(&args.event_key);
                match found {
                    Some(o) => {
                        let some_answer: Option<String> = Some(o.serialize());
                        let _ = resp
                            .send(serde_json::to_string(&some_answer).unwrap())
                            .unwrap();
                    }
                    None => {
                        let no_answer: Option<String> = None;
                        let _ = resp
                            .send(serde_json::to_string(&no_answer).unwrap())
                            .unwrap();
                    }
                }
            }
            CommandToBackend::AddEvent { args, resp } => {
                let message: Result<String, String> = match ost.get_person_by_key(args.person_key) {
                    Some(target_person) => match ost.add_event(&target_person, args.new_event) {
                        Ok(new_event) => Ok(new_event.serialize()),
                        Err(e) => Err(e),
                    },
                    None => Err("Add Event: person not found".to_string()),
                };
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::AddFakeEvents { args, resp } => {
                let result = ost.add_fake_events(args.count);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            CommandToBackend::RemoveEvent { args, resp } => {
                if let Some(target_event) = ost.get_event_by_key(&args.event_key) {
                    let _ = resp
                        .send(serde_json::to_string(&ost.remove_event(target_event)).unwrap())
                        .unwrap();
                } else {
                    let message: Result<String, String> = Err("Event not found".to_string());
                    let _ = resp.send(serde_json::to_string(&message).unwrap());
                }
            }
            CommandToBackend::ModifyEvent { args, resp } => {
                let mut message: Result<(), String> = Ok(());
                if let Some(mut target_event) = ost.get_event_by_key(&args.event_key) {
                    target_event.modify_event(args.time_stamp, args.event_payload);
                    match ost.modify_event(&target_event) {
                        Ok(_) => {}
                        Err(e) => message = Err(e),
                    }
                } else {
                    message = Err("Event not found".to_string());
                }
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
        }
    }
}

pub async fn file_based_ost_context(mut rx: Receiver<CommandToBackend>, file_path: &str) {
    let mut ost = construct_monolith(file_path).unwrap();

    while let Some(cmd) = rx.recv().await {
        log::debug!("cmd received{:#?}", cmd);
        match cmd {
            // Admin Calls
            CommandToBackend::AdminReset { resp } => {
                ost.purge_all_data().unwrap();
                let _ = resp.send("OK".to_string());
            }
            CommandToBackend::AdminPurgeEvents { resp } => {
                let result = ost.purge_all_events();
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            // Person
            CommandToBackend::GetPersons { resp } => {
                let persons: Vec<String> = ost.persons().iter().map(|p| p.serialize()).collect();
                let serialized_response = serde_json::to_string(&persons).unwrap();
                let _ = resp.send(serialized_response);
            }
            CommandToBackend::AddPerson { args, resp } => {
                let result = ost.add_person(args.name.as_str());
                let payload: Result<String, String> = match result {
                    Ok(o) => Ok(o.serialize()),
                    Err(err) => Err(err),
                };
                let to_wire = serde_json::to_string(&payload).unwrap();
                let _ = resp.send(to_wire);
            }
            CommandToBackend::AddFakePerson { args, resp } => {
                let result = ost.add_fake_persons(args.count);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            CommandToBackend::ModifyPerson { args, resp } => {
                let deserialized_person = person_deserialize(&args.serialized_person).unwrap();
                let result = ost.modify_person(&deserialized_person);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            // Feedings
            CommandToBackend::GetFeedings { resp } => {
                let persons: Vec<String> = ost.feedings().iter().map(|p| p.serialize()).collect();
                let serialized_response = serde_json::to_string(&persons).unwrap();
                let _ = resp.send(serialized_response);
            }
            CommandToBackend::GetFeedingByKey { args, resp } => {
                let found = ost.get_feeding_by_key(&args.event_key);
                match found {
                    Some(o) => {
                        let some_answer: Option<String> = Some(o.serialize());
                        let _ = resp
                            .send(serde_json::to_string(&some_answer).unwrap())
                            .unwrap();
                    }
                    None => {
                        let no_answer: Option<String> = None;
                        let _ = resp
                            .send(serde_json::to_string(&no_answer).unwrap())
                            .unwrap();
                    }
                }
            }
            CommandToBackend::AddFakeFeedings { args, resp } => {
                let result = ost.add_fake_feedings(args.count);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            CommandToBackend::AddFeeding { args, resp } => {
                let message: Result<String, String> = match ost.get_person_by_key(args.person_key) {
                    Some(target_person) => {
                        match ost.add_feeding(
                            &target_person,
                            args.breast_milk,
                            args.formula,
                            args.solids,
                        ) {
                            Ok(new_event) => Ok(new_event.serialize()),
                            Err(e) => Err(e),
                        }
                    }
                    None => Err("Add feeding: person not found".to_string()),
                };
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::RemoveFeeding { args, resp } => {
                if let Some(target_feeding) = ost.get_feeding_by_key(&args.event_key) {
                    let _ = resp
                        .send(serde_json::to_string(&ost.remove_feeding(target_feeding)).unwrap())
                        .unwrap();
                } else {
                    let message: Result<String, String> = Err("Feeding not found".to_string());
                    let _ = resp.send(serde_json::to_string(&message).unwrap());
                }
            }
            // Expulsions
            CommandToBackend::GetExpulsions { resp } => {
                let expulsions: Vec<String> =
                    ost.expulsions().iter().map(|p| p.serialize()).collect();
                let serialized_response = serde_json::to_string(&expulsions).unwrap();
                let _ = resp.send(serialized_response);
            }
            CommandToBackend::GetExpulsionByKey { args, resp } => {
                let found = ost.get_expulsion_by_key(&args.event_key);
                match found {
                    Some(o) => {
                        let some_answer: Option<String> = Some(o.serialize());
                        let _ = resp
                            .send(serde_json::to_string(&some_answer).unwrap())
                            .unwrap();
                    }
                    None => {
                        let no_answer: Option<String> = None;
                        let _ = resp
                            .send(serde_json::to_string(&no_answer).unwrap())
                            .unwrap();
                    }
                }
            }
            CommandToBackend::AddFakeExpulsions { args, resp } => {
                let result = ost.add_fake_expulsions(args.count);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            CommandToBackend::AddExpulsion { args, resp } => {
                let message: Result<String, String> = match ost.get_person_by_key(args.person_key) {
                    Some(target_person) => {
                        match ost.add_expulsion(&target_person, args.expulsion_degree) {
                            Ok(new_event) => Ok(new_event.serialize()),
                            Err(e) => Err(e),
                        }
                    }
                    None => Err("Add Expulsion: person not found".to_string()),
                };
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::RemoveExpulsion { args, resp } => {
                if let Some(target_expulsion) = ost.get_expulsion_by_key(&args.event_key) {
                    let _ = resp
                        .send(
                            serde_json::to_string(&ost.remove_expulsion(target_expulsion)).unwrap(),
                        )
                        .unwrap();
                } else {
                    let message: Result<String, String> = Err("Expulsion not found".to_string());
                    let _ = resp.send(serde_json::to_string(&message).unwrap());
                }
            }
            // Events
            CommandToBackend::GetEvents { resp } => {
                let events: Vec<String> = ost.events().iter().map(|p| p.serialize()).collect();
                let serialized_response = serde_json::to_string(&events).unwrap();
                let _ = resp.send(serialized_response);
            }
            CommandToBackend::GetEventByKey { args, resp } => {
                let found = ost.get_event_by_key(&args.event_key);
                match found {
                    Some(o) => {
                        let some_answer: Option<String> = Some(o.serialize());
                        let _ = resp
                            .send(serde_json::to_string(&some_answer).unwrap())
                            .unwrap();
                    }
                    None => {
                        let no_answer: Option<String> = None;
                        let _ = resp
                            .send(serde_json::to_string(&no_answer).unwrap())
                            .unwrap();
                    }
                }
            }
            CommandToBackend::AddEvent { args, resp } => {
                let message: Result<String, String> = match ost.get_person_by_key(args.person_key) {
                    Some(target_person) => match ost.add_event(&target_person, args.new_event) {
                        Ok(new_event) => Ok(new_event.serialize()),
                        Err(e) => Err(e),
                    },
                    None => Err("Add Event: person not found".to_string()),
                };
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::AddFakeEvents { args, resp } => {
                let result = ost.add_fake_events(args.count);
                let _ = resp.send(serde_json::to_string(&result).unwrap());
            }
            CommandToBackend::RemoveEvent { args, resp } => {
                if let Some(target_event) = ost.get_event_by_key(&args.event_key) {
                    let _ = resp
                        .send(serde_json::to_string(&ost.remove_event(target_event)).unwrap())
                        .unwrap();
                } else {
                    let message: Result<String, String> = Err("Event not found".to_string());
                    let _ = resp.send(serde_json::to_string(&message).unwrap());
                }
            }
            CommandToBackend::ModifyEvent { args, resp } => {
                let mut message: Result<(), String> = Ok(());
                if let Some(mut target_event) = ost.get_event_by_key(&args.event_key) {
                    target_event.modify_event(args.time_stamp, args.event_payload);
                    match ost.modify_event(&target_event) {
                        Ok(_) => {}
                        Err(e) => message = Err(e),
                    }
                } else {
                    message = Err("Event not found".to_string());
                }
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::ModifyExpulsion { args, resp } => {
                let mut message: Result<(), String> = Ok(());
                if let Some(mut target_event) = ost.get_expulsion_by_key(&args.event_key) {
                    target_event.modify_expulsion(args.expulsion_degree, args.time_stamp);
                    match ost.modify_expulsion(&target_event) {
                        Ok(_) => {}
                        Err(e) => message = Err(e),
                    }
                } else {
                    message = Err("Event not found".to_string());
                }
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
            CommandToBackend::ModifyFeeding { args, resp } => {
                let mut message: Result<(), String> = Ok(());
                if let Some(mut target_event) = ost.get_feeding_by_key(&args.event_key) {
                    target_event.modify_feed(
                        args.breast_milk,
                        args.formula,
                        args.solids,
                        args.time_stamp,
                    );

                    match ost.modify_feeding(&target_event) {
                        Ok(_) => {}
                        Err(e) => message = Err(e),
                    }
                } else {
                    message = Err("Event not found".to_string());
                }
                let _ = resp.send(serde_json::to_string(&message).unwrap());
            }
        }
    }
}
