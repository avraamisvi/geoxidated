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