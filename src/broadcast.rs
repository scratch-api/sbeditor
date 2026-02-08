use serde_tuple::*;

#[derive(Debug, Deserialize_tuple)]
pub struct Broadcast {
    pub name: String,
}
