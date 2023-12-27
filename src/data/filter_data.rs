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

use crate::model::filter::{Filter, Expression, Equals, Element};

pub trait IntoSQLQuery {
    fn into_sql_query(&self) -> String;
}

impl IntoSQLQuery for Filter {
    fn into_sql_query(&self) -> String {

        let sql_query = match &self.expressions {
            crate::model::filter::Expression::Equals(exp) => exp.into_sql_query(),
            crate::model::filter::Expression::NotEquals(exp) => todo!(),
            crate::model::filter::Expression::And(exp) => todo!(),
            crate::model::filter::Expression::Or(exp) => todo!(),
        };

        format!("{}", sql_query)
    }
}

impl IntoSQLQuery for Equals {
    fn into_sql_query(&self) -> String {
        format!("{} = {}", self.field, self.value.into_sql_query())
    }
}

impl IntoSQLQuery for Element {
    fn into_sql_query(&self) -> String {
        match self.e_type {
            crate::model::filter::ElementType::String => format!(r#"'{}'"#, self.value),
            crate::model::filter::ElementType::Number => format!("{}", self.value),
        }
    }
}