use derive_new::new;
use serde_json::Value;

use super::json::Json;

#[enum_dispatch::enum_dispatch]
pub trait GeometryTrait {
    fn to_geo_json(&self) -> String;
}

#[enum_dispatch::enum_dispatch(GeometryTrait)]
pub enum Geometry {
    Point,
    // MultiPoint,
    // LineString,
    // Polygon,
    // MultiLineString,
    // Polygon,
    // MultiPolygon,
    // GeometryCollection
}

#[derive(new)]
pub struct Point {
    longitude: f64,
    latitude: f64
}

impl GeometryTrait for Point {
    fn to_geo_json(&self) -> String {
        format!(r#"{{"type": "point", "coordinates": [{}, {}] }}"#, self.longitude, self.latitude)
    }
} 

//I will use the simplest approach here, through match, since Geometry is very limited
impl From<&Value> for Geometry {
    fn from(value: &Value) -> Self {
        let r#type = value["type"].as_str().unwrap();

        match r#type {
            "Point" => Geometry::Point(Point::from(value)),
            _ => unimplemented!()
        }
    }
}

impl From<&Value> for Point {
    fn from(value: &Value) -> Self {
        let points = value["coordinates"].as_array().unwrap();
        Point::new(points[0].as_f64().unwrap(), points[1].as_f64().unwrap())
    }
}

impl From<Json> for Geometry {
    fn from(value: Json) -> Self {
        let geom_value: serde_json::Result<serde_json::Value> = serde_json::from_str(value.to_string_ref());
        match geom_value {
            Ok(geom) => Geometry::from(&geom),
            Err(err) => panic!("Geometry could not be parsed {}", err),
        }
        
    }
}

