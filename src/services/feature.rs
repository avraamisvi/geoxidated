use derive_new::new;

use crate::{repository::features_repository::FeatureRepository, model::geo_entity::GeoEntity};

#[derive(new)]
pub struct FeatureService {
    repository: FeatureRepository
}

impl FeatureService {
    pub async fn get_single_feature(&mut self, id: &i64) -> Option<GeoEntity> {
        return self.repository.get_feature_by_id(id).await;
    }    
}

