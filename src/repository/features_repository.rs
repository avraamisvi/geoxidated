/*
    Copyright (c)  Abraão Isvi <avraamisvi@users.noreply.github.com>

    Permission is hereby granted, free of charge, to any
    person obtaining a copy of this software and associated
    documentation files (the "Software"), to deal in the
    Software without restriction, including without
    limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software
    is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice
    shall be included in all copies or substantial portions
    of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
    TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
    PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
    SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
    OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
    IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
 */

use std::{fmt::Display, error::Error};

use derive_new::new;

use crate::model::{feature::Feature, feature_collection::FeatureCollection, value::ValueTrait, geometry::GeometryTrait, bbox::Bbox, filter::Filter};

static GEOXIDATED_SCHEMA: &str = "geoxidated";
static FEATURE_TABLE: &str = "feature";
static COLLECTION_TABLE: &str = "features_collection";
static FEATURES_IN_COLLECTION: &str = "features_in_collection";

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
                                     ON fi.feature_id = fa.id AND fi.collection_id = {id} LIMIT {size} OFFSET {offset}");

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

    pub async fn get_features_in_collection_by_bbox(&mut self, collection_id: i64, bbox: &Bbox, offset: i64, size: i64) -> Result<Vec<Feature>, FeatureRepositoryError> {
        
        let db = &self.pool;

        let bbox_geom = bbox.to_wkt();
        let query = format!(r#"SELECT id, \
                                     properties::text,\
                                     ST_AsGeoJSON(geometry) \
                                     FROM {GEOXIDATED_SCHEMA}.{FEATURE_TABLE} fa \
                                     INNER JOIN {GEOXIDATED_SCHEMA}.{FEATURES_IN_COLLECTION} fi \
                                     ON fi.feature_id = fa.id AND fi.collection_id = {collection_id}
                                     WHERE ST_Intersects(ST_GeomFromText('{bbox_geom}'), 4326, fa.geometry) 
                                     LIMIT {size} OFFSET {offset}"#);

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

    pub async fn get_features_in_collection_by_filter(&mut self, collection_id: i64, 
        filter: &Filter, offset: i64, size: i64) -> Result<Vec<Feature>, FeatureRepositoryError> {
       todo!() 
        // let db = &self.pool;

        // let bbox_geom = filter.;
        // let query = format!(r#"SELECT id, \
        //                              properties::text,\
        //                              ST_AsGeoJSON(geometry) \
        //                              FROM {GEOXIDATED_SCHEMA}.{FEATURE_TABLE} fa \
        //                              INNER JOIN {GEOXIDATED_SCHEMA}.{FEATURES_IN_COLLECTION} fi \
        //                              ON fi.feature_id = fa.id AND fi.collection_id = {collection_id}
        //                              WHERE ST_Intersects(ST_GeomFromText('{bbox_geom}'), 4326, fa.geometry) 
        //                              LIMIT {size} OFFSET {offset}"#);

        // let result = sqlx::query(&query)
        // .fetch_all(db).await;

        // match result {
        //     Ok(rows) => {
        //         let collections: Vec<Feature> = rows.iter().map(|row| {
        //             Feature::from(row)
        //         }).collect();

        //         Ok(collections)
        //     },
        //     Err(err) => {
        //         println!("DB Error {}", err.to_string());
        //         Err(FeatureRepositoryError{message: err.to_string()})
        //     }
        // }
    }

    pub async fn get_feature_by_id(&mut self, feature_id: i64) -> Result<Feature, FeatureRepositoryError> {
        
        let db = &self.pool;

        let query = format!(r#"SELECT id,
                                     properties::text,
                                     ST_AsGeoJSON(geometry)
                                     FROM {GEOXIDATED_SCHEMA}.{FEATURE_TABLE} fa
                                     WHERE fa.id = {feature_id}"#);

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        match result {
            Ok(row) => {
                Ok(Feature::from(&row))
            },
            Err(err) => {
                println!("DB Error {}", err.to_string());
                Err(FeatureRepositoryError{message: err.to_string()})
            }
        }
    }    

    pub async fn create_feature(&mut self, collection_id: i64, feature: &Feature) -> Result<Feature, FeatureRepositoryError> {
        
        let db = &self.pool;
        let geo_json = feature.geometry.to_geo_json();
        let properties_json = feature.properties.to_geo_json();

        let query = format!(r#"
            WITH data(data_properties, data_geometry) AS (
                VALUES('{properties_json}'::json, ST_GeomFromGeoJSON('{geo_json}'))              
            )
            , insert_feature AS (
                INSERT INTO {GEOXIDATED_SCHEMA}.{FEATURE_TABLE}(properties, geometry)
                SELECT data_properties, data_geometry
                FROM   data
                RETURNING properties, geometry, id AS insert_feature_id
                )
            , insert_features_in_col AS (
                INSERT INTO {GEOXIDATED_SCHEMA}.{FEATURES_IN_COLLECTION}(feature_id, collection_id)
                SELECT insert_feature.insert_feature_id, {collection_id} FROM insert_feature
            )
            SELECT insert_feature.insert_feature_id, insert_feature.properties::text, ST_AsGeoJSON(insert_feature.geometry)
            FROM   insert_feature
        "#);

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        match result {
            Ok(row) => Ok(Feature::from(&row)),
            Err(err) => {
                println!("DB Error {}", err.to_string());
                Err(FeatureRepositoryError{message: err.to_string()})
            }
        }
    }

    pub async fn update_feature(&mut self, collection_id: i64, feature: &Feature) -> Result<Feature, FeatureRepositoryError> {
        
        let db = &self.pool;
        let geo_json = feature.geometry.to_geo_json();
        let properties_json = feature.properties.to_geo_json();
        let feature_id = &feature.id;


        let query = format!(r#"
            WITH data(data_properties, data_geometry) AS (
                VALUES('{properties_json}'::json, ST_GeomFromGeoJSON('{geo_json}'))              
            )
            , update_feature AS (
                UPDATE {GEOXIDATED_SCHEMA}.{FEATURE_TABLE}
                    SET properties = '{properties_json}'::json,
                        geometry = ST_GeomFromGeoJSON('{geo_json}')
                    WHERE id = {feature_id}
                RETURNING id, properties, geometry 
                )
            , insert_features_in_col AS (
                INSERT INTO {GEOXIDATED_SCHEMA}.{FEATURES_IN_COLLECTION}(feature_id, collection_id)
                SELECT update_feature.id, {collection_id} FROM update_feature ON CONFLICT (feature_id, collection_id) DO NOTHING
            )
            SELECT update_feature.id, update_feature.properties::text, ST_AsGeoJSON(update_feature.geometry)
            FROM   update_feature
        "#);

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        match result {
            Ok(row) => Ok(Feature::from(&row)),
            Err(err) => {
                println!("DB Error {}", err.to_string());
                Err(FeatureRepositoryError{message: err.to_string()})
            }
        }
    }

     pub async fn create_collection(&mut self, collection: &FeatureCollection) -> Result<FeatureCollection, FeatureRepositoryError> {
        
        let db = &self.pool;
        let label = &collection.label;
        let properties = &collection.properties.to_geo_json();

        let query = format!("INSERT INTO {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE}(label, properties)
         VALUES('{label}', '{properties}'::json) RETURNING id, label, properties::text");

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

    pub async fn update_collection(&mut self, collection: &FeatureCollection) -> Result<FeatureCollection, FeatureRepositoryError> {
        
        let db = &self.pool;
        let label = &collection.label;
        let properties = &collection.properties.to_geo_json();
        let id = &collection.id;

        let query = format!(r#"UPDATE {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE}
        SET label = '{label}', properties = '{properties}'::json
        WHERE id = {id} RETURNING id, label, properties::text"#);

        print!("{}", query);

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
        let query = format!("SELECT id, label, properties::text FROM {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE} \
            INNER JOIN ( SELECT id FROM {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE} LIMIT {size} OFFSET {offset} \
        ) AS tmp USING(id) ORDER BY id, label");

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

        let query = format!("SELECT id, label, properties::text FROM {GEOXIDATED_SCHEMA}.{COLLECTION_TABLE} WHERE id = {id}");

        let result = sqlx::query(&query)
        .fetch_one(db).await;

        match result {
            Ok(row) => { Ok(FeatureCollection::from(&row))},
            Err(err) => Err(FeatureRepositoryError{message: err.to_string()})
        } 
    }    

    
}

