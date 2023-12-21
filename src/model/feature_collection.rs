use derive_new::new;
use serde_json::{Result, Value};

use super::id::Id;
use super::json::Json;
use super::feature::Feature;

#[derive(new)]
pub struct FeatureCollection {
    pub id: Id,
    pub label: String,
    pub features: Vec<Feature>
}

impl FeatureCollection {
    pub fn empty() -> Self {
        FeatureCollection {
            id: Id::None,
            label: "".to_string(),
            features: vec![]            
        }
    }
}

impl From<&serde_json::Value> for FeatureCollection {

    fn from(value: &serde_json::Value) -> Self {
        let id = Id::from(value["id"].as_i64());
        let label = value["label"].as_str().unwrap();

        let features = value["features"].as_array();

        let features = match features {
            Some(values) => {
                values.iter().map(|entry|{
                    Feature::from(entry)
                }).collect()
            },
            None => vec![],
        };

        FeatureCollection::new(id, label.to_string(), features)
    }
}

#[derive(new)]
pub struct FeatureCollectionList(Vec<FeatureCollection>);

impl FeatureCollectionList {

    pub fn to_json(&self) -> String {
        let mut separator = "";
        let collection_json: String = self.0.iter().map(|collection| {
            let geo_json = collection.to_geo_json();
            separator = ",";
            format!("{}{}", geo_json, separator)
        }).collect();

        format!("[{}]", collection_json)
    }
}

impl From<&Json> for FeatureCollection {

    fn from(value: &Json) -> Self {
        let value: &String = value.to_string_ref();
        
        let parsed_value_res: Result<Value> = serde_json::from_str(value);
        
        match parsed_value_res {
            Ok(parsed_value) => FeatureCollection::from(&parsed_value),
            Err(_) => FeatureCollection::empty()
        }
    }
}

impl From<Json> for FeatureCollection {
    fn from(value: Json) -> Self {
        let value: serde_json::Result<Value> = serde_json::from_str(value.to_string_ref());

        match value {
            Ok(parsed) => FeatureCollection::from(&parsed),
            Err(err) => panic!("Error could not parse json into FeatureCollection {}", err),
        }
    }
}

impl FeatureCollection {
    pub fn to_geo_json(&self) -> String {

        let mut output = format!(r#"{{
            "type": "FeatureCollection",
            "id": {},
            "label": "{}"
        "#, self.id, self.label);

        if !self.features.is_empty() {
            output.push_str(format!(r#","features": {}"#, parse_features(&self.features)).as_str());
        }

        output.push_str("}");

        output
    }
}

fn parse_features(features: &Vec<Feature>) -> String {
    let mut separator = "";

    let features: String = features.iter().map(|feature|{
        let formated = format!("{}{}", separator, feature.to_geo_json());
        separator = ",";//ugh
        formated
    }).collect();

    format!("[ {} ]", features)
}