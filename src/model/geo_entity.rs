use crate::model::{geometry::GeometryTrait, value::ValueTrait};

use super::{geometry::Geometry, value::ObjectValue};
use derive_new::new;

#[enum_dispatch::enum_dispatch]
pub trait GeoEntityTrait {//TOOD use super trait here?
    fn to_geo_json(&self) -> String;
}

#[derive(new)]
pub struct Feature {
    id: i64,
    geometry: Geometry,
    properties: ObjectValue
}

pub struct FeatureCollection {
    id: i64,
    features: Vec<Feature>
}

#[enum_dispatch::enum_dispatch(GeoEntityTrait)]
pub enum GeoEntity {
    Feature,
    FeatureCollection,
    EmptyResult
}

impl GeoEntityTrait for Feature {
    fn to_geo_json(&self) -> String {
        format!(r#"{{
            "type": "Feature",
            "id": {},
            "geometry": {},
            "properties": {}
        }}"#, self.id, self.geometry.to_geo_json(), &self.properties.to_geo_json())
    }
}

impl GeoEntityTrait for FeatureCollection {
    fn to_geo_json(&self) -> String {
        format!(r#"{{
            "type": "FeatureCollection",
            "id": {},
            "features": {}
        }}"#, self.id, parse_features(&self.features))
    }
}

// fn parse_properties(properties: &Vec<Property>) -> String {
//     let mut separator = "";

//     let properties: String = properties.iter().map(|property|{
//         let formated = format!("{}{}", separator, property.to_geo_json());
//         separator = ",";//ugh
//         formated
//     }).collect();

//     format!("{{ {} }}", properties)
// }

fn parse_features(features: &Vec<Feature>) -> String {
    let mut separator = "";

    let features: String = features.iter().map(|feature|{
        let formated = format!("{}{}", separator, feature.to_geo_json());
        separator = ",";//ugh
        formated
    }).collect();

    format!("{{ {} }}", features)
}

pub struct EmptyResult;

impl GeoEntityTrait for EmptyResult {
    fn to_geo_json(&self) -> String {
        "".to_string()
    }
}