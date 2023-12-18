use std::{error::Error, fmt::Display};
use sqlx::{postgres::PgRow, Row};

use serde_json::{Value, Map};

use crate::model::{geo_entity::{GeoEntity, Feature}, geometry::{Geometry, Point, GeometryTrait}, value::{NullValue, BooleanValue, NumberValue, StringValue, ArrayValue, ObjectValue, ObjectProperty, ValueTrait}};
use crate::model::value::Value as ModelValue;

#[derive(Debug)]
pub struct ParseError {
    pub message: String
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error when parsing {}", self.message)
    }
}

impl Error for ParseError {}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError{message: value.to_string()}
    }
}

impl Into<ModelValue> for Value {
    fn into(self) -> ModelValue {
        parse_model_value(&self)
    }
}

//TODO recursive, but what would be the best approach?
pub fn parse_model_value(value: &Value) -> ModelValue {
    match value {
        Value::Null => ModelValue::from(NullValue{}),
        Value::Bool(value) => ModelValue::from(BooleanValue::new(*value)),
        Value::Number(value) => {
            
            if value.is_f64() {
                ModelValue::from(NumberValue::from(value.as_f64().unwrap()))
            } else {
                ModelValue::from(NumberValue::from(value.as_i64().unwrap()))
            }
            
        },
        Value::String(value) => ModelValue::from(StringValue::new(value.as_str().to_string())),
        Value::Array(values) => {
            let array_of_values: Vec<ModelValue> = values.iter().map(|value|{
                parse_model_value(value)
            }).collect();

            ModelValue::from(ArrayValue::new(array_of_values))
        },
        Value::Object(values) => {
    
            let properties: Vec<ObjectProperty> = values.iter().map(|(name, value)|{
                ObjectProperty::new(name.clone(), parse_model_value(value))
            }).collect();

            ModelValue::from(ObjectValue::new(properties))
        },
        
    }
}

pub fn parse_feature(row: PgRow) -> Result<Feature, ParseError> {

    let id: i64 = row.try_get(0).unwrap_or(0);
    let properties: String = row.try_get(1).unwrap_or("".to_string());
    let geometry: String = row.try_get(2).unwrap_or("".to_string());

    let geom_value: Value = serde_json::from_str(&geometry)?;

    Ok(Feature::new(id, parse_geometry(&geom_value), parse_properties(properties)?))
}

pub fn parse_geometry(geom: &Value) -> Geometry {
    if geom["type"].as_str().unwrap().to_lowercase() == "point" {
        let points = geom["coordinates"].as_array().unwrap();
        Geometry::from(Point::new(points[0].as_f64().unwrap(), points[1].as_f64().unwrap()))
    } else {
        unimplemented!()
    }
}

pub fn parse_properties(properties: String) -> Result<ObjectValue, ParseError> {
    let str_properties:Map<String, Value> = serde_json::from_str(&properties)?;

    let properties: Vec<ObjectProperty> = str_properties.iter().map(|(name, value)|{
        let model_value: ModelValue = value.clone().into();//TODO IS THIS REALLY SO BAD? SLOW HERE FOR SURE
        ObjectProperty::new(name.clone(), model_value)
    }).collect();

    Ok(ObjectValue::new(properties))
}

pub fn parse_properties_from_value(properties: &Map<String, Value>) -> Result<ObjectValue, ParseError> {

    let parsed_properties: Vec<ObjectProperty> = properties.iter().map(|(name, value)|{
        let model_value: ModelValue = value.clone().into();//TODO IS THIS REALLY SO BAD? SLOW HERE FOR SURE
        ObjectProperty::new(name.clone(), model_value)
    }).collect();

    Ok(ObjectValue::new(parsed_properties))
}

impl From<&Value> for Feature {
    fn from(value: &Value) -> Self {

        let id = &value["id"].as_i64().unwrap_or(0);
        let geometry = parse_geometry(&value["geometry"]);
        let properties = if let Value::Object(properties) = &value["properties"] {
            Some(parse_properties_from_value(properties).unwrap())
        } else {
            None
        };
        
        Feature::new(*id, geometry, properties.unwrap_or(ObjectValue::new(vec![])))
    }
}
