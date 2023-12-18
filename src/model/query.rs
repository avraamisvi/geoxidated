use super::{get_single::GetSingle, geo_entity::{GeoEntity, EmptyResult}};


#[enum_dispatch::enum_dispatch]
pub trait QueryTrait {
    fn execute(&self) -> GeoEntity;
}

#[enum_dispatch::enum_dispatch(QueryTrait)]
pub enum Query {
    GetSingle,
    InvalidQuery
}

pub struct InvalidQuery {

}

impl QueryTrait for InvalidQuery {
    fn execute(&self) -> GeoEntity {
        GeoEntity::from(EmptyResult{})
    }
}