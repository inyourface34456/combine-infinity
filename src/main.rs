mod combo;
mod combos;
mod endpoint_funcs;
mod fourm_data;
mod json_body;
mod props;
mod utils;

use std::fs;

use combo::Combo;
use combos::Combos;
use endpoint_funcs::{clean_hit, combine_hit, index, vote_hit};
use fourm_data::Vote;
use json_body::json_arb_data;
use props::Proposal;
use utils::{Errors, Outer, VOTE_EXPIRE, get_unix_epoch};
use warp::{Filter, any, get, path, post, serve};

#[tokio::main]
async fn main() {
    let combos: Combos;

    if let Ok(file_data) = fs::metadata("data.json") {
        if file_data.is_file() && file_data.len() != 0 {
            combos = Combos::load("data.json".into());
        } else {
            combos = Combos::new();
        }
    } else {
        combos = Combos::new();
    }

    let combos_filter = any().map(move || combos.clone());

    let index = path!("main" / String).map(|path| index(path));

    let combine = post()
        .and(path("combine"))
        .and(path::end())
        .and(combos_filter.clone())
        .and(json_arb_data())
        .and_then(combine_hit);

    let vote = post()
        .and(path("vote"))
        .and(path::end())
        .and(combos_filter.clone())
        .and(json_arb_data())
        .and_then(vote_hit);

    let clean = get()
        .and(path("clean"))
        .and(path::end())
        .and(combos_filter.clone())
        .and_then(clean_hit);

    let route = index.or(combine).or(vote).or(clean);

    serve(route).bind(([127, 0, 0, 1], 3030)).await

}
