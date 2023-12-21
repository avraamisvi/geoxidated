use sqlx::{postgres::PgRow, Row, Error};

use crate::model::{id::Id, feature_collection::FeatureCollection};

//TODO separate Feature from FeatureEntity
impl From<&PgRow> for FeatureCollection {
    fn from(row: &PgRow) -> Self {
        
        let id = {
            let id_res: Result<i64, Error> = row.try_get(0);
            match id_res {
                Ok(id) => Id::from(id),
                Err(err) => panic!("Unexpected value for id {}", err)
            }
        };

        let label: String = row.try_get(1).unwrap_or("".to_string());

        // let properties_str: String = row.try_get(1).unwrap_or("".to_string());

        FeatureCollection::new(id, label, vec![])
    }
}