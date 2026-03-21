use crate::TagError;

macro_rules! set_field {
    ($field:expr, $value:expr) => {
        if !($value.is_none() || $value.clone().unwrap().is_empty()) {
            $field = $value;
        }
    };
}
impl crate::Media {
    pub fn set_title(&mut self, new_title: Option<String>) -> Result<(), TagError> {
        set_field!(self.title, new_title);
        self.save()
    }
    pub fn set_description(&mut self, new_description: Option<String>) -> Result<(), TagError> {
        set_field!(self.description, new_description);
        self.save()
    }
    pub fn set_source(&mut self, new_source: Option<String>) -> Result<(), TagError> {
        set_field!(self.source, new_source);
        self.save()
    }
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
    pub fn source(&self) -> Option<String> {
        self.source.clone()
    }
}
