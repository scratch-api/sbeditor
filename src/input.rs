use serde::Deserialize;
use serde_json;
use serde_tuple::*;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InputValue {
    Id(String),
    Prim(serde_json::Value),
}
#[derive(Debug, Deserialize_tuple)]
pub struct ObscuredInput {
    pub shadow: u8,
    pub value: InputValue,
    pub obscurer: InputValue,
}
#[derive(Debug, Deserialize_tuple)]
pub struct SimpleInput {
    pub shadow: u8,
    pub value: InputValue,
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Simple(SimpleInput),
    Obscured(ObscuredInput),
}
