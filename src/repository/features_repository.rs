use std::collections;

use derive_new::new;
use crate::{model::{geo_entity::{GeoEntity, Feature, FeatureCollection}, geometry::GeometryTrait, value::ValueTrait}, parser::{ParseError, parse_feature}};

static GEOXIDATED_SCHEMA: &str = "geoxidated";
static FEATURE_TABLE: &str = "feature";
static COLLECTION_TABLE: &str = "feature_collection";

#[derive(new)]
pub struct FeatureRepository {
    pool: sqlx::postgres::PgPool
}

impl FeatureRepository {
   
    pub async fn get_feature_by_id(&mut self, id: &i64) -> Option<GeoEntity> {
        
        let db = &self.pool;

        let query = format!("SELECT id, properties::text, ST_AsGeoJSON(geometry)
         FROM {GEOXIDATED_SCHEMA}.{FEATURE_TABLE} WHERE id = {id}");

        print!("{}", query);

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        let parsed_feature_result = match result {
            Ok(row) => parse_feature(row),
            Err(err) => Err(ParseError{message: err.to_string()})
        };

        match parsed_feature_result {
            Ok(feature) => Some(GeoEntity::from(feature)),
            Err(_) => None
        }
    }

    pub async fn save_feature(&mut self, feature: &Feature) -> Option<GeoEntity> {
        
        let db = &self.pool;

        let query = format!("INSERT INTO {GEOXIDATED_SCHEMA}.{FEATURE_TABLE}(properties, geometry)
         VALUES('{}'::json, ST_GeomFromGeoJSON('{}')) RETURNING id, properties::text, ST_AsGeoJSON(geometry)",
         feature.properties.to_geo_json(),
         feature.geometry.to_geo_json());

        println!("{}", query);

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        let parsed_feature_result = match result {
            Ok(row) => parse_feature(row),
            Err(err) => Err(ParseError{message: err.to_string()})
        };

        match parsed_feature_result {
            Ok(feature) => Some(GeoEntity::from(feature)),
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

     pub async fn save_feature_collection(&mut self, label: &String) -> Option<GeoEntity> {
        
        let db = &self.pool;

        let query = format!("INSERT INTO {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE}(label)
         VALUES('{label}')) RETURNING id, label");

        println!("{}", query);

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        let parsed_feature_result = match result {
            Ok(row) => parse_feature(row),
            Err(err) => Err(ParseError{message: err.to_string()})
        };

        match parsed_feature_result {
            Ok(feature) => Some(GeoEntity::from(feature)),
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }    
}

