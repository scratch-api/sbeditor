use std::collections::HashMap;

use crate::{Field, Input, Mutation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Field>,
    pub shadow: bool,
    #[serde(rename = "topLevel")]
    pub is_top_level: bool,
    pub comment: Option<String>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub mutation: Option<Mutation>,
}
