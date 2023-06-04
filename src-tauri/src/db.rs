use std::{
    fs::{self, write, File},
    path::{Path, PathBuf},
};

use crate::model::Game;

pub fn prepare_db(folder_path: &PathBuf) -> Result<PathBuf, String> {
    let db_path = PathBuf::from(folder_path.to_string_lossy().to_string() + "/db.json");

    if Path::new(&db_path).exists() {
        return Ok(db_path);
    };
    File::create(&db_path).map_err(|x| x.to_string())?;
    write(&db_path, "[]").map_err(|err| err.to_string())?;

    Ok(db_path)
}

pub fn get_all(db_path: &PathBuf) -> Result<Vec<Game>, String> {
    let db_content = fs::read_to_string(db_path).map_err(|err| err.to_string())?;

    let json: Vec<Game> =
        serde_json::from_str(&db_content.as_str()).map_err(|err| err.to_string())?;
    Ok(json)
}

pub fn save_new_data(db_path: &PathBuf, data: Vec<Game>) -> Result<(), String> {
    let new_data_string = serde_json::to_string(&data).map_err(|err| err.to_string())?;
    fs::write(db_path, new_data_string).map_err(|err| err.to_string())?;
    Ok(())
}
