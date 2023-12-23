use derive_new::new;


#[derive(new)]
pub struct Bbox {
    min_lng: f64, 
    min_lat: f64,
    max_lng: f64,
    max_lat: f64
}

impl Bbox {
    pub fn to_wkt(&self) -> String {
        format!("POLYGON(({} {}, {} {}, {} {}, {} {}, {} {}))", self.min_lng, self.min_lat, 
                                                                self.min_lng, self.max_lat,
                                                                self.max_lng, self.max_lat,
                                                                self.max_lng, self.min_lat,
                                                                self.min_lng, self.min_lat)
    }
}