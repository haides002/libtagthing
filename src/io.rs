use crate::{Media, Tag, TagError};
use xmp_toolkit::xmp_ns;

impl Media {
    /// Updates the file to the state on disk
    pub(crate) fn update(&mut self) -> Result<(), TagError> {
        use xmp_toolkit::{XmpMeta, XmpValue};

        if !self.exists() {
            return Err(TagError::FileMissing);
        }

        let xmp: Option<XmpMeta> = XmpMeta::from_file(self.path()).ok();

        let tags = || -> Option<Vec<Tag>> {
            Some(
                xmp.clone()?
                    .property_array(xmp_ns::DC, "dc:subject")
                    .map(|value: XmpValue<String>| -> Tag { Tag::new(value.value) })
                    .collect::<Vec<Tag>>(),
            )
        }();

        let date = || -> Option<chrono::DateTime<chrono::FixedOffset>> {
            use chrono::{DateTime, FixedOffset};
            use xmp_toolkit::XmpDateTime;

            let xmp_date: XmpDateTime = xmp
                .clone()?
                .property_date(xmp_ns::EXIF, "DateTimeOriginal")?
                .value;

            let chrono_date: DateTime<FixedOffset> = xmp_date.try_into().ok()?;

            Some(chrono_date)
        }();

        let modified = std::fs::metadata(&self.path).unwrap().modified().unwrap();

        self.date = date;
        self.tags = tags;
        self.modified = modified;

        Ok(())
    }

    /// Check wheather the file was updated on disk
    pub fn was_updated_on_disk(&self) -> Result<bool, TagError> {
        if !self.exists() {
            return Err(TagError::FileMissing);
        }

        let modified = std::fs::metadata(&self.path).unwrap().modified().unwrap();

        Ok(self.modified != modified)
    }

    /// Check whether the file still exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Filesize in Bytes
    pub fn size(&self) -> Result<u64, std::io::Error> {
        Ok(std::fs::metadata(self.path())?.len())
    }

    /// Saves modifications to the struct instance to the associated file
    pub(crate) fn save(&self) -> Result<(), TagError> {
        use xmp_toolkit::{XmpFile, XmpValue};

        if !self.supports_tags() {
            return Err(TagError::TagsNotSupported);
        }

        // create xmp file container
        let mut xmp_file: XmpFile = match XmpFile::new() {
            Ok(file) => file,
            Err(_) => return Err(TagError::UwUpsie),
        };

        // read data into container
        match xmp_file.open_file(
            self.path().to_str().unwrap(),
            xmp_toolkit::OpenFileOptions::default().for_update(),
        ) {
            Ok(_) => {}
            Err(_) => return Err(TagError::CouldNotReadFile),
        };

        // read xmp from container
        let mut xmp: xmp_toolkit::XmpMeta = match xmp_file.xmp() {
            Some(xmp) => xmp,
            None => return Err(TagError::CouldNotReadXMP),
        };

        if xmp.contains_property(xmp_ns::DC, "dc:subject") {
            // clear subject array
            xmp.delete_property(xmp_ns::DC, "dc:subject").unwrap();
        }

        for tag in self.tags().unwrap() {
            xmp.append_array_item(
                xmp_ns::DC,
                &XmpValue::new("dc:subject".to_string()).set_is_array(true),
                &XmpValue::new(tag.to_string()),
            )
            .unwrap();
        }

        match xmp_file.put_xmp(&xmp) {
            Ok(_) => {
                xmp_file.close();
                Ok(())
            }
            Err(_) => Err(TagError::OtherSaveError),
        }
    }
}

/// Reads a given path into a `Vec<Media>`. The path can be a directory or a file. Setting recursive
/// to true will also read out subdirectories.
pub fn read_path(path: std::path::PathBuf, recursive: bool) -> Vec<crate::Media> {
    /// Read a File at the given path
    fn read_file(path: &std::path::Path) -> Result<Media, TagError> {
        let new_media = Media::new(path).ok_or(TagError::CouldNotReadFile)?;

        Ok(new_media)
    }

    let mut media_collection: Vec<crate::Media> = Vec::new();
    let mut todo_paths: Vec<std::path::PathBuf> = vec![path];
    let mut current_path: std::path::PathBuf;

    while !todo_paths.is_empty() {
        current_path = todo_paths.pop().expect("Non-empty vec couldn't pop");

        assert!(
            current_path.is_dir() ^ current_path.is_file(),
            "What the FUCK!"
        );

        if current_path.is_dir() {
            todo_paths.append(
                &mut current_path
                    .read_dir()
                    .expect("Directory couldn't get read")
                    .map(|p| p.unwrap().path())
                    // Per de Moorgans law, !(p.is_dir() && !recursive) could also be written as !p.is_dir() || recursive, idiot.
                    .filter(|p| !(p.is_dir() && !recursive))
                    .collect(),
            )
        } else if current_path.is_file()
            && let Ok(new_image) = read_file(current_path.as_path())
        {
            media_collection.push(new_image)
        }
    }

    media_collection
}
