use serde_tuple::Deserialize_tuple;

#[derive(Debug, Deserialize_tuple, Clone)]
pub struct Field {
    pub value: String, // TODO: it *appears* that this is only string. but im not 100% sure. test
    // on more projects.
    #[serde(default)]
    pub id: Option<String>,
}
