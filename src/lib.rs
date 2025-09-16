mod error;
pub mod filter;
mod io;
mod media;
mod tag;

pub use crate::error::TagError;
pub use crate::media::Media;
pub use crate::tag::Tag;

/// Read a File at the given path
pub fn read_file(path: &std::path::Path) -> Result<Media, TagError> {
    let new_media = Media::new(path).ok_or(TagError::CouldNotReadFile)?;

    Ok(*new_media)
}
