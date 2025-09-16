use crate::Tag;
use std::time::SystemTime;

#[derive(Debug)]
/// Media struct representing a file
///
/// Can contain tags, if tags are supported the tags field is Some()
/// Also contains a path pointing to the file on disk
/// A date read from the xmp metadata as that is more reliable than filesystem dates
/// And a modified time to check whether the file was modified since it was read into the struct
pub struct Media {
    pub(crate) path: std::path::PathBuf,
    pub(crate) date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub(crate) tags: Option<Vec<Tag>>,
    pub(crate) modified: SystemTime,
}

impl Media {
    /// Creates a new media instance for the given path
    pub fn new(path: &std::path::Path) -> Option<Self> {
        let mut new_media = Media {
            path: path.to_path_buf(),
            date: None,
            tags: None,
            modified: SystemTime::now(),
        };

        match new_media.update() {
            Ok(_) => Some(new_media),
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
