use serde::Deserialize;
use serde_tuple::*;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VarValue {
    String(String),
    I32(i32),
    F64(f64),
}

#[derive(Debug, Deserialize_tuple)]
pub struct Var {
    pub name: String,
    pub value: VarValue,
    #[serde(default)]
    pub is_cloud_var: bool,
}
