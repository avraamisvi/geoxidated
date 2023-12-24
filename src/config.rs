/*
    Copyright (c)  Abraão Isvi <avraamisvi@users.noreply.github.com>

    Permission is hereby granted, free of charge, to any
    person obtaining a copy of this software and associated
    documentation files (the "Software"), to deal in the
    Software without restriction, including without
    limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software
    is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice
    shall be included in all copies or substantial portions
    of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
    TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
    PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
    SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
    OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
    IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
 */

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
    let content = std::fs::read_to_string("Config.toml")?;
    let configuration: Configuration = toml::from_str(&content).unwrap();

    Ok(configuration)
}