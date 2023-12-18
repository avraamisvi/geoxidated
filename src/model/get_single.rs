use derive_new::new;

use crate::services::feature::get_single_feature;

use super::query::QueryTrait;
use super::geo_entity::GeoEntity;

#[derive(new)]
pub struct GetSingle {
    feature_id: i64
}

impl QueryTrait for GetSingle {
    fn execute(&self) -> GeoEntity {
        GeoEntity::from(get_single_feature(&self.feature_id))
    }
}
