use derive_new::new;

use crate::{repository::features_repository::FeatureRepository, model::geo_entity::{GeoEntity, Feature}};

#[derive(new)]
pub struct FeatureService {
    repository: FeatureRepository
}

//TODO Service catch repository errors and translate to business errors 
//There is more to be done on service side
impl FeatureService {
    pub async fn get_single_feature(&mut self, id: &i64) -> Option<GeoEntity> {
        return self.repository.get_feature_by_id(id).await;
    }

    pub async fn save_feature(&mut self, feature: &Feature) -> Option<GeoEntity> {
        return self.repository.save_feature(feature).await;
    }   

    pub async fn create_feature_collection(&mut self, label: &String) -> Option<GeoEntity> {
        return self.repository.create_feature_collection(label).await;
    }   
}

