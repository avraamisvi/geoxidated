use sqlx::{postgres::PgRow, Row, Error};

use crate::model::{feature::Feature, id::Id, geometry::Geometry, json::Json, value::ObjectValue};

//TODO separate Feature from FeatureEntity
impl From<&PgRow> for Feature {
    fn from(row: &PgRow) -> Self {
        
        let id = {
            let id_res: Result<i64, Error> = row.try_get(0);
            match id_res {
                Ok(id) => Id::from(id),
                Err(err) => panic!("Unexpected value for id {}", err)
            }
        };

        let properties_str: String = row.try_get(1).unwrap_or("".to_string());
        let geometry_str: String = row.try_get(2).unwrap_or("".to_string());

        Feature::new(id, Geometry::from(Json::new(geometry_str)), ObjectValue::from(Json::new(properties_str)))
    }
}