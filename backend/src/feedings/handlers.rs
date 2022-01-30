use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::command::CommandToBackend;
use crate::command_args::{ArgAModifyFeeding, ArgAddFeeding, ArgEntityKey, ArgFakeCount};
use crate::common_handlers::send_command_to_server;

pub async fn ost_get_feedings(
    tx: Sender<CommandToBackend>,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::GetFeedings { resp: resp_tx };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn add_fake_feedings(
    tx: Sender<CommandToBackend>,
    args: ArgFakeCount,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AddFakeFeedings {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn remove_feeding(
    tx: Sender<CommandToBackend>,
    args: ArgEntityKey,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::RemoveFeeding {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn get_feeding_by_key(
    tx: Sender<CommandToBackend>,
    args: ArgEntityKey,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::GetFeedingByKey {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn add_feeding(
    tx: Sender<CommandToBackend>,
    args: ArgAddFeeding,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AddFeeding {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn modify_feeding(
    tx: Sender<CommandToBackend>,
    args: ArgAModifyFeeding,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::ModifyFeeding {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}
