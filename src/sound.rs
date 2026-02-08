use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
    pub name: String,
    pub asset_id: String,
    pub md5ext: String,
    pub data_format: String,
    pub rate: Option<i32>,
    pub sample_count: Option<i32>,
}
