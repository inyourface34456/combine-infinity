use warp::{Rejection, Reply, http, reject, reply};
use std::fs;

use crate::combos::Combos;
use crate::fourm_data::Combonation;
use crate::utils::Errors;
use crate::Vote;

pub fn index(path: String) -> impl Reply {
    let mut path = format!("static/{}", path);
    
    if path == "" {
        path = "static/index.html".into();
    } else if path == "index.html" {
        path = "static/index.html".into();
    } else {
        match fs::metadata(&path) {
            Ok(dat) => {
                if dat.is_dir() {
                    path = format!("static/{}/index.html", &path);
                }
            },
            Err(_) => return reply::html("404 not found".into())
        }
    }

    match fs::read_to_string(&path) {
        Ok(dat) => {
            if path.ends_with(".html") {
                reply::html(dat)
            } else {
                reply::html(format!("<code>{}</code>", dat))
            }
        },
        Err(err) => {
            println!("cannot find {}", path);
            reply::html(err.to_string())
        },
    }
}

pub async fn combine_hit(combos: Combos, to_combine: Combonation) -> Result<impl Reply, Rejection> {
    let result = combos.combine(to_combine.to_tuple());

    match result {
        Ok(res) => Ok(reply::html(res)),
        Err(error) => match error {
            Errors::VotingInProgress => Ok(reply::html(
                "voting in progress, vote for what you ant at /vote".into(),
            )),
            _ => Err(reject()),
        },
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
