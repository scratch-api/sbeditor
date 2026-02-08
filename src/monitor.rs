use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonitorValue {
    // TODO: Check if these should be f64 or even strings
    I32(i32),
    Vec(Vec<i32>),
    Other(serde_json::Value),
}

#[derive(Debug, Deserialize)]
pub struct Monitor {
    pub id: String,
    pub mode: String,
    pub opcode: String,
    pub params: serde_json::Value, // TODO: type this into a struct
    pub sprite_name: Option<String>,
    pub value: MonitorValue,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub visible: bool,

    #[serde(rename = "sliderMin")]
    pub slider_min: Option<f64>,
    #[serde(rename = "sliderMax")]
    pub slider_max: Option<f64>,
    #[serde(rename = "isDiscrete")]
    pub is_discrete: Option<bool>,
}
