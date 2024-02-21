mod combos;
mod endpoint_funcs;
mod fourm_data;
mod json_body;
mod utils;

use combos::Combos;
use endpoint_funcs::{clean_hit, combine_hit, sse_counter, vote_hit};
use fourm_data::Vote;
use futures_util::StreamExt;
use json_body::json_arb_data;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use utils::{Errors, Outer};
use warp::{any, get, path, post, serve, Filter};

#[tokio::main]
async fn main() {
    let combos = Combos::new();

    // if let Ok(file_data) = fs::metadata("data.json") {
    //     if file_data.is_file() && file_data.len() != 0 {
    //         combos = Combos::load("data.json".into());
    //     } else {
    //         combos = Combos::new();
    //     }
    // } else {
    //     combos = Combos::new();
    // }

    let combos_filter = any().map(move || combos.clone());

    let (tx, _rx1) = broadcast::channel(16);
    let tx_clone = tx.clone();

    let combine = post()
        .and(path("combine"))
        .and(path::end())
        .and(combos_filter.clone())
        .and(json_arb_data())
        .and_then(combine_hit);

    let vote = post()
        .and(path("vote"))
        .and(path::end())
        .map(move || tx_clone.clone())
        .and(combos_filter.clone())
        .and(json_arb_data())
        .and_then(vote_hit);

    let clean = get()
        .and(path("clean"))
        .and(path::end())
        .and(combos_filter.clone())
        .and_then(clean_hit);

    let uptime = warp::path("uptime").and(warp::get()).map(move || {
        let rx2 = tx.subscribe();
        // create server event source
        let stream = BroadcastStream::new(rx2);
        let event_stream = stream.map(move |x| match x {
            Ok(x) => sse_counter(x),
            Err(err) => sse_counter(format!("{}", err.to_string())),
        });
        // reply using server-sent events
        warp::sse::reply(event_stream)
    });

    let route = combine.or(vote).or(clean).or(uptime);

    serve(route).bind(([127, 0, 0, 1], 3030)).await
}
