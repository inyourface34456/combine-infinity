use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum Errors {
    InternelServerError,
    ProposalExsits,
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Errors::InternelServerError => write!(f, "500"),
            Errors::ProposalExsits => write!(f, "vote now"),
        }
    }
}

pub type Outer<T> = Arc<RwLock<T>>;
// pub type Combo = (String, String);

pub fn get_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
