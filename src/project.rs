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
    #[serde(rename = "targets")]
    pub sprites: Vec<Sprite>,
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
        // println!("{data}");
        let deserializer = &mut serde_json::Deserializer::from_str(&data);
        let result: Result<Project, _> = serde_path_to_error::deserialize(deserializer);
        let mut project = match result {
            Ok(project) => project,
            Err(e) => {
                let msg = format!("oerr: {}", e);
                return Err(error::ProjectParseError::from(msg));
            }
        };
        project.title = title;

        Ok(project)
    }
    pub fn get_sprite_by_name(&mut self, name: &str) -> Option<&mut Sprite> {
        self.sprites.iter_mut().find(|s| s.name == name)
    }
}
