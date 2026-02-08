use serde_tuple::Deserialize_tuple;

#[derive(Debug, Deserialize_tuple)]
pub struct Field {
    pub value: serde_json::Value, // TODO: type this
    #[serde(default)]
    pub id: Option<String>,
}
