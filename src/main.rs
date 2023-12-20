#[macro_use] extern crate rocket;

use config::read_config;
use controllers::features_controller::execute_query;
use model::geo_entity::GeoEntityTrait;
use futures::executor;

use repository::features_repository::FeatureRepository;
use rocket::State;
use services::feature::FeatureService;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error, PgPool};

mod model;
mod services;
mod controllers;
mod repository;
mod config;
mod parser;
mod routes;



async fn create_pool() -> Result<Pool<Postgres>, Error> {
    let configuration = read_config().unwrap();

    PgPoolOptions::new()
    .max_connections(5)
    .connect(&configuration.get_database_url()).await
}

#[launch]
fn rocket() -> _ {

    let pool: Pool<Postgres> = executor::block_on(async {
        create_pool().await
    }).unwrap();

    rocket::build()
    .manage(pool)
    .mount("/", routes![execute])
}