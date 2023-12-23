use std::{fmt::Display, error::Error};

use derive_new::new;

use crate::{repository::features_repository::{FeatureRepository, FeatureRepositoryError}, model::{feature_collection::{FeatureCollectionList, FeatureCollection}, feature::{self, Feature}, bbox::Bbox}};


#[derive(new, Debug)]
pub struct FeatureServiceError {
    pub message: String
}

impl Display for FeatureServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error when parsing {}", self.message)
    }
}

impl Error for FeatureServiceError {}

#[derive(new)]
pub struct FeatureService {
    repository: FeatureRepository
}

impl FeatureService {

    pub async fn get_collections(&mut self, page: i64, size: i64) -> Result<FeatureCollectionList, FeatureServiceError> {
        
        let result = self.repository.get_collections(offset(page, size), size).await;
        
        match result {
            Ok(collections) => Ok(FeatureCollectionList::new(collections)),
            Err(err) => Err(FeatureServiceError::new(err.message))
        }
    }

    pub async fn create_collection(&mut self, collection: &FeatureCollection) -> Result<FeatureCollection, FeatureServiceError> {
        let result = self.repository.create_collection(collection).await;

        match result {
            Ok(collection) => Ok(collection),
            Err(err) => Err(FeatureServiceError::new(err.message))
        }
    } 

    pub async fn update_collection(&mut self, collection: &FeatureCollection) -> Result<FeatureCollection, FeatureServiceError> {
        let result = self.repository.update_collection(collection).await;

        match result {
            Ok(collection) => Ok(collection),
            Err(err) => Err(FeatureServiceError::new(err.message))
        }
    } 

    pub async fn get_features_in_collection(&mut self, id: i64, page: i64, size: i64) -> Result<FeatureCollection, FeatureServiceError> {
        let features_result = self.repository.get_features_in_collection(id, offset(page, size), size).await;
        let feature_collection_result = self.repository.get_collection_by_id(id).await;

        match feature_collection_result {
            Ok(mut collection) => wrap_into_collection(collection, features_result),
            Err(err) => Err(FeatureServiceError::new(err.message))
        }
    } 

    pub async fn get_features_in_collection_by_bbox(&mut self, id: i64,
         bbox: &Bbox, page: i64, size: i64) -> Result<FeatureCollection, FeatureServiceError> {
        let features_result = self.repository.get_features_in_collection_by_bbox(id, bbox, offset(page, size), size).await;
        let feature_collection_result = self.repository.get_collection_by_id(id).await;

        match feature_collection_result {
            Ok(mut collection) => wrap_into_collection(collection, features_result),
            Err(err) => Err(FeatureServiceError::new(err.message))
        }
    } 

    pub async fn create_feature(&mut self, collection_id: i64, feature: &Feature) -> Result<FeatureCollection, FeatureServiceError> {
        let features_result = self.repository.create_feature(collection_id, feature).await;
        let feature_collection_result = self.repository.get_collection_by_id(collection_id).await;

        match feature_collection_result {
            Ok(mut collection) => wrap_feature_into_collection(collection, features_result),
            Err(err) => Err(FeatureServiceError::new(err.message))
        }
    }     

    pub async fn update_feature(&mut self, collection_id: i64, feature: &Feature) -> Result<FeatureCollection, FeatureServiceError> {
        let feature_collection_result = self.repository.get_collection_by_id(collection_id).await;
        let features_result = self.repository.update_feature(collection_id, feature).await;

        match feature_collection_result {
            Ok(mut collection) => wrap_feature_into_collection(collection, features_result),
            Err(err) => Err(FeatureServiceError::new(err.message))
        }
    }  
}


fn offset(page: i64, size: i64) -> i64 {
    page * size
}

fn wrap_into_collection(mut collection: FeatureCollection, 
    features_result:  Result<Vec<Feature>, FeatureRepositoryError>) -> Result<FeatureCollection, FeatureServiceError> {
    match features_result {
        Ok(features) => {
            collection.features = features;
            Ok(collection)
        }
        Err(err) => Err(FeatureServiceError::new(err.message))
    }
}

fn wrap_feature_into_collection(mut collection: FeatureCollection, 
    features_result:  Result<Feature, FeatureRepositoryError>) -> Result<FeatureCollection, FeatureServiceError> {
    match features_result {
        Ok(features) => {
            collection.features = vec![features];
            Ok(collection)
        }
        Err(err) => Err(FeatureServiceError::new(err.message))
    }
}