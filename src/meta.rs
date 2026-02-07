use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub semver: String,
    pub vm: String,
    pub agent: String,
    pub platform: Platform,
}
