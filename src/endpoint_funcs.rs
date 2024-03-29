use core::convert::Infallible;
use tokio::sync::broadcast::Sender;
use warp::sse::Event;
use warp::{http, reject, reply, Rejection, Reply};

use crate::combos::Combos;
use crate::fourm_data::Combonation;
use crate::utils::{Errors, NewVote};
use crate::Vote;

pub async fn combine_hit(combos: Combos, to_combine: Combonation) -> Result<impl Reply, Rejection> {
    let combo = to_combine.to_tuple();

    let result = combos.combine((&combo.0.to_string(), &combo.1.to_string()));

    match result {
        Ok(res) => Ok(reply::html(res.clone())),
        Err(error) => match error {
            Errors::VotingInProgress => Ok(reply::html(
                "voting in progress, vote for what you want".into(),
            )),
            _ => Err(reject()),
        },
    }
}

pub async fn vote_hit(
    sender: Sender<String>,
    combos: Combos,
    to_combine: Vote,
) -> Result<impl Reply, Rejection> {
    let tuple = to_combine.to_tuple();
    let result = combos.vote(tuple.clone(), to_combine.result());

    let _ =
        sender.send(serde_json::to_string(&NewVote::new((&tuple.0, &tuple.1), result.1)).unwrap());

    Ok(reply::html(result.0))
}

pub async fn clean_hit(combos: Combos) -> Result<impl Reply, Rejection> {
    combos.clean();

    Ok(reply::with_status("Sucsess", http::StatusCode::OK))
}

pub fn sse_counter(counter: String) -> Result<Event, Infallible> {
    Ok(warp::sse::Event::default().data(counter))
}
