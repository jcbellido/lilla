use tokio::sync::mpsc::Sender;
use warp::{filters::BoxedFilter, Filter, Reply};

use super::handlers;
use crate::command::CommandToBackend;
use crate::command_args::{ArgAModifyPerson, ArgAddPerson, ArgFakeCount};

pub fn all_persons(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    get_persons(tx.clone())
        .or(add_person(tx.clone()))
        .or(modify_person(tx.clone()))
        .or(add_fake_persons(tx))
        .boxed()
}

pub fn get_persons(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "persons")
        .and(warp::get())
        .and(with_command_sender(tx))
        .and_then(handlers::ost_get_persons)
        .boxed()
}

fn with_command_sender(
    tx: Sender<CommandToBackend>,
) -> impl Filter<Extract = (Sender<CommandToBackend>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tx.clone())
}

pub fn add_person(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "persons")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_add_person())
        .and_then(handlers::add_person)
        .boxed()
}

fn json_args_add_person() -> impl Filter<Extract = (ArgAddPerson,), Error = warp::Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn modify_person(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "person")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_modify_person())
        .and_then(handlers::modify_person)
        .boxed()
}

fn json_args_modify_person(
) -> impl Filter<Extract = (ArgAModifyPerson,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn add_fake_persons(tx: Sender<CommandToBackend>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "persons" / "add-fake-count")
        .and(warp::post())
        .and(with_command_sender(tx))
        .and(json_args_fake_count())
        .and_then(handlers::add_fake_persons)
        .boxed()
}

fn json_args_fake_count() -> impl Filter<Extract = (ArgFakeCount,), Error = warp::Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
