use derive_new::new;

use crate::{model::{geo_entity::{Feature, GeoEntity}, geometry::{Geometry, Point}, property::Property, value::{Value, NumberValue}}, repository::features_repository::FeatureRepository};

#[derive(new)]
pub struct FeatureService {
    repository: FeatureRepository
}

impl FeatureService {
    pub async fn get_single_feature(&mut self, id: &i64) -> Option<GeoEntity> {
        return self.repository.get_feature_by_id(id).await;
    }    
}

