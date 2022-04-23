use std::env;
use warp::Filter;

pub fn get_index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let path_index_file =
        env::var("STATIC_FILES_INDEX").expect("Missing env var: `STATIC_FILES_INDEX`");
    warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(path_index_file))
}

pub fn get_static_file() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let path_static_dir =
        env::var("STATIC_FILES_BASE").expect("Missing env var: `STATIC_FILES_BASE`");
    warp::get().and(warp::fs::dir(path_static_dir))
}

pub fn serve_index_by_default_get(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let path_index_file =
        env::var("STATIC_FILES_INDEX").expect("Missing env var: `STATIC_FILES_INDEX`");
    warp::get().and(warp::fs::file(path_index_file))
}
