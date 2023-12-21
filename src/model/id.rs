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