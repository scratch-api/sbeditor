use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ArgumentDefault {
    String(String),
    F64(f64),
    Other(serde_json::Value),
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    // afaik these two attrs are useless
    pub tag_name: String,
    pub children: Vec<()>,

    // the following attrs are not required, so i will #[serde(default)] them
    #[serde(default)]
    #[serde(rename = "proccode")]
    pub proc_code: Option<String>,
    #[serde(default)]
    #[serde(rename = "warp", deserialize_with = "deserialize_json_string")]
    pub is_warp: Option<bool>,

    #[serde(default)]
    #[serde(rename = "argumentids", deserialize_with = "deserialize_json_string")]
    pub argument_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "argumentnames", deserialize_with = "deserialize_json_string")]
    pub argument_names: Option<Vec<String>>,
    #[serde(default)]
    #[serde(
        rename = "argumentdefaults",
        deserialize_with = "deserialize_json_string"
    )]
    pub argument_defaults: Option<Vec<ArgumentDefault>>,
    #[serde(default, deserialize_with = "deserialize_json_string")]
    pub has_next: Option<bool>,
}
// https://users.rust-lang.org/t/need-help-with-serde-deserialize-with/18374/2
fn deserialize_json_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> Deserialize<'a>, // higher rank trait bound. no idea what that actually means but
                                // oh well...
{
    let s = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}
