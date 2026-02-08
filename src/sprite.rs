use std::collections::HashMap;

use crate::{Block, Broadcast, Comment, Costume, List, Sound, Var};
use serde::Deserialize;

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

    pub blocks: HashMap<String, Block>,
    pub comments: HashMap<String, Comment>,

    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
}
