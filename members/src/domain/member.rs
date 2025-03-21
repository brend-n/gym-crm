use serde::Deserialize;

#[derive(Deserialize)]
pub struct Member {
    firstname: String,
    surname: String,
}