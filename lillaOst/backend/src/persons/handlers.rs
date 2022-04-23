use std::convert::Infallible;

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::command::CommandToBackend;
use crate::command_args::{ArgAModifyPerson, ArgAddPerson, ArgFakeCount};
use crate::common_handlers::send_command_to_server;

pub async fn ost_get_persons(tx: Sender<CommandToBackend>) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::GetPersons { resp: resp_tx };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn add_person(
    tx: Sender<CommandToBackend>,
    args: ArgAddPerson,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AddPerson {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn modify_person(
    tx: Sender<CommandToBackend>,
    args: ArgAModifyPerson,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::ModifyPerson {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn add_fake_persons(
    tx: Sender<CommandToBackend>,
    args: ArgFakeCount,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AddFakePerson {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}
