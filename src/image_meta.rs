use super::MANAGEABLE_FILE_EXTENSIONS;
use img_parts::DynImage;
use lcms2::Profile;
use qcms;
use rexiv2::{self, Metadata};

use std::fs;
use std::path::*;
pub use img_parts::{Bytes, ImageICC};

pub type CustomErr = Box<dyn std::error::Error>;
pub struct JpegImage {
    pub decoded: img_parts::DynImage,
    pub path: Box<PathBuf>,
    //now storing raw undecoded buffer in "bytes" field. 
    //maybe should store enum with concrete image type 
    //& decoded data inside appropriate enum variant?
    pub bytes: img_parts::Bytes,
    //pub decoded: Jpeg
}
impl JpegImage {
    pub fn new(path: PathBuf) -> Result<Self, CustomErr> {
        let buffer = read_to_buf(&path).unwrap();
        let bytes = Bytes::from(buffer);
        let decoded = DynImage::from_bytes(bytes.to_owned()).unwrap().unwrap();
        let out = Self {
            path: Box::new(path),
            bytes,
            decoded
        };
        Ok(out)
    }
    pub fn save(self) -> Result<(), CustomErr> {
        let file = std::fs::File::create(*self.path)?;
        DynImage::encoder(self.decoded).write_to(file)?;
        Ok(())
    }
}
pub trait Meta {
    fn metadata(&self) -> Option<rexiv2::Metadata>;
    fn embedded_profile_bytes(&self) -> Option<img_parts::Bytes>;
    fn is_manageable(&self) -> bool;
    fn iccp(&self) -> Option<lcms2::Profile>;
    //fn decoded(&self) -> Option<Jpeg>;
}
impl Meta for JpegImage {
    fn metadata(&self) -> Option<Metadata> {
        if let Ok(meta) = rexiv2::Metadata::new_from_buffer(&self.bytes) {
            return Some(meta)
        }
        None
    }
    
    fn embedded_profile_bytes(&self) -> Option<img_parts::Bytes> {
        self.decoded.icc_profile()
    }
    fn is_manageable(&self) -> bool {
        match self.path.extension() {
            Some(path) => {
                MANAGEABLE_FILE_EXTENSIONS.contains(&path.to_str().unwrap())
            },
            _=> false
        }
    }
    fn iccp(&self) -> Option<lcms2::Profile> {
        let profile_bytes = self.embedded_profile_bytes();
        match profile_bytes {
            Some(bytes) => Profile::new_icc(&bytes[..]).ok(),
            _=> None
        }
    }
    // fn decoded(&self) -> Option<Jpeg> {
    //     decode_jpeg(self.bytes.to_owned()).ok()
    // }
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
// fn decode_jpeg(buff: Bytes) -> Result<Jpeg, CustomErr> {
//     let dyn_image = DynImage::from_bytes(buff).unwrap();
//     println!("dynImage: {:?}", dyn_image);
//     match dyn_image {
//         Some(DynImage::Jpeg(decoded)) => Ok(decoded),
//         _=> {Err (CustomErr::from(std::io::Error::new(std::io::ErrorKind::Other, "cud not decode jpeg")))}
//     }
// }