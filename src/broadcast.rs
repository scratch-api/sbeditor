use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Broadcast(pub String);
