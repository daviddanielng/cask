use actix_files::NamedFile;
use mime::Mime;
use serde::Serialize;

use crate::utils::macros::log_error;

#[derive(Debug, Clone, Serialize)]

pub struct File {
    pub path: String,
    #[serde(serialize_with = "serialize_mime")]
    pub content_type: Mime,
    
}
fn serialize_mime<S>(mime: &Mime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(mime.as_ref())
}
impl File {
    pub fn new(path: String, content_type: Mime) -> Self {
        Self { path, content_type }
    }
    pub fn read(&self) -> Option<NamedFile> {
        let open = NamedFile::open(&self.path);
        match open {
            Ok(file) => Some(file.set_content_type(self.content_type.clone())),
            Err(e) => {
                log_error!("Failed to open file {}: {:?}", self.path,e);
                None
            }
        }
    }
}
