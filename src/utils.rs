use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

pub const VOTE_EXPIRE: u64 = 1;
pub type Outer<T> = Arc<RwLock<T>>;

#[derive(Debug)]
pub enum Errors {
    InternelServerError,
    VotingInProgress,
    ElementDoesNotExist,
    NameTooLarge,
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Errors::InternelServerError => write!(f, "500"),
            Errors::VotingInProgress => write!(f, "vote now"),
            Errors::ElementDoesNotExist => write!(f, "element does not exsist"),
            Errors::NameTooLarge => write!(f, "name to large"),
        }
    }
}

pub fn get_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn to_outer<T>(data: T) -> Arc<RwLock<T>> {
    Arc::new(RwLock::new(data))
}

#[derive(Serialize)]
pub struct NewVote {
    combo: (String, String),
    votes: HashMap<String, u32>,
}

impl NewVote {
    pub fn new(combo: (&str, &str), votes: HashMap<String, u32>) -> Self {
        let combo = (combo.0.into(), combo.1.into());

        Self { combo, votes }
    }
}
