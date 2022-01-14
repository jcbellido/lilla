use tokio::sync::mpsc::Sender;
use warp::Filter;

use crate::command::CommandToBackend;
use crate::command_args::{ArgEntityKey, ArgFakeCount};
pub fn with_command_sender(
    tx: Sender<CommandToBackend>,
) -> impl Filter<Extract = (Sender<CommandToBackend>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tx.clone())
}

pub fn json_args_fake_count(
) -> impl Filter<Extract = (ArgFakeCount,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_args_entity_key(
) -> impl Filter<Extract = (ArgEntityKey,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
