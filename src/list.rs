use serde::Deserialize;
use serde_tuple::*;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LIValue {
    String(String),
    I32(i32),
    F64(f64),
    Bool(bool),
    // Other(serde_json::Value),
}

#[derive(Debug, Deserialize_tuple)]
pub struct List {
    pub name: String,
    pub value: Vec<LIValue>,
}
