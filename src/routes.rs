use rocket::{State};
use sqlx::PgPool;
use rocket::response::Responder;

use crate::{services::feature_service::FeatureService, repository::features_repository::FeatureRepository, model::{feature_collection::FeatureCollection, json::Json, feature::Feature, bbox::Bbox}};

#[derive(Responder)]
pub enum CollectionResponse {
    #[response(status = 200, content_type = "json")]
    Ok(String),
    #[response(status = 201, content_type = "json")]
    Created(String),
    #[response(status = 500, content_type = "json")]
    SystemError(String)
}

#[put("/collections", data = "<body>", format = "json")]
pub fn put_collections(body: String, pg_pool: &State<PgPool>) -> CollectionResponse {

    let mut feature_service = create_features_service(pg_pool);
    
    let feature_collection = FeatureCollection::from(Json::new(body));

    let result = futures::executor::block_on(async {
        feature_service.update_collection(&feature_collection).await
    });
    
    match result {
        Ok(collection) => CollectionResponse::Created(collection.to_geo_json()),
        Err(err) => CollectionResponse::SystemError(err.message)
    }
}

#[post("/collections", data = "<body>", format = "json")]
pub fn post_collections(body: String, pg_pool: &State<PgPool>) -> CollectionResponse {

    let mut feature_service = create_features_service(pg_pool);
    
    let feature_collection = FeatureCollection::from(Json::new(body));

    let result = futures::executor::block_on(async {
        feature_service.create_collection(&feature_collection).await
    });
    
    match result {
        Ok(collection) => CollectionResponse::Created(collection.to_geo_json()),
        Err(err) => CollectionResponse::SystemError(err.message)
    }
}

#[post("/collections/<collection_id>/item", data = "<body>", format = "json")]
pub fn post_feature(pg_pool: &State<PgPool>, collection_id: i64, body: String) -> CollectionResponse {

    let mut feature_service = create_features_service(pg_pool);
    
    let feature = Feature::from(Json::new(body));

    let result = futures::executor::block_on(async {
        feature_service.create_feature(collection_id, &feature).await
    });
    
    match result {
        Ok(collection) => CollectionResponse::Created(collection.to_geo_json()),
        Err(err) => CollectionResponse::SystemError(err.message)
    }
}

#[put("/collections/<collection_id>/item", data = "<body>", format = "json")]
pub fn put_feature(pg_pool: &State<PgPool>, collection_id: i64, body: String) -> CollectionResponse {

    let mut feature_service = create_features_service(pg_pool);
    
    let feature = Feature::from(Json::new(body));

    let result = futures::executor::block_on(async {
        feature_service.update_feature(collection_id, &feature).await
    });
    
    match result {
        Ok(collection) => CollectionResponse::Created(collection.to_geo_json()),
        Err(err) => CollectionResponse::SystemError(err.message)
    }
}

#[get("/collections?<page>&<size>")]
pub fn get_collections(pg_pool: &State<PgPool>, size: i64, page: i64) -> CollectionResponse {

    let mut feature_service = create_features_service(pg_pool);
    
    let result = futures::executor::block_on(async{
        feature_service.get_collections(page, size).await
    });
    
    match result {
        Ok(collection) => CollectionResponse::Created(collection.to_json()),
        Err(err) => CollectionResponse::SystemError(err.message)
    }
}

#[get("/collections/<id>/items?<page>&<size>")]
pub fn get_collections_features(pg_pool: &State<PgPool>, id: i64, size: i64, page: i64) -> CollectionResponse {
    
    let mut feature_service = create_features_service(pg_pool);
    
    let result = futures::executor::block_on(async{
        feature_service.get_features_in_collection(id, page, size).await
    });

    match result {
        Ok(collection) => CollectionResponse::Created(collection.to_geo_json()),
        Err(err) => CollectionResponse::SystemError(err.message)
    }
}

#[get("/collections/<id>/items/<min_lng>/<min_lat>/<max_lng>/<max_lat>?<page>&<size>")]
pub fn get_features_by_bbox(pg_pool: &State<PgPool>,
    id: i64, 
    min_lng: f64, 
    min_lat: f64,
    max_lng: f64,
    max_lat: f64,
    size: i64, 
    page: i64) -> CollectionResponse {

    let bbox = Bbox::new(min_lng, min_lat, max_lng, max_lat);
    let mut feature_service = create_features_service(pg_pool);
    
    let result = futures::executor::block_on(async{
        feature_service.get_features_in_collection_by_bbox(id, &bbox, page, size).await
    });

    match result {
        Ok(collection) => CollectionResponse::Created(collection.to_geo_json()),
        Err(err) => CollectionResponse::SystemError(err.message)
    }
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

fn create_features_service(pool_state: &State<PgPool>) -> FeatureService {
    let pool = pool_state.inner().clone();
    FeatureService::new(FeatureRepository::new(pool))
}