use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    pub name: String,
    pub asset_id: String,
    pub md5ext: String,
    pub data_format: String,
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,
    pub bitmap_resolution: Option<f64>,
}
