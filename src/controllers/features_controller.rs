
use rocket::State;
use serde_json::Value;
use std::{result, sync::Mutex};

use crate::{services::feature::FeatureService, model::geo_entity::GeoEntity};

pub fn execute_query(query: String, feature_service: &State<Mutex<FeatureService>>) -> Option<GeoEntity> {
    let result = execute_query_internal(&query, feature_service);

    match result {
        Ok(value) => value,
        Err(_) => None
    }
}

fn execute_query_internal(query: &String, feature_service: &State<Mutex<FeatureService>>) -> result::Result<Option<GeoEntity>, serde_json::Error> {
    let value: Value = serde_json::from_str(query)?;

    Ok(parse_query_internal(&value, feature_service))
}

fn parse_query_internal(value: &Value, feature_service: &State<Mutex<FeatureService>>) -> Option<GeoEntity>  {
    let request: String = value["request"].as_str().unwrap().to_string();

    //TODO generate this with macro
    if request == "get_feature_by_id" {
        get_feature_by_id(value, feature_service)
    } else {
        None
    }  
}

fn get_feature_by_id(value: &Value, feature_service: &State<Mutex<FeatureService>>) -> Option<GeoEntity> {
    
    let feature_id: i64 = value["id"].as_i64().unwrap();

    futures::executor::block_on(async {
        feature_service.lock().unwrap().get_single_feature(&feature_id).await
    })    
}