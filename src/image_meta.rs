use super::MANAGEABLE_FILE_EXTENSIONS;
use rexiv2::{self, Metadata};

use std::fs;
use std::path::*;
pub use img_parts::{Bytes, ImageICC};

pub type CustomErr = Box<dyn std::error::Error>;
pub struct Image {
    pub path: Box<PathBuf>,
    pub bytes: img_parts::Bytes
}
impl Image {
    pub fn new(path: PathBuf) -> Result<Self, CustomErr> {
        let buffer = read_to_buf(&path)?;
        let bytes = Bytes::from(buffer);
        let out = Self {
            path: Box::new(path),
            bytes
        };
        Ok(out)
    }
}
pub trait Meta {
    fn metadata(&self) -> Option<rexiv2::Metadata>;
    fn jpeg_embedded_profile_bytes(&self) -> Option<img_parts::Bytes>;
    fn is_manageable(&self) -> bool;
}
impl Meta for Image {
    fn metadata(&self) -> Option<Metadata> {
        if let Ok(meta) = rexiv2::Metadata::new_from_buffer(&self.bytes) {
            return Some(meta)
        }
        None
    }
    fn jpeg_embedded_profile_bytes(&self) -> Option<img_parts::Bytes> {
        img_parts::jpeg::Jpeg::from_bytes(self.bytes.slice(0..self.bytes.len() - 1)).unwrap().icc_profile()
    }
    fn is_manageable(&self) -> bool {
        match self.path.extension() {
            Some(path) => MANAGEABLE_FILE_EXTENSIONS.contains(&path.to_str().unwrap()),
            _=> false
        }
    }
}

fn read_to_buf(path: &PathBuf) -> Result<Vec<u8>, CustomErr> {
    let buffer = fs::read(&**path)?;
    Ok(buffer)
}
pub fn print_all_exif_tags(meta: &Metadata) {
    match meta.get_exif_tags() {
        Ok(tags_vec) => tags_vec.iter().for_each(|tag| println!("tag: {}", tag)),
        _=>{}

    }

}