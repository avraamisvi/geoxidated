use crate::model::{query::{Query, QueryTrait, InvalidQuery}, geo_entity::GeoEntity, get_single::GetSingle};
use serde_json::{Result, Value};
use std::str::FromStr;

pub fn execute_query(query: String) -> GeoEntity {
    parse_query(query).execute()
}

pub fn parse_query(query: String) -> Query {
    let value = serde_json::from_str(&query);

    match value {
        Ok(_) => parse_query_internal(value.unwrap()),
        Err(err) => unimplemented!()
    }

}

fn parse_query_internal(value: Value) -> Query  {
    let request: String = value["request"].as_str().unwrap().to_string();
    let feature_id: i64 = value["id"].as_i64().unwrap();

    //TODO generate this with macro
    if request == "get_feature_by_id" {
        Query::from(GetSingle::new(feature_id))
    } else {
        Query::from(InvalidQuery{})
    }  
}