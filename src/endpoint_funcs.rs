use warp::*;

pub async fn index() -> Result<reply::Html<String>, Rejection> {
    Ok(reply::html("hello world".to_string()))
}