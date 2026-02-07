use core::fmt;
use std::io;

#[derive(Debug, Clone)]
pub struct ProjectParseError {
    msg: String,
}

impl std::error::Error for ProjectParseError {}
impl From<serde_json::Error> for ProjectParseError {
    fn from(value: serde_json::Error) -> Self {
        ProjectParseError {
            msg: value.to_string(),
        }
    }
}
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
