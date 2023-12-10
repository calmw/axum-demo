use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub db_pwd: String,
    pub db_user: String,
    pub database: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DbConfig,
}

// #[warn(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

pub fn read_config() -> Config {
    let file_path = "./app.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    let config: Config = toml::from_str(&str_val).unwrap();
    config
}