use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    id: u64,
    title: String,
    background: String,
    icon: String,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            id: 999,
            title: String::from("Game Game"),
            background: String::from("https://wallpaperaccess.com/full/7445.jpg"),
            icon: String::from("https://png.pngtree.com/png-vector/20191028/ourmid/pngtree-game-control-line-icon-vector-png-image_1904129.jpg")
        }
    }
}
