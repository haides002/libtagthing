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
    /// The XMP in the file could not be read
    CouldNotReadXMP,
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
                TagError::CouldNotReadXMP => "The XMP in the file colud not be read.".to_string(),
                TagError::UwUpsie => "other error".to_string(),
            }
        )
    }
}

impl std::error::Error for TagError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
