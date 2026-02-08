use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Monitor {
    pub id: String,
    pub mode: String,
    pub opcode: String,
    // pub params:
    pub sprite_name: Option<String>,
    pub value: i32,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub visible: bool,

    pub slider_min: Option<f64>,
    pub slider_max: Option<f64>,
    pub is_discrete: Option<bool>,
}
