use derive_new::new;

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
            let formated = format!("{}", value.to_geo_json());
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
            let formated = format!("{}", value.to_geo_json());
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