use crate::Extension;
use crate::Meta;
use crate::Monitor;
use crate::Sprite;
use crate::error;
use serde::Deserialize;
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};
use zip::ZipArchive;

#[derive(Debug, Deserialize)]
pub struct Project {
    #[serde(skip)]
    pub title: String,
    pub meta: Meta,
    pub extensions: Vec<Extension>,
    pub monitors: Vec<Monitor>,
    pub targets: Vec<Sprite>,
}

impl Project {
    pub fn from_sb3(path: PathBuf) -> Result<Self, error::ProjectParseError> {
        let bytes = fs::read(&path)?;
        let filename = match path.file_stem() {
            Some(name) => String::from(name.to_str().unwrap_or("Untitled")),
            None => String::from("Untitled"),
        };
        Self::from_sb3_bytes(&bytes, filename)
    }
    pub fn from_sb3_bytes(bytes: &[u8], title: String) -> Result<Self, error::ProjectParseError> {
        let cursor = io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor)?;
        let mut file = archive.by_name("project.json")?;
        let mut project_json = String::new();
        file.read_to_string(&mut project_json)?;

        Self::from_sb3_json(project_json, title)
    }
    pub fn from_sb3_json(data: String, title: String) -> Result<Self, error::ProjectParseError> {
        println!("{data}");

        let mut project: Project = serde_json::from_str(&data)?;
        project.title = title;
        println!("{project:#?}");
        Ok(project)
    }
}
