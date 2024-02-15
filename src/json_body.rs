use serde::Deserialize;
use warp::{Filter, Rejection, body};

pub fn json_arb_data<T: std::marker::Send + for<'de> Deserialize<'de>>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}
