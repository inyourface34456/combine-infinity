use serde::Deserialize;

#[derive(Deserialize)]
struct Combonation {
    left: String,
    right: String
}

#[derive(Deserialize)]
struct Vote {
    left: String,
    right: String,
    result: String
}
