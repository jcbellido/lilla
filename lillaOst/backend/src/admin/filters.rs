use tokio::sync::mpsc::Sender;
use warp::{filters::BoxedFilter, Filter, Reply};

use super::handlers;
use crate::command::CommandToBackend;
use crate::common_filters::with_command_sender;

pub fn all_admin(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    reset(tx.clone()).or(purge_all_events(tx)).boxed()
}

pub fn reset(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "admin" / "reset")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and_then(handlers::reset)
        .boxed()
}

pub fn purge_all_events(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "admin" / "purge-all-events")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and_then(handlers::purge_all_events)
        .boxed()
}
