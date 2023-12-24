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
use serde_json::Map;

use super::json::Json;

#[enum_dispatch::enum_dispatch]
pub trait ValueTrait {
    fn to_geo_json(&self) -> String;
}

#[enum_dispatch::enum_dispatch(ValueTrait)]
pub enum Value {
    StringValue,
    NumberValue,
    BooleanValue,
    ArrayValue,
    NullValue,
    ObjectValue,
}

#[derive(new)]
pub struct StringValue {
    value: String
}

#[derive(new)]
pub struct NumberValue {
    value: NumericalValue
}

#[derive(new)]
enum NumericalValue {
    Integer(i64),
    Float(f64)
}

#[derive(new)]
pub struct BooleanValue {
    value: bool
}

#[derive(new)]
pub struct ArrayValue {
    value: Vec<Value>
}

#[derive(new)]
pub struct NullValue {}

#[derive(new)]
pub struct ObjectValue {
    value: Vec<ObjectProperty>
}

impl ObjectValue {
    pub fn empty() -> Self {
        ObjectValue::new(vec![])
    }
}

#[derive(new)]
pub struct ObjectProperty {
    name: String,
    value: Value
}

impl ValueTrait for ObjectProperty {
    fn to_geo_json(&self) -> String {
        format!(r#""{}":{}"#, self.name, self.value.to_geo_json())
    }
}

impl ValueTrait for StringValue {
    fn to_geo_json(&self) -> String {
        format!("\"{}\"", &self.value)
    }
}

impl ValueTrait for NumberValue {
    fn to_geo_json(&self) -> String {
        format!("{}", &self.value.to_geo_json())
    }
}

impl From<i64> for NumberValue {
    fn from(value: i64) -> Self {
        NumberValue::new(NumericalValue::Integer(value))
    }
}

impl From<f64> for NumberValue {
    fn from(value: f64) -> Self {
        NumberValue::new(NumericalValue::Float(value))
    }
}

impl ValueTrait for BooleanValue {
    fn to_geo_json(&self) -> String {
        format!("{}", &self.value)
    }
}

impl ValueTrait for ArrayValue {
    fn to_geo_json(&self) -> String {
        let mut separator = "";
        let collected: String = self.value.iter().map(|value| {
            let formated = format!("{}{}", separator, value.to_geo_json());
            println!("{formated}");
            separator = ",";
            formated
        }).collect();

        format!("[{}]", collected)
    }
}

impl ValueTrait for NullValue {
    fn to_geo_json(&self) -> String {
        "NULL".to_string()
    }
}

impl ValueTrait for ObjectValue {
    fn to_geo_json(&self) -> String {
        let mut separator = "";
        let collected: String = self.value.iter().map(|value| {
            let formated = format!("{}{}", separator, value.to_geo_json());
            separator = ",";
            formated
        }).collect();

        format!("{{{}}}", collected)
    }
}

impl ValueTrait for NumericalValue {
    fn to_geo_json(&self) -> String {
        match self {
            NumericalValue::Integer(value) => format!("{}", value),
            NumericalValue::Float(value) => format!("{}", value),
        }
    }
}

struct SerdeProperty<'a>(&'a String, &'a serde_json::Value);

impl From<&Map<String, serde_json::Value>> for ObjectValue {
    fn from(properties: &Map<String, serde_json::Value>) -> Self {

        let parsed_properties: Vec<ObjectProperty> = properties.iter()
            .map(|(name, value)| {
                ObjectProperty::from(SerdeProperty(name, value))

        }).collect();
    
        ObjectValue::new(parsed_properties)
    }
}

impl From<SerdeProperty<'_>> for ObjectProperty {
    fn from(value: SerdeProperty) -> Self {
        let SerdeProperty(name, value) = value;

        let model_value: Value = value.clone().into();//TODO IS THIS REALLY SO BAD? SLOWER HERE FOR SURE
        ObjectProperty::new(name.clone(), model_value)
    }
}

impl From<Json> for ObjectValue {
    fn from(value: Json) -> Self {
        let mapped_values: serde_json::Result<Map<String, serde_json::Value>> = serde_json::from_str(value.to_string_ref());

        match mapped_values {
            Ok(values) => ObjectValue::from(&values),
            Err(err) => panic!("Value could not be parsed into ObjectValue {}", err),
        }
    }
}

impl Into<Value> for serde_json::Value {
    fn into(self) -> Value {
        parse_model_value(&self)
    }
}

fn parse_model_value(value: &serde_json::Value) -> Value {
    match value {
        serde_json::Value::Null => Value::from(NullValue{}),
        serde_json::Value::Bool(value) => Value::from(BooleanValue::new(*value)),
        serde_json::Value::Number(value) => {
            
            if value.is_f64() {
                Value::from(NumberValue::from(value.as_f64().unwrap()))
            } else {
                Value::from(NumberValue::from(value.as_i64().unwrap()))
            }
            
        },
        serde_json::Value::String(value) => Value::from(StringValue::new(value.as_str().to_string())),
        serde_json::Value::Array(values) => {
            let array_of_values: Vec<Value> = values.iter().map(|value|{
                parse_model_value(value)
            }).collect();

            Value::from(ArrayValue::new(array_of_values))
        },
        serde_json::Value::Object(values) => {
    
            let properties: Vec<ObjectProperty> = values.iter().map(|(name, value)|{
                ObjectProperty::new(name.clone(), parse_model_value(value))
            }).collect();

            Value::from(ObjectValue::new(properties))
        },
        
    }
}