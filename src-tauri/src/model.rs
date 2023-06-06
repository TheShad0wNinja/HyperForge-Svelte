use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    id: u64,
    title: String,
    background: String,
    icon: String,
    path: String,
}
