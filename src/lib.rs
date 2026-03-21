mod error;
pub mod filter;
mod io;
mod media;
mod metadata;
mod tag;

pub use crate::error::TagError;
pub use crate::io::read_path;
pub use crate::media::Media;
pub use crate::tag::Tag;
