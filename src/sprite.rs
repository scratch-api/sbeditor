use std::collections::HashMap;

use crate::prim;
use crate::{Block, Broadcast, Comment, Costume, List, Prim, Sound, Var};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BlockValue {
    Block(Box<Block>),
    #[serde(deserialize_with = "prim::deserialize_prim")]
    Prim(Prim),
}

#[derive(Debug, Deserialize)]
pub struct Sprite {
    #[serde(rename = "isStage")]
    pub is_stage: bool,
    pub name: String,
    #[serde(rename = "currentCostume")]
    pub current_costume: u32,
    pub volume: u8,
    #[serde(rename = "layerOrder")]
    pub layer_order: i32,

    pub lists: HashMap<String, List>,
    #[serde(rename = "variables")]
    pub vars: HashMap<String, Var>,
    pub broadcasts: HashMap<String, Broadcast>,

    pub blocks: HashMap<String, BlockValue>,
    pub comments: HashMap<String, Comment>,

    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
}
