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
use serde_json::Value;

use super::json::Json;

/**
 * this is the version 1 of the query language, the plan is to allow for querying within sub fields values, like field = "field1.x1.y2.z3" 
{
    "collection_id": 123,
    "where": {
        "type": "and",
        "expressions": {
            {
                "type": "equals",
                "field": "filed_name",
                "value": 100
            },
            {
                "type": "not_equals",
                "field": "filed_name",
                "value": 100
            }                    
        }                    
    }
}
 */

#[derive(new)]
pub struct Filter {
    pub collection_id: i64,
    pub expressions: Expression
}

pub enum ElementType {
    String, 
    Number
}

pub struct Element {
    pub name: String,
    pub value: String,
    pub e_type: ElementType
}

pub struct Equals {
    pub field: String,
    pub value: Element
}

pub struct NotEquals {
    pub field: String,
    pub value: Element
}

pub struct And(Vec<Expression>);
pub struct Or(Vec<Expression>);

pub enum Expression {
    Equals(Equals),
    NotEquals(NotEquals),
    And(And),
    Or(Or)
}

impl From<Json> for Filter {

    fn from(value: Json) -> Self {
        let value: serde_json::Result<Value> = serde_json::from_str(value.to_string_ref());

        match value {
            Ok(parsed) => Filter::from(&parsed),
            Err(err) => panic!("Error could not parse json into Filter {}", err),
        }
    }
}

struct ExpressionValueWrapper<'a> {
    pub ex_type: String,
    pub value: &'a Value 
}

impl From<ExpressionValueWrapper<'_>> for Expression {
    fn from(value: ExpressionValueWrapper) -> Self {
        if value.ex_type == "equals" {
            Expression::Equals(Equals::from(value))
        } else if value.ex_type == "and" {
            Expression::And(And::from(value))
        } else {
            unimplemented!()
        }
    }
}

impl From<&Value> for Expression {
    fn from(value: &Value) -> Self {
        Expression::from(parse_expression_value_wrapper(value))
    }
}

impl From<ExpressionValueWrapper<'_>> for Equals {

    fn from(expression: ExpressionValueWrapper) -> Self {
        let value = expression.value;
        let field = value["field"].as_str();
        let field_value = &value["value"];

        panic_when_none(field.is_none(), "field");

        Equals { field: field.unwrap().to_string(), value: Element::from(field_value) }
    }
}

impl From<ExpressionValueWrapper<'_>> for And {

    fn from(expression: ExpressionValueWrapper) -> Self {
        let expressions = expression.value["expressions"].as_array();

        panic_when_none(expressions.is_none(), "Expressions in And must not be empty or null");

        let expressions_parsed = expressions.unwrap().iter().map(|exp|{
            Expression::from(exp)
        }).collect();

        And(expressions_parsed)
    }
}

impl From<&Value> for Element {

    fn from(value: &Value) -> Self {
        match value {
            Value::String(data) => {
                Element {
                    e_type: ElementType::String,
                    value: data.clone(),
                    name: "".to_string(),
                }
            },
            Value::Number(data) => {
                Element {
                    e_type: ElementType::Number,
                    value: data.to_string(),
                    name: "".to_string(),
                }
            },
            _ => panic!("The value type of {value} is not supported.")
        }
    }
}


// impl From<Value> for Filter {
//     fn from(value: Value) -> Self {
//         let expressions_opt = value["query"].as_array();

//         match expressions_opt {
//             Some(expressions) => {
//                 let expressions_parsed: Vec<Expression> = expressions.iter().map(|expression|{
//                     Expression::from(parse_expression_value_wrapper(expression))
//                 }).collect();

//                 Filter::new(expressions_parsed)
//             },
//             None => Filter::new(vec![]),
//         }
//     }
// }

impl From<&Value> for Filter {
    fn from(value: &Value) -> Self {
        let expression = &value["where"];
        let collection_id = value["collection_id"].as_i64();

        panic_when_none(expression.is_null(), "where");
        panic_when_none(collection_id.is_none(), "collection_id");

        let expressions_parsed = Expression::from(parse_expression_value_wrapper(expression));

        Filter::new(collection_id.unwrap(), expressions_parsed)
    }
}

fn parse_expression_value_wrapper(value: &Value) -> ExpressionValueWrapper {
    let ex_type_opt = value["type"].as_str();
    match ex_type_opt {
        Some(ex_type) => {
            ExpressionValueWrapper{ex_type: ex_type.to_lowercase(), value: value}
        },
        None => panic!("It was not possible to determine the type of the expression {}", value),
    }
}

fn panic_when_none(empty: bool, name: &'static str) {
    if empty {
        panic!("Error, the {name} is empty or null")
    }
}