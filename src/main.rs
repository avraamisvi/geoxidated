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

#[macro_use] extern crate rocket;

use config::read_config;
use futures::executor;
use routes::{post_collections, get_collections, get_collections_feature,
    get_collections_features, post_feature, put_collections,
    get_features_by_bbox, put_feature};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error};

mod data;
mod model;
mod services;
mod repository;
mod config;
mod routes;

#[launch]
fn rocket() -> _ {

    let pool: Pool<Postgres> = executor::block_on(async {
        create_pool().await
    }).unwrap();

    rocket::build()
    .manage(pool)
    .mount("/", routes![post_collections, 
                        post_feature,
                        put_collections,
                        put_feature,
                        get_features_by_bbox,
                        get_collections, 
                        get_collections_features])
}

async fn create_pool() -> Result<Pool<Postgres>, Error> {
    let configuration = read_config().unwrap();

    PgPoolOptions::new()
    .max_connections(5)
    .connect(&configuration.get_database_url()).await
}