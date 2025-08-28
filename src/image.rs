use crate::{Media, Tag};
use std::{fs::metadata, time::SystemTime};

pub(crate) struct Image {
    path: std::path::PathBuf,
    thumbnail_path: Option<std::path::PathBuf>,
    date: Option<chrono::NaiveDateTime>,
    tags: Option<Vec<Tag>>,
    modified: SystemTime,
}

impl Image {
    pub fn new(path: &std::path::Path) -> Option<Box<Self>> {
        let mut new_image = Image {
            path: path.to_path_buf(),
            thumbnail_path: None,
            date: None,
            tags: None,
            modified: SystemTime::now(),
        };

        match new_image.update() {
            Ok(_) => Some(Box::new(new_image)),
            Err(_) => None,
        }
    }
}

impl Media for Image {
    fn update(&mut self) -> Result<(), crate::TagError> {
        use allmytoes::*;
        use exempi2::{OpenFlags, PropFlags, Xmp, XmpFile, XmpString};

        const THUMBNAIL_SIZE: ThumbSize = ThumbSize::Large;

        const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
        const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";

        if !self.exists() {
            return Err(crate::TagError::FileMissing);
        }

        //println!("reading file://{}", self.path.to_str().unwrap());

        let xmp: Option<Xmp> = XmpFile::new_from_file(&self.path, OpenFlags::ONLY_XMP)
            .expect("failed to read file")
            .get_new_xmp()
            .ok();

        let tags = || -> Option<Vec<Tag>> {
            let mut tags: Vec<Tag> = Vec::new();

            let xmp = xmp.clone()?;
            for i in 1.. {
                match xmp.get_array_item(
                    DUBLIN_CORE_SCHEMA,
                    "dc:subject",
                    i,
                    &mut PropFlags::empty(),
                ) {
                    Ok(tag) => tags.push(Tag::new(tag.to_string())),
                    Err(_) => break,
                }
            }

            Some(tags)
        }();

        let date = || -> Option<chrono::NaiveDateTime> {
            let formats: Vec<&str> = vec!["%+", "%FT%T", "%FT%T%.f"];

            let exif_date: Result<XmpString, exempi2::Error> =
                xmp.clone()?
                    .get_property(EXIF_SCHEMA, "DateTimeOriginal", &mut PropFlags::empty());

            let date = exif_date.ok()?;

            match formats
                .iter()
                .map(|format: &&str| -> Result<chrono::NaiveDateTime, _> {
                    chrono::NaiveDateTime::parse_from_str(date.to_str().unwrap(), *format)
                })
                .filter(|result| -> bool { result.is_ok() })
                .nth(0)
            {
                Some(date) => Some(date.unwrap()),
                None => panic!("unknown date format for date: {}", date.to_str().unwrap()),
            }
        }();

        let thumbnail_path = || -> Option<std::path::PathBuf> {
            let config = AMTConfiguration {
                force_creation: false,
                return_smallest_feasible: false,
                leak: false,
                custom_provider_spec_file: None,
                force_inbuilt_provider_spec: false,
            };

            match AMT::new(&config).get(&self.path, THUMBNAIL_SIZE) {
                Ok(thumbnail) => Some(std::path::PathBuf::from(&thumbnail.path)),
                Err(_) => None,
            }
        }();

        let modified = metadata(&self.path).unwrap().modified().unwrap();

        self.thumbnail_path = thumbnail_path;
        self.date = date;
        self.tags = tags;
        self.modified = modified;

        Ok(())
    }

    fn was_updated_on_disk(&self) -> Result<bool, crate::TagError> {
        if !self.exists() {
            return Err(crate::TagError::FileMissing);
        }

        let modified = metadata(&self.path).unwrap().modified().unwrap();

        Ok(self.modified != modified)
    }

    fn media_type(&self) -> crate::MediaType {
        crate::MediaType::Image
    }

    fn path(&self) -> &std::path::Path {
        &self.path
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn thumbnail_path(&self) -> Option<&std::path::Path> {
        self.thumbnail_path.as_deref()
    }

    fn date(&self) -> Option<chrono::NaiveDateTime> {
        self.date
    }

    fn save(&self) -> Result<(), crate::TagError> {
        todo!()
    }

    fn tags(&self) -> Option<&Vec<Tag>> {
        match &self.tags {
            Some(tags) => Some(&tags),
            None => None,
        }
    }

    fn add_tag(&mut self, new_tag: Tag) -> Result<(), crate::TagError> {
        match self.tags {
            Some(ref mut tags) => {
                tags.push(new_tag);
                self.save()?;
                Ok(())
            }
            None => Err(crate::TagError::TagsNotSupported),
        }
    }

    fn remove_tag(&mut self, tag_to_remove: Tag) -> Result<(), crate::TagError> {
        match self.tags {
            Some(ref mut tags) => {
                if let Some(index) = tags.iter().position(|tag| *tag == tag_to_remove) {
                    tags.remove(index);
                    self.save()?;
                    Ok(())
                } else {
                    Err(crate::TagError::TagMissing)
                }
            }
            None => Err(crate::TagError::TagsNotSupported),
        }
    }
}
