use core::fmt;
use std::{fs, io, path::PathBuf};
use zip::ZipArchive;

pub fn speak() {
    println!("I spake, and thou hearest");
}

#[derive(Debug, Clone)]
pub struct ProjectParseError {
    msg: String,
}
impl std::error::Error for ProjectParseError {}
impl From<zip::result::ZipError> for ProjectParseError {
    fn from(value: zip::result::ZipError) -> Self {
        ProjectParseError {
            msg: value.to_string(),
        }
    }
}
impl From<io::Error> for ProjectParseError {
    fn from(value: io::Error) -> Self {
        ProjectParseError {
            msg: value.to_string(),
        }
    }
}
impl fmt::Display for ProjectParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse Project: {}", self.msg)
    }
}

#[derive(Debug)]
pub struct Project {
    pub title: String,
}

impl Project {
    pub fn from_sb3(path: PathBuf) -> Result<Self, ProjectParseError> {
        let bytes = fs::read(&path)?;
        let filename = match path.file_stem() {
            Some(name) => String::from(name.to_str().unwrap_or("Untitled")),
            None => String::from("Untitled"),
        };
        Self::from_sb3_bytes(&bytes, filename)
    }
    pub fn from_sb3_bytes(bytes: &[u8], title: String) -> Result<Self, ProjectParseError> {
        let cursor = io::Cursor::new(bytes);
        let archive = ZipArchive::new(cursor)?;
        println!("{}", archive.file_names().collect::<Vec<_>>().join(", "));
        Ok(Self { title })
    }
}
