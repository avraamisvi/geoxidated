use std::sync::Mutex;

use config::read_config;
use controllers::features_controller::execute_query;
use model::geo_entity::GeoEntityTrait;
use futures::executor;

#[macro_use] extern crate rocket;

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

#[post("/execute", data = "<query>", format = "json")]
fn execute(query: String, pg_pool: &State<PgPool>) -> String {

    let mut feature_service = create_features_service(pg_pool);
    let result = execute_query(&query, &mut feature_service);
    
    match result {
        Some(feature) => feature.to_geo_json(),
        None => "{}".to_string()
    }
}

fn create_features_service(pool_state: &State<PgPool>) -> FeatureService {
    let pool = pool_state.inner().clone();
    FeatureService::new(FeatureRepository::new(pool))
}

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