use std::convert::Infallible;

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::command::CommandToBackend;
use crate::common_handlers::send_command_to_server;

pub async fn reset(tx: Sender<CommandToBackend>) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AdminReset { resp: resp_tx };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}

pub async fn purge_all_events(
    tx: Sender<CommandToBackend>,
) -> Result<impl warp::Reply, Infallible> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = CommandToBackend::AdminPurgeEvents { resp: resp_tx };
    Ok(send_command_to_server(tx, resp_rx, cmd).await)
}
