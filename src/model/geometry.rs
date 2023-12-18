use derive_new::new;

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
        format!(r#"{{
            "type": "point",
            "coordinates": [{}, {}]
        }}"#, self.longitude, self.latitude)
    }
} 
