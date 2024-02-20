use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::Combo;

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

#[derive(Serialize, Deserialize)]
pub struct SaveFormat {
    pub elements: Vec<String>,
    pub combos: Vec<Combo>,
}

impl SaveFormat {
    pub fn new() -> Self {
        Self {elements: vec![], combos: vec![]}
    }

    pub fn set_elements(&mut self, elements: &Vec<String>) {
        self.elements = elements.clone();
    }

    pub fn set_combos(&mut self, combos: &Vec<Combo>) {
        self.combos = combos.clone();
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