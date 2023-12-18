use controllers::features_controller::execute_query;
use model::geo_entity::{GeoEntity, GeoEntityTrait};
use futures::executor;

#[macro_use] extern crate rocket;

use repository::features_repository::{FeatureRepository};
use rocket::futures::TryFutureExt;
use services::feature::FeatureService;
use sqlx::postgres::PgPoolOptions;

mod model;
mod services;
mod controllers;
mod repository;

#[post("/execute", data = "<query>", format = "json")]
fn execute(query: String) -> String {
    let result = execute_query(query);
    result.to_geo_json()
}

async fn create_features_service() -> Option<FeatureService> {
    let pool_res = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:password@localhost/test").await;

    match pool_res {
        Ok(pool) => Some(FeatureService::new(FeatureRepository::new(pool))),
        Err(_) => None,
    }
}

#[launch]
fn rocket() -> _ {
    
    let feature_service: Option<FeatureService> = executor::block_on(async {
        create_features_service().await
    });

    rocket::build()
    .manage(feature_service)
    .mount("/", routes![execute])
}