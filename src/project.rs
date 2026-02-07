use crate::error;
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};
use zip::ZipArchive;

#[derive(Debug)]
pub struct Project {
    pub title: String,
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
        Ok(Self { title })
    }
}
