use tokio::sync::mpsc::Sender;
use warp::{filters::BoxedFilter, Filter, Reply};

use super::handlers;
use crate::command::CommandToBackend;
use crate::command_args::{ArgAddExpulsion, ArgModifyExpulsion};
use crate::common_filters::{json_args_entity_key, json_args_fake_count, with_command_sender};

pub fn all_expulsions(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    get_expulsions(tx.clone())
        .or(add_expulsion(tx.clone()))
        .or(add_fake_expulsions(tx.clone()))
        .or(remove_expulsion(tx.clone()))
        .or(get_expulsion_by_key(tx.clone()))
        .or(modify_expulsion(tx))
        .boxed()
}

pub fn get_expulsions(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "expulsions")
        .and(warp::get())
        .and(with_command_sender(tx))
        .and_then(handlers::get_expulsions)
        .boxed()
}

pub fn add_expulsion(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "expulsions" / "add"))
        .and(with_command_sender(tx))
        .and(json_args_add_expulsion())
        .and_then(handlers::add_expulsion)
        .boxed()
}

pub fn add_fake_expulsions(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "expulsions" / "add-fake-count")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_fake_count())
        .and_then(handlers::add_fake_expulsions)
        .boxed()
}

pub fn remove_expulsion(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "expulsions" / "remove")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_entity_key())
        .and_then(handlers::remove_expulsion)
        .boxed()
}

pub fn get_expulsion_by_key(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    //  -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "expulsions")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_entity_key())
        .and_then(handlers::get_expulsion_by_key)
        .boxed()
}

pub fn modify_expulsion(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "expulsion"))
        .and(with_command_sender(tx))
        .and(json_args_modify_expulsion())
        .and_then(handlers::modify_expulsion)
        .boxed()
}

pub fn json_args_add_expulsion(
) -> impl Filter<Extract = (ArgAddExpulsion,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_args_modify_expulsion(
) -> impl Filter<Extract = (ArgModifyExpulsion,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
