use tokio::sync::oneshot;

use crate::command_args::*;

type Responder<T> = oneshot::Sender<T>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum CommandToBackend {
    // ---- Persons
    GetPersons {
        resp: Responder<String>,
    },
    AddPerson {
        args: ArgAddPerson,
        resp: Responder<String>,
    },
    AddFakePerson {
        args: ArgFakeCount,
        resp: Responder<String>,
    },
    ModifyPerson {
        args: ArgAModifyPerson,
        resp: Responder<String>,
    },
    // Feedings
    GetFeedings {
        resp: Responder<String>,
    },
    GetFeedingByKey {
        args: ArgEntityKey,
        resp: Responder<String>,
    },
    AddFakeFeedings {
        args: ArgFakeCount,
        resp: Responder<String>,
    },
    AddFeeding {
        args: ArgAddFeeding,
        resp: Responder<String>,
    },
    ModifyFeeding {
        args: ArgAModifyFeeding,
        resp: Responder<String>,
    },
    RemoveFeeding {
        args: ArgEntityKey,
        resp: Responder<String>,
    },
    // Expulsions Section
    GetExpulsions {
        resp: Responder<String>,
    },
    AddExpulsion {
        args: ArgAddExpulsion,
        resp: Responder<String>,
    },
    ModifyExpulsion {
        args: ArgModifyExpulsion,
        resp: Responder<String>,
    },
    AddFakeExpulsions {
        args: ArgFakeCount,
        resp: Responder<String>,
    },
    RemoveExpulsion {
        args: ArgEntityKey,
        resp: Responder<String>,
    },
    GetExpulsionByKey {
        args: ArgEntityKey,
        resp: Responder<String>,
    },
    // Events section
    GetEvents {
        resp: Responder<String>,
    },
    GetEventByKey {
        args: ArgEntityKey,
        resp: Responder<String>,
    },
    AddEvent {
        args: ArgAddEvent,
        resp: Responder<String>,
    },
    AddFakeEvents {
        args: ArgFakeCount,
        resp: Responder<String>,
    },
    RemoveEvent {
        args: ArgEntityKey,
        resp: Responder<String>,
    },
    ModifyEvent {
        args: ArgAModifyEvent,
        resp: Responder<String>,
    },
    // Admin section
    AdminReset {
        resp: Responder<String>,
    },
    AdminPurgeEvents {
        resp: Responder<String>,
    },
}
