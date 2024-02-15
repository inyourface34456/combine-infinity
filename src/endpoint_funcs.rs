use warp::*;

use crate::combos::Combos;
use crate::fourm_data::Combonation;
use crate::utils::Errors;
use crate::Vote;

pub async fn index() -> Result<reply::Html<String>, Rejection> {
    Ok(reply::html("hello world".to_string()))
}

pub async fn combine_hit(combos: Combos, to_combine: Combonation) -> Result<impl Reply, Rejection> {
    let result = combos.combine(to_combine.to_tuple());

    match result {
        Ok(res) => Ok(reply::html(res)),
        Err(error) => {
            match error {
                Errors::VotingInProgress => Ok(reply::html("voting in progress, vote for what ou ant at /vote".into())),
                Errors::InternelServerError => Err(reject())
            }
        }
    }
}

pub async fn vote_hit(combos: Combos, to_combine: Vote) -> Result<impl Reply, Rejection> {
    let result = combos.vote(to_combine.to_tuple(), to_combine.result());

    Ok(reply::html(result))
}

pub async fn clean_hit(combos: Combos) -> Result<impl Reply, Rejection> {
    combos.clean();

    Ok(reply::with_status("Sucsess", http::StatusCode::OK))
}