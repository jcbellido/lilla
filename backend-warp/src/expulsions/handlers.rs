use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::command::CommandToBackend;
use crate::command_args::{ArgAddExpulsion, ArgEntityKey, ArgFakeCount, ArgModifyExpulsion};
use crate::common_handlers::send_command_to_server;

pub async fn get_expulsions(tx: Sender<CommandToBackend>) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::GetExpulsions { resp: resp_tx };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn add_fake_expulsions(
    tx: Sender<CommandToBackend>,
    args: ArgFakeCount,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AddFakeExpulsions {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn remove_expulsion(
    tx: Sender<CommandToBackend>,
    args: ArgEntityKey,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::RemoveExpulsion {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn get_expulsion_by_key(
    tx: Sender<CommandToBackend>,
    args: ArgEntityKey,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::GetExpulsionByKey {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn add_expulsion(
    tx: Sender<CommandToBackend>,
    args: ArgAddExpulsion,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AddExpulsion {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn modify_expulsion(
    tx: Sender<CommandToBackend>,
    args: ArgModifyExpulsion,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::ModifyExpulsion {
        resp: resp_tx,
        args,
    };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}
