use std::sync::Mutex;

use controllers::features_controller::execute_query;
use model::geo_entity::GeoEntityTrait;
use futures::executor;

#[macro_use] extern crate rocket;

use repository::features_repository::FeatureRepository;
use rocket::State;
use services::feature::FeatureService;
use sqlx::postgres::PgPoolOptions;

use std::env;

mod model;
mod services;
mod controllers;
mod repository;

#[post("/execute", data = "<query>", format = "json")]
fn execute(query: String, feature_service: &State<Mutex<FeatureService>>) -> String {

    let result = execute_query(query, feature_service);
    
    match result {
        Some(feature) => feature.to_geo_json(),
        None => "{}".to_string()
    }
}

async fn create_features_service() -> Option<FeatureService> {

    let password = env::var("DB_PASSWORD").unwrap();
    let user = env::var("DB_USER").unwrap();

    let db_url = format!("postgres://{}:{}@localhost:5432/geo", user, password);

    let pool_res = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url).await;

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
    .manage(Mutex::new(feature_service.unwrap()))
    .mount("/", routes![execute])
}