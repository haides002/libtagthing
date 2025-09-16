use crate::Tag;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Media {
    pub(crate) path: std::path::PathBuf,
    pub(crate) date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub(crate) tags: Option<Vec<Tag>>,
    pub(crate) modified: SystemTime,
}

impl Media {
    pub fn new(path: &std::path::Path) -> Option<Box<Self>> {
        let mut new_media = Media {
            path: path.to_path_buf(),
            date: None,
            tags: None,
            modified: SystemTime::now(),
        };

        match new_media.update() {
            Ok(_) => Some(Box::new(new_media)),
            Err(_) => None,
        }
    }

    /// Every struct implementing Media will need to point to a file
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Date of file creation
    pub fn date(&self) -> Option<chrono::DateTime<chrono::FixedOffset>> {
        self.date
    }
}
