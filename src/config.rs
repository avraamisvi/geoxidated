
use std::env;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    database: Database
}

#[derive(Deserialize)]
struct Database {
    db_password: String,
    db_user: String,
}

impl Configuration {
    pub fn get_database_url(&self) -> String {
        self.database.get_url()
    }
}

impl Database {
    pub fn get_url(&self) -> String {
        let password = env::var("DB_PASSWORD").unwrap_or(self.db_password.clone());
        let user = env::var("DB_USER").unwrap_or(self.db_user.clone());

        format!("postgres://{}:{}@localhost:5432/geo", user, password)
    }
}


pub fn read_config() -> std::io::Result<Configuration> {
    let content = std::fs::read_to_string("configuration.toml")?;
    let configuration: Configuration = toml::from_str(&content).unwrap();

    Ok(configuration)
    // todo!()
}