use std::{error::Error, str::FromStr};
mod filter;
mod image;

/// Read a File at the given path
pub fn read_file(path: &std::path::Path) -> Result<Box<dyn Media>, TagError> {
    let file_extension = match path.extension() {
        Some(extension) => extension.to_str().unwrap(),
        None => {
            return Err(TagError::MissingFileExtension);
        }
    };

    let media: Box<dyn Media> = match file_extension {
        "jpg" | "jpeg" | "png" => {
            crate::image::Image::new(path).ok_or(TagError::CouldNotReadFile)?
        }
        _ => {
            return Err(TagError::UnsupportedFile);
        }
    };

    Ok(media)
}

/// Trait shared by all structs that will be representing files
pub trait Media {
    /// Returns the type of media
    fn media_type(&self) -> MediaType;
    /// Every struct implementing Media will need to point to a file
    fn path(&self) -> &std::path::Path;
    /// Check whether the file still exists
    fn exists(&self) -> bool;
    /// Check wheather the file was updated on disk
    fn was_updated_on_disk(&self) -> Result<bool, TagError>;
    /// Updates the file to the state on disk
    fn update(&mut self) -> Result<(), TagError>;
    /// Path for the thumbnail for previews, otherwise None
    fn thumbnail_path(&self) -> Option<&std::path::Path>;
    /// Date of file creation
    fn date(&self) -> Option<chrono::NaiveDateTime>;
    /// Filesize in ?
    fn size(&self) -> Result<u64, std::io::Error> {
        Ok(std::fs::metadata(self.path())?.len())
    }

    /// Returns true if the file passes specified conditions
    fn matches_filter(&self, fltr: Vec<filter::Token>) -> bool {
        use filter::*;

        let mut stack: Vec<bool> = Vec::new();
        for element in fltr {
            match element {
                Token::Atom(content) => {
                    let mut matches: bool = false;
                    if self.supports_tags() {
                        for tag in self.tags().expect("has_tags() seems to have returned BS") {
                            if tag.matches(&content) {
                                matches = true;
                                break;
                            }
                        }
                    }

                    stack.push(matches);
                }
                Token::Or => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(left || right);
                }
                Token::Xor => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(left ^ right);
                }
                Token::And => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(left && right);
                }
                Token::Xnor => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(!(left ^ right));
                }
                Token::Nand => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(!(left && right));
                }
                Token::Not => {
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(!left);
                }
                Token::GroupOpen => {}
                Token::GroupClose => {}
            }
        }

        // check if the evaluation went cleanly
        assert!(stack.len() == 1);

        stack.pop().unwrap()
    }
    /// Saves modifications to the struct instance to the associated file
    fn save(&self) -> Result<(), TagError>;

    /// Returns tags in an ordered manner, if the file doesn't support tags None is returned
    fn tags(&self) -> Option<&Vec<Tag>>;
    /// Returns true if the piece of media can hold tags
    fn supports_tags(&self) -> bool {
        self.tags().is_some()
    }
    /// Adds a new tag to the file
    fn add_tag(&mut self, new_tag: Tag) -> Result<(), TagError>;
    /// Removes a tag from the file
    fn remove_tag(&mut self, tag_to_remove: Tag) -> Result<(), TagError>;
    /// Checks whether the file has the specified tag
    fn has_tag(&self, tag_to_search: Tag) -> bool {
        match self.tags() {
            Some(tags) => {
                for tag in tags {
                    if *tag == tag_to_search {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }
}

/// Specify type of media for more sophisticated functions and handling
#[non_exhaustive]
pub enum MediaType {
    Image,
}

// Tag Struct ========================================================

/// Stores tag name and connections to namespace or other things
#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
pub struct Tag {
    tag_string: String,
}

impl Tag {
    pub fn new(tag_content: String) -> Self {
        Self {
            tag_string: tag_content,
        }
    }

    pub fn matches(&self, criteria: &str) -> bool {
        let namespace_regex = regex::Regex::new(r"([\s\w]+:)").unwrap();
        let tag_no_namespace = namespace_regex.replace_all(&self.tag_string, "");

        let criteria_wildcard_support = criteria.replace("*", ".*");
        let criteria_regex =
            regex::Regex::new(&format!(r"^((?:{})(?:/.*)?)$", criteria_wildcard_support)).unwrap();

        criteria_regex.is_match(&tag_no_namespace)
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
    /// Tagging is not supported for the file
    TagsNotSupported,
    /// The returned character is invalid
    InvalidCharacter(char),
    /// The file does not have a valid sufix
    MissingFileExtension,
    /// The file could not be read
    CouldNotReadFile,
    /// Files with this file-extension are not supported.
    UnsupportedFile,
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
                TagError::TagsNotSupported => "Tagging is not supported for this file.".to_string(),
                TagError::InvalidCharacter(char) => format!("\"{char}\" is not a valid char."),
                TagError::MissingFileExtension => "The File is missing an extension.".to_string(),
                TagError::CouldNotReadFile => "Could not read file.".to_string(),
                TagError::UnsupportedFile =>
                    "Files with this file-extension are not supported.".to_string(),
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

    #[test]
    fn tag_matching() {
        let tag = Tag::from_str("nature:tree/person:john").unwrap();

        assert!(tag.matches("*"));
        assert!(tag.matches("tree"));
        assert!(tag.matches("tr*"));
        assert!(tag.matches("tr*e"));
        assert!(tag.matches("tree/john"));

        assert!(!tag.matches("john"));
    }
}
