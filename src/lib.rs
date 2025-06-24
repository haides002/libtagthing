use std::{error::Error, str::FromStr};

pub trait Media {
    fn media_type(&self) -> MediaType;
    fn path(&self) -> &std::path::Path;
    fn exists(&self) -> bool;
    fn thumbnail_path(&self) -> Option<&std::path::Path>;
    fn date(&self) -> Option<chrono::NaiveDateTime>;
    fn size(&self) -> u64;

    fn matches_filter(filter: &str) -> bool;
    fn save() -> Result<(), TagError>;

    fn tags(&self) -> Option<&Vec<Tag>>;
    fn add_tag(&self, new_tag: Tag) -> Result<(), TagError>;
    fn remove_tag(&self, tag_to_remove: Tag) -> Result<(), TagError>;
    fn has_tag(&self, tag_to_search: Tag) -> bool;
}

pub enum MediaType {}

// Tag Struct ========================================================
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
#[derive(Debug)]
pub enum TagError {
    FileMissing,
    OtherSaveError,
    TagMissing,
    InvalidCharacter(char),
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
