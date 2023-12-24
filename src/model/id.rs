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

use std::fmt::Display;

pub enum Id {
    IntId(IntId),
    None
} 

pub struct IntId(pub i64);

impl Id {
    pub fn as_int(&self) -> i64 {
        if let Id::IntId(id) = self {
            id.0
        } else {
            panic!("Can not convert Id to Integer");
        }
    } 

    pub fn is_empty(&self) -> bool {
        match self {
            Id::None => true,
            _ => false
        }
    }
}

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Id::IntId(IntId(value))
    }
}

impl From<Option<i64>> for Id {
    fn from(value: Option<i64>) -> Self {
        match value {
            Some(val) => Id::IntId(IntId(val)),
            None => Id::None,
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Id::IntId(id) => write!(f, "{}", id.0),
            Id::None => write!(f, ""),
        }
    }
}