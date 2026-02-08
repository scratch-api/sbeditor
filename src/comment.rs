use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Comment {
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub minimized: bool,
    pub text: String,
}
