use serde_json::Value;
use std::result;
use crate::{services::feature::FeatureService, model::geo_entity::{GeoEntity, Feature}};

static GET_FEATURE_BY_ID: &str = "get_feature_by_id";
static SAVE_FEATURE: &str = "save_feature";

static SAVE_COLLECTION: &str = "save_collection";
static GET_COLLECTION_BY_ID: &str = "get_collection_by_id";
static ADD_TO_COLLECTION: &str = "add_to_collection";

pub fn execute_query(query: &String, feature_service: &mut FeatureService) -> Option<GeoEntity> {
    let parsed_query_res = parse_query_internal(query);

    match parsed_query_res {
        Ok(parsed_query) => execute_query_internal(&parsed_query, feature_service),
        Err(err) => None
    }
}

fn parse_query_internal(query: &String) -> result::Result<Value, serde_json::Error> {
    let value: Value = serde_json::from_str(query)?;
    Ok(value)
}

fn execute_query_internal(value: &Value, feature_service: &mut FeatureService) -> Option<GeoEntity>  {
    let request: String = value["request"].as_str().unwrap().to_string();

    //TODO generate this with macro or use other strategy to improve code maintenability
    if request == "get_feature_by_id" {
        get_feature_by_id(value, feature_service)
    } 
    else if request == "save_feature" {
        save_feature(value, feature_service)
    } else {
        None
    }  
}

fn get_feature_by_id(value: &Value, feature_service: &mut FeatureService) -> Option<GeoEntity> {
    
    let feature_id: i64 = value["id"].as_i64().unwrap();

    futures::executor::block_on(async {
        feature_service.get_single_feature(&feature_id).await
    })    
}

fn save_feature(value: &Value, feature_service: &mut FeatureService) -> Option<GeoEntity> {
    
    let feature: Feature = Feature::from(value);

    futures::executor::block_on(async {
        let result = feature_service.save_feature(&feature).await;
        result
    })    
}
