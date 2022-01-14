use tokio::sync::mpsc::Sender;
use warp::{filters::BoxedFilter, Filter, Reply};

use super::handlers;
use crate::command::CommandToBackend;
use crate::command_args::{ArgAModifyFeeding, ArgAddFeeding};
use crate::common_filters::{json_args_entity_key, json_args_fake_count, with_command_sender};

pub fn all_feedings(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    get_feedings(tx.clone())
        .or(add_feeding(tx.clone()))
        .or(add_fake_feedings(tx.clone()))
        .or(remove_feeding(tx.clone()))
        .or(modify_feeding(tx.clone()))
        .or(get_feeding_by_key(tx))
        .boxed()
}

pub fn get_feedings(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "feedings")
        .and(warp::get())
        .and(with_command_sender(tx))
        .and_then(handlers::ost_get_feedings)
        .boxed()
}

pub fn add_feeding(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "feedings" / "add"))
        .and(with_command_sender(tx))
        .and(json_args_add_event())
        .and_then(handlers::add_feeding)
        .boxed()
}

pub fn add_fake_feedings(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "feedings" / "add-fake-count")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_fake_count())
        .and_then(handlers::add_fake_feedings)
        .boxed()
}

pub fn remove_feeding(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "feedings" / "remove")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_entity_key())
        .and_then(handlers::remove_feeding)
        .boxed()
}

pub fn get_feeding_by_key(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "feedings")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_entity_key())
        .and_then(handlers::get_feeding_by_key)
        .boxed()
}

pub fn modify_feeding(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "feed"))
        .and(with_command_sender(tx))
        .and(json_args_modify_feeding())
        .and_then(handlers::modify_feeding)
        .boxed()
}

pub fn json_args_add_event(
) -> impl Filter<Extract = (ArgAddFeeding,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_args_modify_feeding(
) -> impl Filter<Extract = (ArgAModifyFeeding,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
