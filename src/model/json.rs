use derive_new::new;

#[derive(new)]
pub struct Json(String);

impl Json {
    pub fn to_string_ref(&self) -> &String {
        &self.0
    }
} 