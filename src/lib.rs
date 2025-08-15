use std::{error::Error, str::FromStr};

/// Trait shared by all structs that will be representing files
pub trait Media {
    /// Returns the type of media
    fn media_type(&self) -> MediaType;
    /// Every struct implementing Media will need to point to a file
    fn path(&self) -> &std::path::Path;
    /// Check whether the file still exists
    fn exists(&self) -> bool;
    /// Path for the thumbnail for previews, otherwise None
    fn thumbnail_path(&self) -> Option<&std::path::Path>;
    /// Date of file creation
    fn date(&self) -> Option<chrono::NaiveDateTime>;
    /// Filesize in ?
    fn size(&self) -> u64;

    /// Returns true if the file passes specified conditions
    fn matches_filter(filter: &str) -> bool;
    /// Saves modifications to the struct instance to the associated file
    fn save() -> Result<(), TagError>;

    /// Returns tags, if the file doesn't support tags None is returned
    fn tags(&self) -> Option<&Vec<Tag>>;
    /// Adds a new tag to the file
    fn add_tag(&self, new_tag: Tag) -> Result<(), TagError>;
    /// Removes a tag from the file
    fn remove_tag(&self, tag_to_remove: Tag) -> Result<(), TagError>;
    /// Checks whether the file has the specified tag
    fn has_tag(&self, tag_to_search: Tag) -> bool;
}

/// Specify type of media for more sophisticated functions and handling
#[non_exhaustive]
pub enum MediaType {}

// Tag Struct ========================================================

/// Stores tag name and connections to namespace or other things
#[derive(PartialEq, Eq, Debug)]
pub struct Tag {
    tag_string: String,
}

impl Tag {
    pub fn matches(criteria: &str) -> bool {
        todo!();
    }
}

impl FromStr for Tag {
    type Err = TagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tag {
            tag_string: s.to_owned(),
        })
    }
}

// Tag Errors ========================================================

/// Error for tag related operations
#[derive(Debug)]
pub enum TagError {
    /// The file is missing or inaccessible
    FileMissing,
    /// The file exists but saving was not possible
    OtherSaveError,
    /// The tag does not exist
    TagMissing,
    /// The returned character is invalid
    InvalidCharacter(char),
    /// Other UwU
    UwUpsie,
}

impl std::fmt::Display for TagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TagError::FileMissing => "File Missing.".to_string(),
                TagError::OtherSaveError => "The file could not be saved.".to_string(),
                TagError::TagMissing => "Selected tag does not exist!".to_string(),
                TagError::InvalidCharacter(char) => format!("\"{char}\" is not a valid char."),
                TagError::UwUpsie => "other error".to_string(),
            }
        )
    }
}

impl std::error::Error for TagError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// Other ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_from_string_parse() {
        let expected = Tag {
            tag_string: "games/amogus".to_string(),
        };
        assert_eq!(expected, Tag::from_str("games/amogus").unwrap());
    }

    #[test]
    fn tag_from_string_unwrap() {
        Tag::from_str("test").unwrap();
    }
}
