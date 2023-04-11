use serde::{Serialize, Deserialize};
use toml;
use std::{fs, io::{self, Write}};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub day: i32,
}

impl Config {
    pub fn new() -> Self {
        Config { day: 1 }
    }

    pub fn from_file() -> Result<Self, io::Error> {
        let buffer = fs::read("./config.toml")?;
        let contents = String::from_utf8(buffer)
            .expect("Unable to read file");
        let config: Config = toml::from_str(contents.as_str()).unwrap();
        return Ok(config);
    }

    pub fn write(&self, path: String) -> Result<(), io::Error> {
        let mut file = fs::File::create(format!("{}/config.toml", path))?;
        file.write(toml::to_string(self).unwrap().as_bytes())?;
        Ok(())
    }
}
