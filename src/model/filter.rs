use derive_new::new;
use serde_json::Value;

use super::json::Json;

/**
 * this is the version 1 of the query language, the plan is to allow for querying within sub fields values, like field = "field1.x1.y2.z3" 
    {
        "query": [
            {
                "type": "and",
                "expressions": [
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
                ]
            }
        ]
    }
 */

#[derive(new)]
pub struct Filter {
    pub expressions: Vec<Expression>
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
    pub field: Element,
    pub value: Element
}

pub struct NotEquals {
    pub field: Element,
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
        }
    }
}

impl From<ExpressionValueWrapper<'_>> for Equals {

    fn from(expression: ExpressionValueWrapper) -> Self {
        let value = expression.value;
        let field = value["field"].as_str();
        let field_value = value["value"].as_str();

        panic_when_none(field.is_none(), "field");
        panic_when_none(field_value.is_none(), "value");

        Equals { field: field.unwrap(), value: Element::from(value) }
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
                    value: data.clone(),
                    name: "".to_string(),
                }
            },
            _ => panic!("The value type of {value} is not supported.")
        }
    }
}


impl From<Value> for Filter {
    fn from(value: Value) -> Self {
        let expressions_opt = value["query"].as_array();

        match expressions_opt {
            Some(expressions) => {
                let expressions_parsed: Vec<Expression> = expressions.iter().map(|expression|{
                    Expression::from(parse_expression_value_wrapper(expression))
                }).collect();

                Filter::new(expressions_parsed)
            },
            None => Filter::new(vec![]),
        }
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