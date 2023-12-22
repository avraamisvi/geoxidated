use std::{fmt::Display, error::Error};

use derive_new::new;

use crate::model::{feature::Feature, feature_collection::FeatureCollection};

static GEOXIDATED_SCHEMA: &str = "geoxidated";
static FEATURE_TABLE: &str = "feature";
static COLLECTION_TABLE: &str = "features_collection";
static FEATURES_IN_COLLECTION: &str = "features_in_collection";
// static HEXAGON_TABLE: &str = "hexagon";

#[derive(Debug)]
pub struct FeatureRepositoryError {
    pub message: String
}

impl Display for FeatureRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error when parsing {}", self.message)
    }
}

impl Error for FeatureRepositoryError {}



#[derive(new)]
pub struct FeatureRepository {
    pool: sqlx::postgres::PgPool
}

impl FeatureRepository {
   
    pub async fn get_features_in_collection(&mut self, id: i64, offset: i64, size: i64) -> Result<Vec<Feature>, FeatureRepositoryError> {
        
        let db = &self.pool;

        let query = format!("SELECT id, \
                                     properties::text,\
                                     ST_AsGeoJSON(geometry) \
                                     FROM {GEOXIDATED_SCHEMA}.{FEATURE_TABLE} fa \
                                     INNER JOIN {GEOXIDATED_SCHEMA}.{FEATURES_IN_COLLECTION} fi \
                                     ON fi.feature_id = fa.id AND fi.collection_id = {id}");

        print!("{}", query);

        let result = sqlx::query(&query)
        .fetch_all(db).await;

        match result {
            Ok(rows) => {
                let collections: Vec<Feature> = rows.iter().map(|row| {
                    Feature::from(row)
                }).collect();

                Ok(collections)
            },
            Err(err) => {
                println!("DB Error {}", err.to_string());
                Err(FeatureRepositoryError{message: err.to_string()})
            }
        }
    }

    // pub async fn save_feature(&mut self, feature: &Feature) -> Option<GeoEntity> {
        
    //     let db = &self.pool;

    //     let query = format!("INSERT INTO {GEOXIDATED_SCHEMA}.{FEATURE_TABLE}(properties, geometry)
    //      VALUES('{}'::json, ST_GeomFromGeoJSON('{}')) RETURNING id, properties::text, ST_AsGeoJSON(geometry)",
    //      feature.properties.to_geo_json(),
    //      feature.geometry.to_geo_json());

    //     println!("{}", query);

    //     let result = sqlx::query(&query)
    //     .fetch_one(db).await;

    //     let parsed_feature_result = match result {
    //         Ok(row) => parse_feature(row),
    //         Err(err) => Err(ParseError{message: err.to_string()})
    //     };

    //     match parsed_feature_result {
    //         Ok(feature) => Some(GeoEntity::from(feature)),
    //         Err(err) => {
    //             println!("{}", err);
    //             None
    //         }
    //     }
    // }

     pub async fn create_collection(&mut self, collection: &FeatureCollection) -> Result<FeatureCollection, FeatureRepositoryError> {
        
        let db = &self.pool;
        let label = &collection.label;

        let query = format!("INSERT INTO {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE}(label)
         VALUES('{label}') RETURNING id, label");

        println!("{}", query);

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        match result {
            Ok(row) => Ok(FeatureCollection::from(&row)),
            Err(err) => {
                println!("DB Error {}", err.to_string());
                Err(FeatureRepositoryError{message: err.to_string()})
            }
        }
    }

    pub async fn get_collections(&mut self, offset: i64, size: i64) -> Result<Vec<FeatureCollection>, FeatureRepositoryError> {
        
        let db = &self.pool;

        //NOTE: using deferred join to improve pagination
        let query = format!("SELECT id, label FROM {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE} \
            INNER JOIN ( SELECT id FROM {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE} LIMIT {size} OFFSET {offset} \
        ) AS tmp USING(id) ORDER BY id, label");

        print!("{}", query);

        let result = sqlx::query(&query)
        .fetch_all(db).await;

        match result {
            Ok(rows) => {
                let collections: Vec<FeatureCollection> = rows.iter().map(|row| {
                    FeatureCollection::from(row)
                }).collect();

                Ok(collections)
            },
            Err(err) => Err(FeatureRepositoryError{message: err.to_string()})
        } 
    }

    pub async fn get_collection_by_id(&mut self, id: i64) -> Result<FeatureCollection, FeatureRepositoryError> {
        
        let db = &self.pool;

        let query = format!("SELECT id, label FROM {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE} WHERE id = {id}");

        print!("{}", query);

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        match result {
            Ok(row) => { Ok(FeatureCollection::from(&row))},
            Err(err) => Err(FeatureRepositoryError{message: err.to_string()})
        } 
    }    

    
}

