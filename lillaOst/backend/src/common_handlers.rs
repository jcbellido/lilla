use crate::command::CommandToBackend;
use tokio::sync::mpsc::Sender;

use tokio::sync::oneshot;

pub async fn send_command_to_server(
    tx: Sender<CommandToBackend>,
    resp_rx: oneshot::Receiver<String>,
    cmd: CommandToBackend,
) -> String {
    tx.send(cmd).await.unwrap();
    let res = resp_rx.await;
    res.unwrap()
}
