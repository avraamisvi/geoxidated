use rocket::State;
use sqlx::PgPool;

use crate::{controllers::features_controller::execute_query, model::geo_entity::GeoEntityTrait,
     services::feature::FeatureService, repository::features_repository::FeatureRepository};

//the ideia is to have a filtering query language using json maybe?
#[post("/execute", data = "<query>", format = "json")]
pub fn execute(query: String, pg_pool: &State<PgPool>) -> String {

    let mut feature_service = create_features_service(pg_pool);
    let result = execute_query(&query, &mut feature_service);
    
    match result {
        Some(feature) => feature.to_geo_json(),
        None => "{}".to_string()
    }
}

#[get("/collections?<page>&<size>")]
pub fn get_collections(pg_pool: &State<PgPool>, size: i64, page: i64) -> String {
    "".to_string()
    // let mut feature_service = create_features_service(pg_pool);
    // let result = execute_query(&query, &mut feature_service);
    
    // match result {
    //     Some(feature) => feature.to_geo_json(),
    //     None => "{}".to_string()
    // }
}

#[get("/collections/<min_lng>/<min_lat>/<max_lng>/<max_lat>?<page>&<size>")]
pub fn get_collections_by_bbox(pg_pool: &State<PgPool>, 
    min_lng: f64, 
    min_lat: f64,
    max_lng: f64,
    max_lat: f64,
    size: i64, 
    page: i64) -> String {
    "".to_string()
    // let mut feature_service = create_features_service(pg_pool);
    // let result = execute_query(&query, &mut feature_service);
    
    // match result {
    //     Some(feature) => feature.to_geo_json(),
    //     None => "{}".to_string()
    // }
}

#[get("/collections/<id>/items?<page>&<size>")]
pub fn get_collections_features(pg_pool: &State<PgPool>, id: i64, size: i64, page: i64) -> String {
    "".to_string()
    // let mut feature_service = create_features_service(pg_pool);
    // let result = execute_query(&query, &mut feature_service);
    
    // match result {
    //     Some(feature) => feature.to_geo_json(),
    //     None => "{}".to_string()
    // }
}

#[get("/collections/<collection_id>/items/<feature_id>")]
pub fn get_collections_feature(pg_pool: &State<PgPool>, collection_id: i64, feature_id: i64) -> String {
    "".to_string()
    // let mut feature_service = create_features_service(pg_pool);
    // let result = execute_query(&query, &mut feature_service);
    
    // match result {
    //     Some(feature) => feature.to_geo_json(),
    //     None => "{}".to_string()
    // }
}

#[get("/collections/<id>/items/<min_lng>/<min_lat>/<max_lng>/<max_lat>?<page>&<size>")]
pub fn get_features_by_bbox(pg_pool: &State<PgPool>,
    id: i64, 
    min_lng: f64, 
    min_lat: f64,
    max_lng: f64,
    max_lat: f64,
    size: i64, 
    page: i64) -> String {
    "".to_string()
    // let mut feature_service = create_features_service(pg_pool);
    // let result = execute_query(&query, &mut feature_service);
    
    // match result {
    //     Some(feature) => feature.to_geo_json(),
    //     None => "{}".to_string()
    // }
}


fn create_features_service(pool_state: &State<PgPool>) -> FeatureService {
    let pool = pool_state.inner().clone();
    FeatureService::new(FeatureRepository::new(pool))
}