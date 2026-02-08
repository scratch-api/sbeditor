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
}
