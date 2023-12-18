use super::value::Value;
use super::value::ValueTrait;

use derive_new::new;

#[derive(new)]
pub struct Property {
    name: String,
    value: Value
}

impl Property {
    pub fn to_geo_json(&self) -> String {
        format!(r#""{}": {}"#, self.name, self.value.to_geo_json())
    }
}
