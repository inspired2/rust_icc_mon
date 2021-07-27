use super::*;
pub use image::{EncodableLayout, ImageFormat, ImageOutputFormat};

pub use img_parts::{Bytes, ImageICC};
use std::fs;
use std::path::*;

pub trait Meta {
    fn metadata(&self) -> Option<rexif::ExifData>;
    fn embedded_profile_bytes(&self) -> Option<img_parts::Bytes>;
    fn is_manageable(&self) -> bool;
    fn iccp(&self) -> Option<Iccp>;
    fn img_format(&self) -> Option<ImageFormat>;
}
impl Meta for Image {
    fn metadata(&self) -> Option<rexif::ExifData> {
        if let Ok(meta) = rexif::parse_buffer(&self.bytes) {
            return Some(meta);
        }
        None
    }

    fn embedded_profile_bytes(&self) -> Option<img_parts::Bytes> {
        self.decoded.icc_profile()
    }
    fn is_manageable(&self) -> bool {
        match self.path.extension() {
            Some(path) => MANAGEABLE_FILE_EXTENSIONS.contains(&path.to_str().unwrap()),
            _ => false,
        }
    }
    fn iccp(&self) -> Option<Iccp> {
        Iccp::new(self)
    }
    fn img_format(&self) -> Option<image::ImageFormat> {
        image::guess_format(self.bytes.as_bytes()).ok()
    }
}

pub fn read_to_buf(path: &PathBuf) -> Result<Vec<u8>, CustomErr> {
    let buffer = fs::read(&**path)?;
    Ok(buffer)
}
// pub fn print_all_exif_tags(meta: &Metadata) {
//     match meta.get_exif_tags() {
//         Ok(tags_vec) => tags_vec.iter().for_each(|tag| println!("tag: {}", tag)),
//         _=>{}

//     }
// }
