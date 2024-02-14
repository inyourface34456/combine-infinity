mod endpoint_funcs;
mod combos;
mod props;
mod utils;
mod combo;

use endpoint_funcs::*;
use combos::Combos;
use props::*;
use combo::*;
use utils::*;
use warp::*;

#[tokio::main]
async fn main() {
    let index = get()
    .and(path::end())
    .and_then(index);

    let route = index;

    serve(route).run(([127, 0, 0, 1], 3030)).await;
}
