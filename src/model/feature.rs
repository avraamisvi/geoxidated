use serde_json::Value;

use crate::model::geometry::{Geometry, GeometryTrait};
use crate::model::value::{ObjectValue, ValueTrait};

use super::id::Id;

#[derive(derive_new::new)]
pub struct Feature {
    pub id: Id,
    pub geometry: Geometry,
    pub properties: ObjectValue
}

impl Feature {
    pub fn to_geo_json(&self) -> String {
        format!(r#"{{
            "type": "Feature",
            "id": {},
            "geometry": {},
            "properties": {}
        }}"#, self.id, self.geometry.to_geo_json(), &self.properties.to_geo_json())
    }
}

impl From<&Value> for Feature {
    fn from(value: &Value) -> Self {

        let id = Id::from(value["id"].as_i64());
        let geometry = Geometry::from(&value["geometry"]);
        let properties = if let Value::Object(properties) = &value["properties"] {
            Some(ObjectValue::from(properties))
        } else {
            None
        };
        
        Feature::new(id, geometry, properties.unwrap_or(ObjectValue::new(vec![])))
    }
}