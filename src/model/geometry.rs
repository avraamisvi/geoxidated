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
        format!(r#"{{"type": "Point", "coordinates": [{}, {}] }}"#, self.longitude, self.latitude)
    }
} 

//I will use the simplest approach here, through match, since Geometry is very limited
impl From<&Value> for Geometry {
    fn from(value: &Value) -> Self {
        let r#type = value["type"].as_str().unwrap().to_lowercase();

        if r#type == "point" {
            Geometry::Point(Point::from(value))
        } else {
            unimplemented!()
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

