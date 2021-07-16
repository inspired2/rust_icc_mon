use super::MANAGEABLE_FILE_EXTENSIONS;
use image::DynamicImage;
use image::EncodableLayout;
use image::ImageOutputFormat;
use img_parts::DynImage;
use img_parts::jpeg::Jpeg;
use lcms2::Profile;
use qcms;
use rexiv2::{self, Metadata};
use webp;

use std::fs;
use std::path::*;
pub use img_parts::{Bytes, ImageICC};

enum ImageType {
    Jpeg,
    Webp,
    Tiff,
    Heic,
    Bmp,
    Other,
}
pub type CustomErr = Box<dyn std::error::Error>;
pub struct Image {
    pub decoded: img_parts::DynImage,
    pub path: Box<PathBuf>,
    //now storing raw undecoded buffer in "bytes" field. 
    //maybe should store enum with concrete image type 
    //& decoded data inside appropriate enum variant?
    pub bytes: img_parts::Bytes,
    //pub decoded: Jpeg
}
unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    pub fn new(path: PathBuf) -> Result<Self, CustomErr> {
        let buffer = read_to_buf(&path).unwrap();
        let bytes = Bytes::from(buffer);
        if let Some(decoded) = DynImage::from_bytes(bytes.to_owned()).unwrap() {
        //let decoded = decode_jpeg(bytes.to_owned()).unwrap();
        let out = Self {
            path: Box::new(path),
            bytes,
            decoded
        };
        Ok(out) } else {return Err(CustomErr::from(std::io::Error::new(std::io::ErrorKind::Other, "not an image")))}
    }
    pub fn save(self) -> Result<(), CustomErr> {
        let file = std::fs::File::create(*self.path)?;
        DynImage::encoder(self.decoded).write_to(file)?;
        Ok(())
    }
    pub fn convert_webp_to_jpeg(mut self) -> Result<Image, CustomErr> {
        let dyn_img = 
            webp::Decoder::new(self.bytes.as_bytes())
                .decode().unwrap()
                .to_image();
                
                //.to_vec();
        let mut path = self.path.to_owned();
        path.set_extension("jpeg");
        //std::fs::remove_file(&*path)?;
        let mut bytes = Vec::new();
        println!("dyn image: {:?}, path: {:?}", self.decoded, self.path.file_name().unwrap());
        dyn_img.write_to(&mut bytes,ImageOutputFormat::Jpeg(100))?;
        self = Image {
            decoded: DynImage::Jpeg(Jpeg::from_bytes(Bytes::from(bytes.to_owned()))?),
            path,
            bytes: Bytes::from(bytes)
        };
        Ok(self)
//         self.path.set_extension("jpeg");
//         self.bytes = Bytes::from(buffer);
//         self.decoded = DynImage::Jpeg(Jpeg::from_bytes(self.bytes.clone())?);
//         Ok(self)
    }
}
pub trait Meta {
    fn metadata(&self) -> Option<rexiv2::Metadata>;
    fn embedded_profile_bytes(&self) -> Option<img_parts::Bytes>;
    fn is_manageable(&self) -> bool;
    fn iccp(&self) -> Option<lcms2::Profile>;
    fn img_type(&self) -> ImageType;
    //fn decoded(&self) -> Option<Jpeg>;
}
impl Meta for Image {
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
    fn img_type(&self) -> ImageType {
        match image::guess_format(self.bytes.as_bytes()) {
            Ok(image::ImageFormat::Jpeg) => ImageType::Jpeg,
            Ok(image::ImageFormat::WebP) => ImageType::Webp,
            Ok(image::ImageFormat::Bmp) => ImageType::Bmp,
            Ok(image::ImageFormat::Tiff) => ImageType::Tiff,
            _ => ImageType::Other
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
