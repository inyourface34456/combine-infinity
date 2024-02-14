use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, RwLock};

pub type Outer<T> = Arc<RwLock<T>>;
// pub type Combo = (String, String);

pub fn get_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}