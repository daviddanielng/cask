use actix_files::NamedFile;
use mime::Mime;
use serde::Serialize;

use crate::utils::macros::log_error;
use crate::utils::util::file_mime;

#[derive(Debug, Clone, Serialize)]

pub struct File {
    pub path: String,
    #[serde(serialize_with = "serialize_mime")]
    pub content_type: Mime,
    pub size: u64,
}
fn serialize_mime<S>(mime: &Mime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(mime.as_ref())
}
impl File {
    pub fn new(path: String, size: u64) -> Self {
        let mime = file_mime(path.as_str());
        Self {
            path,
            content_type: mime,
            size,
        }
    }
    pub fn read(&self) -> Option<NamedFile> {
        let open = NamedFile::open(&self.path);
        match open {
            Ok(file) => Some(file.set_content_type(self.content_type.clone())),
            Err(e) => {
                log_error!("Failed to open file {}: {:?}", self.path, e);
                None
            }
        }
    }
    pub fn is_equal(&self, file: &File) -> bool {
        if self.path != file.path {
            return  false;
        }
        if self.content_type != file.content_type {
            return false;
        }
        if self.size != file.size {
            return false;
        }
        true
        
    }
    pub fn name(&self)->String{
        self.path.split("/").last().unwrap().to_string()
    }
}
