use std::{fmt::Display, error::Error};

use derive_new::new;

use crate::{repository::features_repository::FeatureRepository, model::feature_collection::{FeatureCollectionList, FeatureCollection}};


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

//TODO Service catch repository errors and translate to business errors 
//There is more to be done on service side
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
    
    // pub async fn get_single_feature(&mut self, id: &i64) -> Option<GeoEntity> {
    //     return self.repository.get_feature_by_id(id).await;
    // }

    // pub async fn save_feature(&mut self, feature: &Feature) -> Option<GeoEntity> {
    //     return self.repository.save_feature(feature).await;
    // }     
}


fn offset(page: i64, size: i64) -> i64 {
    page * size
}
