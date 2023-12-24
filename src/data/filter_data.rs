use crate::model::filter::{Filter, Expression, Equals, Element};


pub trait IntoSQLQuery {
    fn into_sql_query(&self) -> String;
}

impl IntoSQLQuery for Filter {
    fn into_sql_query(&self) -> String {

        let sql_query = self.expressions.iter().map(|expression|{
            match expression {
                crate::model::filter::Expression::Equals(exp) => exp.into_sql_query(),
                crate::model::filter::Expression::NotEquals(exp) => todo!(),
                crate::model::filter::Expression::And(exp) => todo!(),
                crate::model::filter::Expression::Or(exp) => todo!(),
            }
        });
    }
}

impl IntoSQLQuery for Equals {
    fn into_sql_query(&self) -> String {
        format!("{} = {}", self.field.into_sql_query(), self.value.into_sql_query())
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