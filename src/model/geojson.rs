pub trait GeoJsonSerializer {
    fn to_geo_json(&self) -> String;
}