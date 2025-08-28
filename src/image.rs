use crate::{Media, Tag};

struct Image {
    path: std::path::PathBuf,
    thumbnail_path: Option<std::path::PathBuf>,
    date: Option<chrono::NaiveDateTime>,
    tags: Option<Vec<Tag>>,
}

impl Media for Image {
    fn new(path: &std::path::Path) -> Option<Box<Self>> {
        use allmytoes::*;
        use exempi2::{OpenFlags, PropFlags, Xmp, XmpFile, XmpString};

        const THUMBNAIL_SIZE: ThumbSize = ThumbSize::Large;

        const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
        const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";

        println!("reading file://{}", path.to_str().unwrap());

        let xmp: Option<Xmp> = XmpFile::new_from_file(&path, OpenFlags::ONLY_XMP)
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

            match AMT::new(&config).get(&path, THUMBNAIL_SIZE) {
                Ok(thumbnail) => Some(std::path::PathBuf::from(&thumbnail.path)),
                Err(_) => None,
            }
        }();

        Some(Box::new(Self {
            path: path.to_path_buf(),
            thumbnail_path,
            date,
            tags,
        }))
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
