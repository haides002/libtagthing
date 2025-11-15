use std::fmt::Display;

use crate::TagError;

/// Stores tag name and connections to namespace or other things
#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
pub struct Tag {
    pub tag_string: String,
}

impl Tag {
    /// Creates a new tag from the string
    pub fn new(tag_content: String) -> Self {
        Self {
            tag_string: tag_content,
        }
    }

    /// Checks whether the tag matches the given criteria
    pub(crate) fn matches(&self, criteria: &str) -> bool {
        let namespace_regex = regex::Regex::new(r"([\s\w]+:)").unwrap();
        let tag_no_namespace = namespace_regex.replace_all(&self.tag_string, "");

        let criteria_wildcard_support = criteria.replace("*", ".*");
        let criteria_regex =
            regex::Regex::new(&format!(r"^((?:{})(?:/.*)?)$", criteria_wildcard_support)).unwrap();

        criteria_regex.is_match(&tag_no_namespace)
    }
}

impl std::str::FromStr for Tag {
    type Err = crate::TagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tag {
            tag_string: s.to_owned(),
        })
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag_string)
    }
}

impl crate::Media {
    /// Returns tags in an ordered manner, if the file doesn't support tags None is returned
    pub fn tags(&self) -> Option<&Vec<Tag>> {
        match &self.tags {
            Some(tags) => Some(tags),
            None => None,
        }
    }

    /// Returns true if the piece of media can hold tags
    pub fn supports_tags(&self) -> bool {
        self.tags().is_some()
    }

    /// Adds a new tag to the file
    pub fn add_tag(&mut self, new_tag: Tag) -> Result<(), TagError> {
        match self.tags {
            Some(ref mut tags) => {
                tags.push(new_tag);
                tags.dedup();
                self.save()?;
                Ok(())
            }
            None => Err(TagError::TagsNotSupported),
        }
    }

    /// Removes a tag from the file
    pub fn remove_tag(&mut self, tag_to_remove: Tag) -> Result<(), TagError> {
        match self.tags {
            Some(ref mut tags) => {
                if let Some(index) = tags.iter().position(|tag| *tag == tag_to_remove) {
                    tags.remove(index);
                    self.save()?;
                    Ok(())
                } else {
                    Err(TagError::TagMissing)
                }
            }
            None => Err(TagError::TagsNotSupported),
        }
    }

    /// Checks whether the file has the specified tag
    pub fn has_tag(&self, tag_to_search: Tag) -> bool {
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

// TESTS ==================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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
