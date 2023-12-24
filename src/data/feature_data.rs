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

use sqlx::{postgres::PgRow, Row, Error};

use crate::model::{feature::Feature, id::Id, geometry::Geometry, json::Json, value::ObjectValue};

//TODO separate Feature from FeatureEntity
impl From<&PgRow> for Feature {
    fn from(row: &PgRow) -> Self {
        
        let id = {
            let id_res: Result<i64, Error> = row.try_get(0);
            match id_res {
                Ok(id) => Id::from(id),
                Err(err) => panic!("Unexpected value for id {}", err)
            }
        };

        let properties_str: String = row.try_get(1).unwrap_or("{}".to_string());
        let geometry_str: String = row.try_get(2).unwrap_or("".to_string());

        Feature::new(id, Geometry::from(Json::new(geometry_str)), ObjectValue::from(Json::new(properties_str)))
    }
}