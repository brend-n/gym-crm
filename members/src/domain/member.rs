use serde::Deserialize;

#[derive(Deserialize)]
pub struct Member {
    pub firstname: String,
    pub surname: String,
}