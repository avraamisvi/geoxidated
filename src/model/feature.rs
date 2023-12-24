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

use serde_json::Value;

use crate::model::geometry::{Geometry, GeometryTrait};
use crate::model::value::{ObjectValue, ValueTrait};

use super::id::Id;
use super::json::Json;

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

impl From<Json> for Feature {
    fn from(value: Json) -> Self {
        let value: serde_json::Result<Value> = serde_json::from_str(value.to_string_ref());

        match value {
            Ok(parsed) => Feature::from(&parsed),
            Err(err) => panic!("Error could not parse json into Feature {}", err),
        }
    }
}