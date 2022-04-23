use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::command::CommandToBackend;
use crate::command_args::{ArgAModifyEvent, ArgAddEvent, ArgEntityKey, ArgFakeCount};
use crate::common_handlers::send_command_to_server;

pub async fn get_events(tx: Sender<CommandToBackend>) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::GetEvents { resp: resp_tx };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn get_event_by_key(
    tx: Sender<CommandToBackend>,
    args: ArgEntityKey,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::GetEventByKey {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn add_fake_events(
    tx: Sender<CommandToBackend>,
    args: ArgFakeCount,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AddFakeEvents {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn remove_event(
    tx: Sender<CommandToBackend>,
    args: ArgEntityKey,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::RemoveEvent {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn add_event(
    tx: Sender<CommandToBackend>,
    args: ArgAddEvent,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AddEvent {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn modify_event(
    tx: Sender<CommandToBackend>,
    args: ArgAModifyEvent,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::ModifyEvent {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}
