mod combo;
mod combos;
mod endpoint_funcs;
mod fourm_data;
mod props;
mod utils;
mod json_body;

use combo::*;
use combos::Combos;
use endpoint_funcs::*;
use fourm_data::*;
use props::*;
use utils::*;
use warp::*;
use json_body::*;
use Filter;

#[tokio::main]
async fn main() {
    let combos = Combos::new();
    let combos_filter = any().map(move || combos.clone());

    let index = get().and(path::end()).and_then(index);

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

    serve(route).run(([127, 0, 0, 1], 3030)).await;
}
