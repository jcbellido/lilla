use tokio::sync::mpsc::Sender;
use warp::{filters::BoxedFilter, Filter, Reply};

use super::handlers;
use crate::command::CommandToBackend;
use crate::command_args::{ArgAModifyEvent, ArgAddEvent};
use crate::common_filters::{json_args_entity_key, json_args_fake_count, with_command_sender};

pub fn all_events(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    get_events(tx.clone())
        .or(get_event_by_key(tx.clone()))
        .or(add_event(tx.clone()))
        .or(add_fake_events(tx.clone()))
        .or(remove_event(tx.clone()))
        .or(modify_event(tx))
        .boxed()
}

pub fn get_events(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("api" / "events"))
        .and(with_command_sender(tx))
        .and_then(handlers::get_events)
        .boxed()
}

pub fn get_event_by_key(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "events"))
        .and(with_command_sender(tx))
        .and(json_args_entity_key())
        .and_then(handlers::get_event_by_key)
        .boxed()
}

pub fn add_event(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "events" / "add"))
        .and(with_command_sender(tx))
        .and(json_args_add_event())
        .and_then(handlers::add_event)
        .boxed()
}

pub fn add_fake_events(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "events" / "add-fake-count"))
        .and(with_command_sender(tx))
        .and(json_args_fake_count())
        .and_then(handlers::add_fake_events)
        .boxed()
}

pub fn remove_event(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "events" / "remove"))
        .and(with_command_sender(tx))
        .and(json_args_entity_key())
        .and_then(handlers::remove_event)
        .boxed()
}

pub fn modify_event(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "event")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_modify_event())
        .and_then(handlers::modify_event)
        .boxed()
}

pub fn json_args_add_event(
) -> impl Filter<Extract = (ArgAddEvent,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_args_modify_event(
) -> impl Filter<Extract = (ArgAModifyEvent,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
