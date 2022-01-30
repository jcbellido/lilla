use std::convert::Infallible;
use warp::http::StatusCode;

#[allow(dead_code)]
pub async fn accept_always() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::ACCEPTED)
}

#[allow(dead_code)]
pub async fn ok_always() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}
