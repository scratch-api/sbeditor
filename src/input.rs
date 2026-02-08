use serde::Deserialize;
use serde_tuple::*;

use crate::Prim;
use crate::prim;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum InputValue {
    Id(String),
    #[serde(deserialize_with = "prim::deserialize_prim")]
    Prim(Prim),
}
#[derive(Debug, Deserialize_tuple, Clone)]
pub struct ObscuredInput {
    pub shadow: u8,
    pub value: Option<InputValue>,
    pub obscurer: InputValue,
}
#[derive(Debug, Deserialize_tuple, Clone)]
pub struct SimpleInput {
    pub shadow: u8,
    pub value: Option<InputValue>,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Input {
    Simple(SimpleInput),
    Obscured(ObscuredInput),
}
