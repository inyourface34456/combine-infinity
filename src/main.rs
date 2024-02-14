mod combo;
mod combos;
mod endpoint_funcs;
mod fourm_data;
mod props;
mod utils;

use combo::*;
use combos::Combos;
use endpoint_funcs::*;
use fourm_data::*;
use props::*;
use std::thread;
use utils::*;
use warp::*;
use Filter;

#[tokio::main]
async fn main() {
    let combos = Combos::new();
    let combos_filter = any().map(move || combos.clone());

    thread::spawn(move || loop {
        combos_filter.clone().map(|x: Combos| x.clean());
        thread::sleep(std::time::Duration::from_secs(3600));
    });

    let index = get().and(path::end()).and_then(index);

    let route = index;

    serve(route).run(([127, 0, 0, 1], 3030)).await;
}
